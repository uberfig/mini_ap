use async_trait::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use url::Url;

use crate::{
    activitystream_objects::{
        actors::{Actor, ActorType, PublicKey},
        object::{Object, ObjectType},
    },
    db::{conn::Conn, generate_links, get_post_id_and_published, InstanceActor, PostSupertype},
};

pub struct PgConn {
    pub db: Pool,
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

    async fn create_new_post(
        &self,
        post: &crate::db::PostType,
        instance_domain: &str,
        is_local: bool,
        uid: i64,
        in_reply_to: Option<i64>,
    ) -> i64 {
        let (post_id, published) = get_post_id_and_published(is_local, &post);
        let (fedi_actor, local_actor) = match is_local {
            true => (None, Some(uid)),
            false => (Some(uid), None),
        };
        match &post {
            crate::db::PostType::Object(x) => {
                let client = self.db.get().await.expect("failed to get client");

                let stmt = r#"
INSERT INTO posts 
(
    is_local, fedi_id, surtype, subtype,
    local_only, published, in_reply_to,
    content, domain,
    fedi_actor, local_actor
)
VALUES
(
    $1, $2, $3, $4,
    $5, $6, $7,
    $8, $9,
    $10, $11
)
RETURNING obj_id;
        "#;
                let stmt = client.prepare(stmt).await.unwrap();

                let result: i64 = client
                    .query(
                        &stmt,
                        &[
                            &is_local,
                            &post_id,
                            &post.get_surtype(),
                            &post.get_subtype(),
                            &false,
                            &published,
                            &in_reply_to,
                            &x.object.content,
                            &instance_domain,
                            &fedi_actor,
                            &local_actor,
                        ],
                    )
                    .await
                    .expect("failed to insert post")
                    .pop()
                    .expect("did not return obj_id")
                    .get("obj_id");
                result
            }
            crate::db::PostType::Question(x) => todo!(),
        }
    }

    async fn create_follow_request(&self, from_id: &str, to_id: &str) -> Result<(), ()> {
        todo!()
    }

    async fn approve_follow_request(&self, from_id: &str, to_id: &str) -> Result<(), ()> {
        todo!()
    }

    async fn get_followers(&self, preferred_username: &str) -> Result<(), ()> {
        todo!()
    }

    async fn get_follower_count(&self, preferred_username: &str) -> Result<(), ()> {
        todo!()
    }
    async fn get_post(&self, object_id: i64) -> Option<crate::db::PostType> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM posts JOIN internal_users ON local_actor = uid WHERE obj_id = $1;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&object_id])
            .await
            .expect("failed to get post")
            .pop();
        let Some(result) = result else {
            return None;
        };

        let supertype: String = result.get("surtype");
        let supertype = PostSupertype::from_str(&supertype).expect("unkown supertype in posts");
        match supertype {
            PostSupertype::Object => {
                let is_local: bool = result.get("is_local");
                let (id, attributed_to) = match is_local {
                    true => {
                        let preferred_username: String = result.get("preferred_username");
                        let domain: String = result.get("domain");
                        (
                            format!(
                                "https://{}/users/{}/statuses/{}",
                                &domain, &preferred_username, object_id
                            ),
                            format!("https://{}/users/{}", &domain, &preferred_username),
                        )
                    }
                    false => {
                        // (result.get("posts.fedi_id"), )
                        todo!()
                    }
                };
                let subtype: String = result.get("subtype");
                let subtype: ObjectType =
                    serde_json::from_str(&subtype).expect("unkown object type stored in db");
                dbg!(&attributed_to);
                let attributed_to = Url::parse(&attributed_to).expect("invalid attributed to");

                let replied_obj: Option<i64> = result.get("in_reply_to");
                let replied_obj: Option<String> = match replied_obj {
                    Some(x) => {
                        todo!()
                    }
                    None => None,
                };

                let object = Object::new(Url::parse(&id).unwrap(), attributed_to)
                    .content(result.get("content"))
                    .in_reply_to(replied_obj)
                    .published_milis(result.get("published"))
                    .to_public()
                    .wrap(subtype);

                Some(crate::db::PostType::Object(object))
            }
            PostSupertype::Question => todo!(),
        }
    }
    async fn get_instance_actor(&self) -> Option<crate::db::InstanceActor> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM ap_instance_actor;
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[])
            .await
            .expect("failed to get instance actor")
            .pop();
        match result {
            Some(result) => Some(InstanceActor {
                private_key_pem: result.get("private_key_pem"),
                public_key_pem: result.get("public_key_pem"),
            }),
            None => None,
        }
    }
    async fn create_instance_actor(&self, private_key_pem: String, public_key_pem: String) {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        INSERT INTO ap_instance_actor 
        (private_key_pem, public_key_pem)
        VALUES
        ($1, $2);
        "#;
        let stmt = client.prepare(stmt).await.unwrap();

        let result = client
            .query(&stmt, &[&private_key_pem, &public_key_pem])
            .await;
        result.unwrap();
    }
}
