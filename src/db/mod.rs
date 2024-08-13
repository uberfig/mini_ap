// pub mod account_creation;
// pub mod actor_utilities;
// pub mod pg_conn;
pub mod postgres;
// pub mod internal_actor;
// pub mod private_key;
// pub mod public_key;

use async_trait::async_trait;
use url::Url;

use crate::activitystream_objects::{activities::Question, actors::Actor, object::ObjectWrapper};

#[derive(Debug, Clone, Copy)]
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

impl From<i16> for PermissionLevel {
    fn from(value: i16) -> Self {
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
impl From<PermissionLevel> for i16 {
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

pub struct UserLinks {
    pub id: Url,
    pub inbox: Url,
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,
    pub liked: Url,
    pub url: Url,
    pub pub_key_id: Url,
}

pub fn generate_links(domain: &str, uname: &str) -> UserLinks {
    UserLinks {
        id: Url::parse(&format!("https://{domain}/users/{uname}")).unwrap(),
        inbox: Url::parse(&format!("https://{domain}/users/{uname}/inbox")).unwrap(),
        outbox: Url::parse(&format!("https://{domain}/users/{uname}/outbox")).unwrap(),
        followers: Url::parse(&format!("https://{domain}/users/{uname}/followers")).unwrap(),
        following: Url::parse(&format!("https://{domain}/users/{uname}/following")).unwrap(),
        liked: Url::parse(&format!("https://{domain}/users/{uname}/liked")).unwrap(),
        url: Url::parse(&format!("https://{domain}/@{uname}")).unwrap(),
        pub_key_id: Url::parse(&format!("https://{domain}/users/{uname}#main-key")).unwrap(),
    }
}

fn instance_actor_links(domain: &str) -> UserLinks {
    UserLinks {
        id: Url::parse(&format!("https://{domain}/actor")).unwrap(),
        inbox: Url::parse(&format!("https://{domain}/actor/inbox")).unwrap(),
        outbox: Url::parse(&format!("https://{domain}/actor/outbox")).unwrap(),
        followers: Url::parse(&format!("https://{domain}/actor/followers")).unwrap(),
        following: Url::parse(&format!("https://{domain}/actor/following")).unwrap(),
        liked: Url::parse(&format!("https://{domain}/actor/liked")).unwrap(),
        url: Url::parse(&format!("https://{domain}:")).unwrap(),
        pub_key_id: Url::parse(&format!("https://{domain}/actor#main-key")).unwrap(),
    }
}

pub struct NewLocal {
    pub username: String,
    pub password: String,
    pub email: String,
    pub permission_level: PermissionLevel,
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub custom_domain: Option<String>,
}

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
    ) -> Option<Actor>;

    /// see documentation for [`Conn::get_local_user_actor()`] for more
    /// info on instance domain
    async fn get_local_user_actor_db_id(&self, uid: i64, instance_domain: &str) -> Option<Actor>;
    // async fn get_local_user_private_key(&self, preferred_username: &str) -> String;
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
