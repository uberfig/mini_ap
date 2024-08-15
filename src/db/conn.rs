use async_trait::async_trait;

use crate::activitystream_objects::actors::Actor;

use super::{InstanceActor, NewLocal, PermissionLevel, PostType};

#[async_trait]
pub trait Conn {
    async fn create_federated_user(&self, actor: &Actor) -> i64;
    async fn get_federated_user_db_id(&self, actor_id: &str) -> Option<i64>;
    async fn get_federated_actor(&self, actor_id: &str) -> Option<Actor>;
    async fn get_federated_actor_db_id(&self, id: i64) -> Option<Actor>;

    /// since this is intended to be a dumb implimentation, the
    /// "password" being passed in should be the hashed argon2
    /// output containing the hash and the salt. the database
    /// should not be responsible for performing this task
    async fn create_local_user(&self, user: &NewLocal) -> Result<i64, ()>;

    async fn set_permission_level(&self, uid: i64, permission_level: PermissionLevel);
    async fn update_password(&self, uid: i64, password: &str);
    async fn set_manually_approves_followers(&self, uid: i64, value: bool);

    async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64>;

    /// instance_domain must be provided as internal users will
    /// need to have their links generated based on the instance
    /// domain. instances running in local only mode should be able
    /// to change domains without any affect for the internal users
    ///
    /// in the case of users using a custom domain name, it will take
    /// precidence over the user. how exactly this will be implimented
    /// is not set in stone but we are keeping the door open to it so
    /// that once a nice system is figured out we can impliment it
    /// without too much hastle
    // async fn get_local_user_actor(
    //     &self,
    //     preferred_username: &str,
    //     instance_domain: &str,
    // ) -> Option<Actor>;
    async fn get_local_user_actor(
        &self,
        preferred_username: &str,
        instance_domain: &str,
    ) -> Option<(Actor, i64)>;

    /// see documentation for [`Conn::get_local_user_actor()`] for more
    /// info on instance domain
    async fn get_local_user_actor_db_id(&self, uid: i64, instance_domain: &str) -> Option<Actor>;
    // async fn get_local_user_private_key(&self, preferred_username: &str) -> String;
    async fn get_local_user_private_key(&self, preferred_username: &str) -> String;

    async fn create_new_post(
        &self,
        post: &PostType,
        instance_domain: &str,
        is_local: bool,
        uid: i64,
        in_reply_to: Option<i64>,
    ) -> i64;

    async fn create_follow_request(&self, from_id: &str, to_id: &str) -> Result<(), ()>;

    /// approves an existing follow request and creates the record in
    /// the followers
    async fn approve_follow_request(&self, from_id: &str, to_id: &str) -> Result<(), ()>;

    /// in the event that we cannot view from the source instance, just show
    /// local followers
    async fn get_followers(&self, preferred_username: &str) -> Result<(), ()>;

    /// in the event we cannot view from the source domain, just show
    /// the source instance has not made this information available
    async fn get_follower_count(&self, preferred_username: &str) -> Result<(), ()>;

    async fn get_post(&self, object_id: i64) -> Option<PostType>;

    async fn get_instance_actor(&self) -> Option<InstanceActor>;
    async fn create_instance_actor(&self, private_key_pem: String, public_key_pem: String);
}
