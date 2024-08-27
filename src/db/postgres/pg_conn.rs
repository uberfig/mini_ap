use std::ops::DerefMut;

use async_trait::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::{
    activitystream_objects::actors::{Actor, ActorType, PublicKey},
    db::{conn::Conn, generate_links, UserRef},
};

use super::{follows, instance_actor, posts};

pub struct PgConn {
    pub db: Pool,
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

fn local_user_from_row(result: Row, instance_domain: &str) -> Actor {
    let preferred_username: String = result.get("preferred_username");
    let links = generate_links(instance_domain, &preferred_username);

    let key = PublicKey {
        id: links.pub_key_id,
        owner: links.id.clone(),
        public_key_pem: result.get("public_key_pem"),
    };

    Actor {
        type_field: ActorType::Person,
        id: links.id,
        preferred_username,
        summary: result.get("summary"),
        name: result.get("display_name"),
        url: Some(links.url),
        public_key: key,
        inbox: links.inbox,
        outbox: links.outbox,
        followers: links.followers,
        following: links.following,
        domain: Some(instance_domain.to_string()),
        liked: Some(links.liked),
    }
}

#[allow(unused_variables)]
#[async_trait]
impl Conn for PgConn {
    async fn create_federated_user(&self, actor: &Actor) -> i64 {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        INSERT INTO federated_ap_users 
        (
            id, type_field, preferred_username, domain,
            name, summary, url, public_key_pem,
            inbox, outbox, followers, following
            manual_followers, memorial, indexable, discoverable
        )
        VALUES
        (
            $1, $2, $3, $4, 
            $5, $6, $7, $8,
            $9, $10, $11, $12,
            $13, $14, $15, $16
        )
        RETURNING ap_user_id;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let domain = actor.id.domain().unwrap();
        let url = actor.url.as_ref().map(|url| url.as_str());
        let result: i64 = client
            .query(
                &stmt,
                &[
                    &actor.id.as_str(),
                    &serde_json::to_string(&actor.type_field).unwrap(),
                    &actor.preferred_username,
                    &domain,
                    &actor.name,
                    &actor.summary,
                    &url,
                    &actor.public_key.public_key_pem,
                    &actor.inbox.as_str(),
                    &actor.outbox.as_str(),
                    &actor.followers.as_str(),
                    &actor.following.as_str(),
                ],
            )
            .await
            .expect("failed to insert user")
            .pop()
            .expect("did not return uid")
            .get("uid");

        result
    }

    async fn get_federated_user_db_id(&self, actor_id: &str) -> Option<i64> {
        todo!()
    }

    async fn get_federated_actor(
        &self,
        actor_id: &str,
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        todo!()
    }

    async fn get_federated_actor_db_id(
        &self,
        id: i64,
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        todo!()
    }

    async fn create_local_user(&self, user: &crate::db::NewLocal) -> Result<i64, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        INSERT INTO internal_users 
        (
            password, preferred_username, email, private_key_pem,
            public_key_pem, permission_level, custom_domain
        )
        VALUES
        (
            $1, $2, $3, $4,
            $5, $6, $7
        )
        RETURNING uid;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let permission: i16 = user.permission_level.into();

        let result: i64 = client
            .query(
                &stmt,
                &[
                    &user.password,
                    &user.username,
                    &user.email,
                    &user.private_key_pem,
                    &user.public_key_pem,
                    &permission,
                    &user.custom_domain,
                ],
            )
            .await
            .expect("failed to insert user")
            .pop()
            .expect("did not return uid")
            .get("uid");

        Ok(result)
    }

    async fn set_permission_level(&self, uid: i64, permission_level: crate::db::PermissionLevel) {
        todo!()
    }

    async fn update_password(&self, uid: i64, password: &str) {
        todo!()
    }

    async fn set_manually_approves_followers(&self, uid: i64, value: bool) {
        todo!()
    }

    async fn get_local_manually_approves_followers(&self, uid: i64) -> bool {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM internal_users WHERE uid = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&uid])
            .await
            .expect("failed to get local user")
            .pop()
            .unwrap();
        result.get("manual_followers")
    }

    async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64> {
        todo!()
    }

    async fn get_local_user_actor(
        &self,
        preferred_username: &str,
        instance_domain: &str,
    ) -> Option<(Actor, i64)> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM internal_users WHERE preferred_username = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&preferred_username])
            .await
            .expect("failed to get local user")
            .pop();

        let result = match result {
            Some(x) => x,
            None => return None,
        };
        let id: i64 = result.get("uid");

        Some((local_user_from_row(result, instance_domain), id))
    }

    async fn get_local_user_actor_db_id(
        &self,
        uid: i64,
        instance_domain: &str,
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM internal_users WHERE uid = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&uid])
            .await
            .expect("failed to get local user")
            .pop();

        let result = match result {
            Some(x) => x,
            None => return None,
        };

        Some(local_user_from_row(result, instance_domain))
    }

    async fn get_local_user_private_key(&self, preferred_username: &str) -> String {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM internal_users WHERE preferred_username = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&preferred_username])
            .await
            .expect("failed to get local user")
            .pop();
        let result = result.expect("could not get private key");

        let private_key_pem: String = result.get("private_key_pem");
        private_key_pem
    }

    async fn get_local_user_private_key_db_id(&self, uid: i64) -> String {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM internal_users WHERE uid = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&uid])
            .await
            .expect("failed to get local user")
            .pop();
        let result = result.expect("could not get private key");

        let private_key_pem: String = result.get("private_key_pem");
        private_key_pem
    }

    async fn create_new_post(
        &self,
        post: &crate::db::PostType,
        instance_domain: &str,
        uid: UserRef,
        in_reply_to: Option<i64>,
    ) -> i64 {
        posts::create_new_post(self, post, instance_domain, uid, in_reply_to).await
    }

    async fn create_follow_request(
        &self,
        from: UserRef,
        to: UserRef,
        pending: bool,
    ) -> Result<(), ()> {
        follows::create_follow_request(self, from, to, pending).await
    }

    async fn approve_follow_request(&self, from: UserRef, to: UserRef) -> Result<(), ()> {
        follows::approve_follow_request(self, from, to).await
    }

    async fn get_followers(&self, user: UserRef) -> Result<Vec<UserRef>, ()> {
        follows::get_followers(self, user).await
    }

    async fn get_follower_count(&self, user: UserRef) -> Result<i64, ()> {
        follows::get_follower_count(self, user).await
    }
    async fn get_post(&self, object_id: i64) -> Option<crate::db::PostType> {
        posts::get_post(self, object_id).await
    }
    async fn get_instance_actor(&self) -> Option<crate::db::InstanceActor> {
        instance_actor::get_instance_actor(self).await
    }
    async fn create_instance_actor(&self, private_key_pem: String, public_key_pem: String) {
        instance_actor::create_instance_actor(self, private_key_pem, public_key_pem).await
    }
    async fn init(&self) -> Result<(), String> {
        let mut conn = self
            .db
            .get()
            .await
            .expect("could not get conn for migrations");
        let client = conn.deref_mut().deref_mut();
        let report = embedded::migrations::runner().run_async(client).await;
        match report {
            Ok(x) => {
                println!("migrations sucessful");
                if x.applied_migrations().is_empty() {
                    println!("no migrations applied")
                } else {
                    println!("applied migrations: ");
                    for migration in x.applied_migrations() {
                        match migration.applied_on() {
                            Some(x) => println!(" - {} applied {}", migration.name(), x),
                            None => println!(" - {} applied N/A", migration.name()),
                        }
                    }
                }
            }
            Err(x) => {
                return Err(x.to_string());
            }
        }
        Ok(())
    }
}
