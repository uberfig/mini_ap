use actix_web::web::Data;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activitystream_objects::actors::Actor,
    cryptography::openssl::OpenSSLPublic,
    protocol::{
        errors::FetchErr,
        versia_protocol::{requests::Signer, verify::VersiaVerificationCache},
    },
    versia_types::{
        entities::{instance_metadata::InstanceMetadata, user::User},
        postable::Postable,
    },
};

use super::{
    utility::{instance_actor::InstanceActor, new_actor::NewLocal, protocols::Protocol},
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
    async fn get_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic> {
        self.conn.get_public_key(signed_by).await
    }
}

pub enum ProtoUser {
    Versia(User),
    ActivityPub(Actor),
}

#[async_trait]
pub trait Conn: Sync {
    /// run any prep for the database, for example running migrations
    async fn init(&self) -> Result<(), String>;
    /// gets the instance actor. creates one if its not present
    async fn get_instance_actor(&self) -> InstanceActor;

    /// returns the uid if sucessful
    async fn create_user(&self, domain: &str, content: &NewLocal) -> Result<String, ()>;
    /// backfills if not in db
    async fn get_actor(&self, uuid: &str, origin: &EntityOrigin) -> Option<u64>;
    /// only gets an actor we have authority over, does not backfill
    async fn get_local_actor(&self, username: &str, domain: &str) -> Option<Actor>;

    /// signed_by will always be user for activitypub users
    /// this will backfill the user if they aren't in the db yet
    async fn get_public_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic>;

    //-------------------------versia---------------------

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
    async fn get_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic>;
    /// gets the metadata of an instance, backfills if not present
    async fn get_versia_instance_metadata(&self, instance_domain: &str)
        -> Option<InstanceMetadata>;
    /// get the protocol of the given instance. will backfill if the instance isn't in the db
    async fn get_protocol(&self, instance: &str) -> Protocol;
    async fn get_versia_user(&self, uuid: &str, origin: &EntityOrigin) -> Option<User>;
    async fn get_versia_post(&self, post_id: &str, origin: &EntityOrigin) -> Option<Postable>;
    /// create a post and return the post
    async fn create_versia_post(
        &self,
        post: Postable,
        origin: &EntityOrigin,
    ) -> Result<Postable, ()>;
    async fn delete_post(&self, post_id: &str, origin: &EntityOrigin) -> Result<(), ()>;
    async fn delete_user(&self, uid: &Url, origin: &EntityOrigin) -> Result<(), ()>;

    // //----------------------actors---------------------------

    // /// instance_domain must be provided as internal users will
    // /// need to have their links generated based on the instance
    // /// domain. instances running in local only mode should be able
    // /// to change domains without any affect for the internal users
    // ///
    // /// in the case of users using a custom domain name, it will take
    // /// precidence over the user. how exactly this will be implimented
    // /// is not set in stone but we are keeping the door open to it so
    // /// that once a nice system is figured out we can impliment it
    // /// without too much hastle
    // async fn get_actor(&self, uid: i64, instance_domain: &str) -> Option<Actor>;
    // async fn get_local_user_actor(
    //     &self,
    //     preferred_username: &str,
    //     instance_domain: &str,
    // ) -> Option<(Actor, i64)>;

    // async fn is_local(&self, uid: i64) -> bool;

    // async fn get_federated_db_id(&self, actor_id: &str) -> Option<i64>;
    // async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64>;

    // async fn get_federated_actor(&self, actor_id: &str) -> Option<Actor>;

    // //-----------------------account managment-----------------------------

    // /// since this is intended to be a dumb implimentation, the
    // /// "password" being passed in should be the hashed argon2
    // /// output containing the hash and the salt. the database
    // /// should not be responsible for performing this task
    // async fn update_password(&self, uid: i64, password: &str);
    // async fn set_manually_approves_followers(&self, uid: i64, value: bool);
    // async fn get_local_manually_approves_followers(&self, uid: i64) -> bool;
    // async fn set_permission_level(&self, uid: i64, permission_level: PermissionLevel);

    // //------------------------------posts---------------------------------

    // async fn create_new_post(
    //     &self,
    //     post: &PostType,
    //     instance_domain: &str,
    //     uid: i64,
    //     is_local: bool,
    //     in_reply_to: Option<i64>,
    // ) -> i64;

    // async fn get_post(&self, object_id: i64) -> Option<PostType>;

    // //------------------------------likes-----------------------------------

    // // async fn create_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    // // async fn remove_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    // // async fn get_post_likes(&self, obj_id: i64) -> Result<Vec<Like>, ()>;
    // // async fn get_user_likes(&self, uid: i64) -> Result<Vec<Like>, ()>;

    // //-------------------------private keys----------------------------

    // /// get the private key of a local user, none if we don't have authority over them
    // async fn get_private_key_pem(&self, uid: &Url, origin: &EntityOrigin) -> Option<String>;

    // //----------------------managing actors-------------------------------

    // ///used for deleting both federated and local accounts
    // // async fn delete_user(&self, uid: &Url, origin: &EntityOrigin, reason: Option<&str>) -> Result<(), ()>;

    // async fn create_local_user(&self, user: &NewLocal) -> Result<i64, DbErr>;
    // async fn create_federated_actor(&self, actor: &Actor) -> i64;

    // ///instance domain needed to form the instance actor for the request
    // async fn load_new_federated_actor(
    //     &self,
    //     actor_id: &Url,
    //     instance_domain: &str,
    // ) -> Result<i64, DbErr>;

    // //--------------------followers---------------------------------

    // async fn create_follow_request(&self, from: i64, to: i64, pending: bool) -> Result<(), ()>;

    // /// approves an existing follow request and creates the record in
    // /// the followers
    // async fn approve_follow_request(&self, from: i64, to: i64) -> Result<(), ()>;

    // /// in the event that we cannot view from the source instance, just show
    // /// local followers
    // async fn get_followers(&self, user: i64) -> Result<Vec<Follower>, ()>;

    // /// really just for local users, if used for a federated user it
    // /// will only show the amout of local users following them
    // async fn get_follower_count(&self, user: i64) -> Result<i64, ()>;

    // async fn get_follow(&self, from_id: i64, to_id: i64) -> Option<Follower>;
}
