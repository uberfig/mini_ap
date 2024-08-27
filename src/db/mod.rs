pub mod conn;
pub mod incoming;
pub mod postgres;

use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::DateTime;
use conn::Conn;
use openssl::{pkey::Private, rsa::Rsa};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::activitystream_objects::{
    activities::Question,
    actors::{Actor, PublicKey},
    core_types::ActivityStream,
    object::ObjectWrapper,
};

#[derive(Debug, Clone, Copy)]
pub enum UserRef {
    Local(i64),
    Activitypub(i64),
}

impl UserRef {
    /// outputs (fedi, local)
    pub fn parts(self) -> (Option<i64>, Option<i64>) {
        match self {
            UserRef::Local(x) => (None, Some(x)),
            UserRef::Activitypub(x) => (Some(x), None),
        }
    }
    pub fn id(self) -> i64 {
        match self {
            UserRef::Local(x) => x,
            UserRef::Activitypub(x) => x,
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            UserRef::Local(_) => true,
            UserRef::Activitypub(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FollowType {
    LocalToLocal,
    LocalToFedi,
    FediToLocal,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostSupertype {
    Object,
    Question,
}
impl PostSupertype {
    pub fn from_str(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Debug, Clone)]
/// a concrete post to be stored in the database.
/// surtype of either object or question, then subtypes of their
/// respective types, eg note, or for a question multi or single select
pub enum PostType {
    Object(ObjectWrapper),
    Question(Question),
}
impl PostType {
    pub fn to_create_activitystream(self) -> ActivityStream {
        match self {
            PostType::Object(x) => x.to_create_activitystream(),
            PostType::Question(_) => todo!(),
        }
    }
    pub fn get_surtype(&self) -> String {
        match self {
            PostType::Object(_) => serde_json::to_string(&PostSupertype::Object).unwrap(),
            PostType::Question(_) => serde_json::to_string(&PostSupertype::Question).unwrap(),
        }
    }
    pub fn get_subtype(&self) -> String {
        match self {
            PostType::Object(x) => serde_json::to_string(&x.type_field).unwrap(),
            PostType::Question(x) => serde_json::to_string(&x.type_field).unwrap(),
        }
    }
    pub fn get_published(&self) -> &Option<String> {
        match self {
            PostType::Object(x) => &x.object.published,
            PostType::Question(_) => todo!(),
        }
    }
    pub fn get_id(&self) -> &str {
        match self {
            PostType::Object(x) => x.get_id().as_str(),
            PostType::Question(_) => todo!(),
        }
    }
}

impl From<PostType> for ActivityStream {
    fn from(value: PostType) -> Self {
        match value {
            PostType::Object(x) => x.to_activitystream(),
            PostType::Question(_x) => todo!(),
        }
    }
}

impl From<PostType> for String {
    fn from(value: PostType) -> Self {
        match value {
            PostType::Object(_) => "Object".to_string(),
            PostType::Question(_) => "Question".to_string(),
        }
    }
}

pub fn get_post_id_and_published(
    is_local: bool,
    post: &PostType,
) -> (std::option::Option<String>, i64) {
    match is_local {
        true => (
            None,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        ),
        false => {
            let time = match post.get_published() {
                Some(x) => {
                    let parsed = DateTime::parse_from_rfc3339(x);
                    match parsed {
                        Ok(x) => x.timestamp_millis(),
                        Err(_) => SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64,
                    }
                }
                None => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
            };

            (Some(post.get_id().to_string()), time)
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

pub fn instance_actor_links(domain: &str) -> UserLinks {
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

pub struct InstanceActor {
    pub private_key_pem: String,
    pub public_key_pem: String,
}

impl InstanceActor {
    pub fn pub_key_id(domain: &str) -> String {
        format!("https://{domain}/actor#main-key")
    }
    pub async fn init_instance_actor(conn: &Box<dyn Conn + Sync>) {
        if conn.get_instance_actor().await.is_none() {
            let rsa = Rsa::generate(2048).unwrap();
            let private_key_pem = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
            let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
            conn.create_instance_actor(private_key_pem, public_key_pem)
                .await;
        }
    }
    pub fn get_rsa(&self) -> Rsa<Private> {
        openssl::rsa::Rsa::private_key_from_pem(self.private_key_pem.as_bytes()).unwrap()
    }
    pub fn to_actor(&self, domain: &str) -> Actor {
        let links = instance_actor_links(domain);
        Actor {
            type_field: crate::activitystream_objects::actors::ActorType::Application,
            id: links.id.clone(),
            preferred_username: domain.to_string(),
            summary: None,
            name: None,
            url: Some(
                Url::parse(&format!("https://{domain}/about/more?instance_actor=true")).unwrap(),
            ),
            public_key: PublicKey {
                id: links.pub_key_id,
                owner: links.id,
                public_key_pem: self.public_key_pem.clone(),
            },
            inbox: links.inbox,
            outbox: links.outbox,
            followers: links.followers,
            following: links.following,
            domain: Some(domain.to_string()),
            liked: Some(links.liked),
        }
    }
}

pub struct NewLocal {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub permission_level: PermissionLevel,
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub custom_domain: Option<String>,
}

impl NewLocal {
    pub fn new(
        username: String,
        password: String,
        email: Option<String>,
        custom_domain: Option<String>,
        permission_level: Option<PermissionLevel>,
    ) -> Self {
        let permission_level = match permission_level {
            Some(x) => x,
            None => PermissionLevel::UntrustedUser,
        };
        let rsa = Rsa::generate(2048).unwrap();
        let private_key_pem = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        NewLocal {
            username,
            password: password_hash,
            email,
            permission_level,
            private_key_pem,
            public_key_pem,
            custom_domain,
        }
    }
}
