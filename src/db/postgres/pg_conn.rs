use std::ops::DerefMut;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::{
    activitystream_objects::actors::Actor,
    db::{conn::Conn, Follower},
};

use super::{actors, follows, instance_actor, local_users, posts};

pub struct PgConn {
    pub db: Pool,
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[allow(unused_variables)]
#[async_trait]
impl Conn for PgConn {
    async fn get_actor(&self, uid: i64, instance_domain: &str) -> Option<Actor> {
        actors::get_actor(self, uid, instance_domain).await
    }
    async fn is_local(&self, uid: i64) -> bool {
        todo!()
    }
    async fn create_federated_actor(&self, actor: &Actor) -> i64 {
        actors::create_federated_actor(self, actor).await
    }

    async fn get_federated_db_id(&self, actor_id: &str) -> Option<i64> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM unified_users JOIN federated_ap_users fedi_id = fedi_id WHERE id = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        client
            .query(&stmt, &[&actor_id])
            .await
            .expect("failed to get local user")
            .pop()
            .map(|x| x.get("uid"))
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
        local_users::create_local_user(self, user).await
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
        SELECT * FROM unified_users JOIN internal_users local_id = local_id WHERE uid = $1;
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
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM unified_users JOIN internal_users local_id = local_id WHERE preferred_username = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&preferred_username])
            .await
            .expect("failed to get local user")
            .pop();
        result.map(|x| x.get("uid"))
    }

    async fn get_local_user_actor(
        &self,
        preferred_username: &str,
        instance_domain: &str,
    ) -> Option<(Actor, i64)> {
        actors::get_local_user_actor(self, preferred_username, instance_domain).await
    }

    // async fn get_local_user_private_key(&self, preferred_username: &str) -> String {
    //     let client = self.db.get().await.expect("failed to get client");
    //     let stmt = r#"
    //     SELECT * FROM internal_users WHERE preferred_username = $1;
    //     "#;
    //     let stmt = client.prepare(stmt).await.unwrap();

    //     let result = client
    //         .query(&stmt, &[&preferred_username])
    //         .await
    //         .expect("failed to get local user")
    //         .pop();
    //     let result = result.expect("could not get private key");

    //     let private_key_pem: String = result.get("private_key_pem");
    //     private_key_pem
    // }

    async fn get_local_user_private_key_db_id(&self, uid: i64) -> String {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM unified_users JOIN internal_users local_id = local_id WHERE uid = $1;
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
        uid: i64,
        is_local: bool,
        in_reply_to: Option<i64>,
    ) -> i64 {
        posts::create_new_post(self, post, instance_domain, uid, is_local, in_reply_to).await
    }

    async fn create_follow_request(&self, from: i64, to: i64, pending: bool) -> Result<(), ()> {
        follows::create_follow_request(self, from, to, pending).await
    }

    async fn approve_follow_request(&self, from: i64, to: i64) -> Result<(), ()> {
        follows::approve_follow_request(self, from, to).await
    }

    async fn get_followers(&self, user: i64) -> Result<Vec<Follower>, ()> {
        follows::get_followers(self, user).await
    }

    async fn get_follower_count(&self, user: i64) -> Result<i64, ()> {
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
