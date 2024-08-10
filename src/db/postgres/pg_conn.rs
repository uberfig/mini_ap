use deadpool_postgres::Pool;

use crate::{activitystream_objects::actors::Actor, db::Conn};

pub struct PgConn {
    pub db: Pool,
}

impl Conn for PgConn {
    async fn create_federated_user(
        &self,
        actor: &Actor,
    ) -> i64 {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        INSERT INTO federated_ap_users 
        VALUES (
            id, type_field, preferred_username, domain,
            name, summary, url, public_key_pem,
            inbox, outbox, followers, following
            manual_followers, memorial, indexable, discoverable
        )
        RETURNING ap_user_id;
        "#;
        let stmt = client.prepare(&stmt).await.unwrap();

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
                    &actor.inbox,
                    &actor.outbox,
                    &actor.followers,
                    &actor.following,
                ],
            )
            .await
            .expect("failed to insert user")
            .pop()
            .expect("did not return uid")
            .get("uid");

        todo!()
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

    async fn create_local_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
        permission_level: crate::db::PermissionLevel,
        private_key_pem: &str,
        public_key_pem: &str,
        custom_domain: Option<&str>,
    ) -> Result<i64, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        INSERT INTO internal_users 
        VALUES (
            password, preferred_username, email, private_key_pem,
            public_key_pem, permission_level, custom_domain
        )
        RETURNING uid;
        "#;
        let stmt = client.prepare(&stmt).await.unwrap();

        let result: i64 = client
            .query(
                &stmt,
                &[
                    &password,
                    &username,
                    &email,
                    &private_key_pem,
                    &public_key_pem,
                    &custom_domain,
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
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        todo!()
    }

    async fn get_local_user_actor_db_id(
        &self,
        uid: i64,
        instance_domain: &str,
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        todo!()
    }

    async fn get_local_user_private_key(&self, preferred_username: &str) -> String {
        todo!()
    }

    async fn create_new_post(&self, post: crate::db::PostType) -> i64 {
        todo!()
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
}
