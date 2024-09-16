use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::{
    activitystream_objects::actors::Actor,
    db::{
        conn::{Conn, DbErr},
        utility::{instance_actor::InstanceActor, new_actor::NewLocal},
        Follower, Like,
    },
};

use super::{acct_mgmt, actors, follows, init, instance_actor, local_users, posts};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

#[allow(unused_variables)]
#[async_trait]
impl Conn for PgConn {
    async fn init(&self) -> Result<(), String> {
        init::init(self).await
    }

    //-------------------instance actor------------------------------
    async fn get_instance_actor(&self) -> Option<InstanceActor> {
        instance_actor::get_instance_actor(self).await
    }
    async fn create_instance_actor(&self, private_key_pem: &str, public_key_pem: &str) {
        instance_actor::create_instance_actor(self, private_key_pem, public_key_pem).await
    }

    //----------------------actors---------------------------

    async fn get_actor(&self, uid: i64, instance_domain: &str) -> Option<Actor> {
        actors::get_actor(self, uid, instance_domain).await
    }
    async fn get_local_user_actor(
        &self,
        preferred_username: &str,
        instance_domain: &str,
    ) -> Option<(Actor, i64)> {
        actors::get_local_user_actor(self, preferred_username, instance_domain).await
    }
    async fn is_local(&self, uid: i64) -> bool {
        todo!()
    }

    async fn get_federated_db_id(&self, actor_id: &str) -> Option<i64> {
        actors::get_federated_db_id(self, actor_id).await
    }
    async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64> {
        actors::get_local_user_db_id(self, preferred_username).await
    }

    async fn get_federated_actor(
        &self,
        actor_id: &str,
    ) -> Option<crate::activitystream_objects::actors::Actor> {
        todo!()
    }

    //-----------------------account managment-----------------------------

    async fn update_password(&self, uid: i64, password: &str) {
        todo!()
    }
    async fn set_manually_approves_followers(&self, uid: i64, value: bool) {
        todo!()
    }
    async fn get_local_manually_approves_followers(&self, uid: i64) -> bool {
        acct_mgmt::get_local_manually_approves_followers(self, uid).await
    }
    async fn set_permission_level(&self, uid: i64, permission_level: crate::db::PermissionLevel) {
        todo!()
    }

    //------------------------------posts---------------------------------

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
    async fn delete_post(&self, uid: i64) -> Result<(), ()> {
        todo!()
    }
    async fn get_post(&self, object_id: i64) -> Option<crate::db::PostType> {
        posts::get_post(self, object_id).await
    }

    //------------------------------likes-----------------------------------

    async fn create_like(&self, uid: i64, obj_id: i64) -> Result<(), ()> {
        todo!()
    }
    async fn remove_like(&self, uid: i64, obj_id: i64) -> Result<(), ()> {
        todo!()
    }
    async fn get_post_likes(&self, obj_id: i64) -> Result<Vec<Like>, ()> {
        todo!()
    }
    async fn get_user_likes(&self, uid: i64) -> Result<Vec<Like>, ()> {
        todo!()
    }

    //----------------------managing actors-------------------------------

    ///used for deleting both federated and local accounts
    async fn delete_actor(&self, uid: i64, reason: Option<&str>) -> Result<(), ()> {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");

        let stmt = r#"
        SELECT * FROM unified_users WHERE uid = $1;
        "#;
        let stmt = transaction.prepare(stmt).await.unwrap();

        let result = transaction
            .query(&stmt, &[&uid])
            .await
            .expect("failed to get actor")
            .pop();

        let Some(result) = result else {
            return Err(());
        };

        let is_local: bool = result.get("is_local");

        match is_local {
            true => {
                let stmt = r#"
                    DELETE FROM internal_users WHERE uid = $1;
                "#;
                let stmt = transaction.prepare(stmt).await.unwrap();

                let result = transaction
                    .query(&stmt, &[&uid])
                    .await
                    .expect("failed to delete local");
            }
            false => {
                let stmt = r#"
                    DELETE FROM federated_ap_users WHERE uid = $1;
                "#;
                let stmt = transaction.prepare(stmt).await.unwrap();

                let result = transaction
                    .query(&stmt, &[&uid])
                    .await
                    .expect("failed to delete fedi");
            }
        };

        transaction.commit().await.expect("failed to commit");

        Ok(())
    }

    async fn create_local_user(&self, user: &NewLocal) -> Result<i64, DbErr> {
        local_users::create_local_user(self, user).await
    }
    async fn create_federated_actor(&self, actor: &Actor) -> i64 {
        actors::create_federated_actor(self, actor).await
    }

    async fn get_local_user_private_key_db_id(&self, uid: i64) -> String {
        let client = self.db.get().await.expect("failed to get client");
        let stmt = r#"
        SELECT * FROM unified_users NATURAL JOIN internal_users WHERE uid = $1;
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

    //-------------------------followers---------------------------------

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

    async fn get_follow(&self, from_id: i64, to_id: i64) -> Option<Follower> {
        follows::get_follow(self, from_id, to_id).await
    }
}
