// pub mod account_creation;
// pub mod actor_utilities;
pub mod pg_conn;
// pub mod internal_actor;
// pub mod private_key;
// pub mod public_key;

use crate::activitystream_objects::{activities::Question, actors::Actor, object::ObjectWrapper};

pub enum PermissionLevel {
    /// intended for the main admin account(s) of the server, will be 
    /// featured and considered the pont of contact for the instance, 
    /// can be set to be auto followed by new users
    AdminOne, 
    /// intended for anyone who has admin access to the server
    AdminTwo, 
    /// intended for mods who can take vito actoin in an emergency
    ModOne,
    /// intended for mods who need to open a proposal for mod changes
    ModTwo,
    /// intended for public registration servers to limit things to only
    /// known users for example if they wish to have only known users or
    /// higher able to vote on proposals so that a malicious actor can't
    /// start making accounts to influence a decision. When manual approval
    /// is used, all approved users will be trusted and pending users will
    /// be untrusted. this would allow for a switching to manual approval
    /// in the event of an emergency still allowing trusted users to 
    /// continue unnaffected and untrusted accounts would be preserved and
    /// prompted to send an application for approval when they log in next
    TrustedUser,
    /// the default, what they can do is up to server policy, used for 
    /// accounts pending approval in a manual approval setup
    UntrustedUser,
}

#[derive(Debug, Clone)]
/// a concrete post to be stored in the database.
/// surtype of either object or question, then subtypes of their 
/// respective types, eg note, or for a question multi or single select
pub enum PostType {
    Object(ObjectWrapper),
    Question(Question),
}

impl From<PostType> for String {
    fn from(value: PostType) -> Self {
        match value {
            PostType::Object(_) => "Object".to_string(),
            PostType::Question(_) => "Question".to_string(),
        }
    }
}

impl From<u16> for PermissionLevel {
    fn from(value: u16) -> Self {
        match value {
            1 => PermissionLevel::AdminOne,
            2 => PermissionLevel::AdminTwo,
            3 => PermissionLevel::ModOne,
            4 => PermissionLevel::ModTwo,
            5 => PermissionLevel::TrustedUser,
            6 => PermissionLevel::UntrustedUser,
            _ => PermissionLevel::UntrustedUser,
        }
    }
}
impl From<PermissionLevel> for u16 {
    fn from(val: PermissionLevel) -> Self {
        match val {
            PermissionLevel::AdminOne => 1,
            PermissionLevel::AdminTwo => 2,
            PermissionLevel::ModOne => 3,
            PermissionLevel::ModTwo => 4,
            PermissionLevel::TrustedUser => 5,
            PermissionLevel::UntrustedUser => 6,
        }
    }
}

pub trait Conn {
    
    // async fn get_public_key(&self, owner: &str) -> String;
    // async fn insert_public_key(
    //     &self,
    //     id: &str,
    //     actor_id: &str,
    //     public_key_pem: &str,
    // ) -> Result<i64, ()>;
    
    async fn create_federated_user(&self, actor: Actor) -> i64;
    async fn get_federated_user_db_id(&self, actor_id: &str) -> Option<i64>;
    async fn get_federated_actor(&self, actor_id: &str) -> Option<Actor>;
    async fn get_federated_actor_db_id(&self, id: i64) -> Option<Actor>;
    
    /// since this is intended to be a dumb implimentation, the 
    /// "password" being passed in should be the hashed argon2 
    /// output containing the hash and the salt. the database 
    /// should not be responsible for performing this task
    async fn create_local_user(
        &self,
        username: String,
        password: String, 
        permission_level: PermissionLevel,
        custom_domain: Option<&str>,
    ) -> Result<i64, ()>;

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
    async fn get_local_user_actor(&self, preferred_username: &str, instance_domain: &str) -> Option<Actor>;
    /// see documentation for [`Conn::get_local_user_actor()`] for more 
    /// info on instance domain 
    async fn get_local_user_actor_db_id(&self, id: i64, instance_domain: &str) -> Option<Actor>;
    async fn get_local_user_private_key(&self, preferred_username: &str) -> String;

    async fn create_new_post(&self, post: PostType) -> i64;

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
}

