use actix_web::web::Data;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activitystream_objects::actors::Actor,
    protocol::{
        ap_protocol::fetch::authorized_fetch,
        errors::FetchErr,
        versia_protocol::{requests::Signer, verify::VersiaVerificationCache},
    },
    versia_types::{
        entities::{
            instance_metadata::InstanceMetadata, public_key::AlgorithmsPublicKey, user::User,
        },
        postable::Postable,
    },
};

use super::{
    utility::{instance_actor::InstanceActor, new_actor::NewLocal, protocols::Protocols},
    Follower, Like, PermissionLevel, PostType,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InsertErr {
    AlreadyExists,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DbErr {
    FetchErr(FetchErr),
    InsertErr(InsertErr),
    InvalidType,
}

impl std::fmt::Display for DbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

/// the origin of a post containing its instance domain
pub enum EntityOrigin<'a> {
    Local(&'a str),
    Federated(&'a str),
}

pub struct VersiaConn<'a> {
    pub conn: &'a Data<Box<dyn Conn + Sync>>,
}

impl VersiaVerificationCache for VersiaConn<'_> {
    async fn get_key(&self, signed_by: &Signer) -> Option<AlgorithmsPublicKey> {
        self.conn.get_key(signed_by).await
    }
}

#[async_trait]
pub trait Conn: Sync {
    // versia new
    async fn get_user_post_count(&self, uuid: &str, origin: &EntityOrigin) -> Option<u64>;
    /// ofset is one based
    async fn get_user_posts_versia(
        &self,
        uuid: &str,
        origin: &EntityOrigin,
        page_size: u64,
        ofset: u64,
    ) -> Option<Vec<Postable>>;
    async fn get_key(&self, signed_by: &Signer) -> Option<AlgorithmsPublicKey>;
    async fn get_versia_instance_metadata(&self, instance_domain: &str) -> InstanceMetadata;
    /// get the protocol of the given instance. will backfill if the instance isn't in the db
    async fn get_protocol(&self, instance: &str) -> Protocols;
    async fn get_versia_user(&self, uuid: &str, origin: &EntityOrigin) -> Option<User>;
    async fn get_versia_post(&self, post_id: &str, origin: &EntityOrigin) -> Option<Postable>;
    /// create a post and return the post
    async fn create_versia_post(
        &self,
        post: Postable,
        origin: &EntityOrigin,
    ) -> Result<Postable, ()>;
    async fn delete_post(&self, post_id: &str, origin: &EntityOrigin) -> Result<(), ()>;
    async fn delete_user(&self, uuid: &str, origin: &EntityOrigin) -> Result<(), ()>;

    /// run any prep for the database, for example running migrations
    async fn init(&self) -> Result<(), String>;

    //-------------------instance actor------------------------------
    /// gets the instance actor. creates one if its not present
    async fn get_instance_actor(&self) -> InstanceActor;
    // async fn create_instance_actor(&self, private_key_pem: &str, public_key_pem: &str);

    //----------------------actors---------------------------

    // async fn get_key(&self, signed_by: &Url) -> Option<AlgorithmsPublicKey>;

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
    async fn get_actor(&self, uid: i64, instance_domain: &str) -> Option<Actor>;
    async fn get_local_user_actor(
        &self,
        preferred_username: &str,
        instance_domain: &str,
    ) -> Option<(Actor, i64)>;

    async fn is_local(&self, uid: i64) -> bool;

    async fn get_federated_db_id(&self, actor_id: &str) -> Option<i64>;
    async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64>;

    async fn get_federated_actor(&self, actor_id: &str) -> Option<Actor>;

    //-----------------------account managment-----------------------------

    /// since this is intended to be a dumb implimentation, the
    /// "password" being passed in should be the hashed argon2
    /// output containing the hash and the salt. the database
    /// should not be responsible for performing this task
    async fn update_password(&self, uid: i64, password: &str);
    async fn set_manually_approves_followers(&self, uid: i64, value: bool);
    async fn get_local_manually_approves_followers(&self, uid: i64) -> bool;
    async fn set_permission_level(&self, uid: i64, permission_level: PermissionLevel);

    //------------------------------posts---------------------------------

    async fn create_new_post(
        &self,
        post: &PostType,
        instance_domain: &str,
        uid: i64,
        is_local: bool,
        in_reply_to: Option<i64>,
    ) -> i64;

    async fn get_post(&self, object_id: i64) -> Option<PostType>;

    //------------------------------likes-----------------------------------

    async fn create_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    async fn remove_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    async fn get_post_likes(&self, obj_id: i64) -> Result<Vec<Like>, ()>;
    async fn get_user_likes(&self, uid: i64) -> Result<Vec<Like>, ()>;

    //-------------------------private keys----------------------------

    // async fn get_local_user_private_key(&self, preferred_username: &str) -> String;
    async fn get_local_user_private_key_db_id(&self, uid: i64) -> String;

    //----------------------managing actors-------------------------------

    ///used for deleting both federated and local accounts
    async fn delete_actor(&self, uid: i64, reason: Option<&str>) -> Result<(), ()>;

    async fn create_local_user(&self, user: &NewLocal) -> Result<i64, DbErr>;
    async fn create_federated_actor(&self, actor: &Actor) -> i64;

    ///instance domain needed to form the instance actor for the request
    async fn load_new_federated_actor(
        &self,
        actor_id: &Url,
        instance_domain: &str,
    ) -> Result<i64, DbErr> {
        let instance_actor = self.get_instance_actor().await;
        let key_id = InstanceActor::pub_key_id(instance_domain);

        let fetched =
            authorized_fetch(actor_id, &key_id, &mut instance_actor.get_private_key()).await;
        let fetched = match fetched {
            Ok(x) => x,
            Err(x) => return Err(DbErr::FetchErr(x)),
        };

        let actor = fetched.get_actor();
        let actor = match actor {
            Some(x) => x,
            None => return Err(DbErr::InvalidType),
        };

        Ok(self.create_federated_actor(&actor).await)
    }

    //--------------------followers---------------------------------

    async fn create_follow_request(&self, from: i64, to: i64, pending: bool) -> Result<(), ()>;

    /// approves an existing follow request and creates the record in
    /// the followers
    async fn approve_follow_request(&self, from: i64, to: i64) -> Result<(), ()>;

    /// in the event that we cannot view from the source instance, just show
    /// local followers
    async fn get_followers(&self, user: i64) -> Result<Vec<Follower>, ()>;

    /// really just for local users, if used for a federated user it
    /// will only show the amout of local users following them
    async fn get_follower_count(&self, user: i64) -> Result<i64, ()>;

    async fn get_follow(&self, from_id: i64, to_id: i64) -> Option<Follower>;
}
