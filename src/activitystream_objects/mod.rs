pub mod activities;
pub mod actors;
pub mod collections;
pub mod core_types;
pub mod link;
pub mod object;

use actors::PublicKey;
use serde::{Deserialize, Serialize};
use url::Url;

//-----------old implimentation kept in this file until I get rid of it, depreciated----------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OldActorType {
    Person,
    Other,
}

impl From<String> for OldActorType {
    fn from(value: String) -> Self {
        if value.eq_ignore_ascii_case("Person") {
            return OldActorType::Person;
        }
        OldActorType::Other
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PublicKey {
//     pub id: String,    //https://my-example.com/actor#main-key
//     pub owner: String, //"https://my-example.com/actor"
//     pub public_key_pem: String,
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OldActor {
    #[serde(skip)]
    pub database_id: i64,
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: OldActorType,
    #[serde(skip)]
    pub name: Option<String>,
    pub preferred_username: String,
    #[serde(skip)]
    pub domain: String,
    #[serde(skip)]
    pub summary: String,
    pub inbox: String,
    #[serde(skip)]
    pub outbox: String,
    #[serde(skip)]
    pub followers: String,
    #[serde(skip)]
    pub following: String,
    #[serde(skip)]
    pub liked: Option<String>,

    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
///Actor type for just deserializing the useful bits for verifying post came from an actor
pub struct VerificationActor {
    pub id: Url,
    #[serde(rename = "type")]
    pub type_field: OldActorType,
    pub preferred_username: String,
    pub public_key: PublicKey,
}

impl From<DatabaseActor> for OldActor {
    fn from(value: DatabaseActor) -> Self {
        OldActor {
            database_id: value.ap_user_id,
            context: vec![
                "https://www.w3.org/ns/activitystreams".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            type_field: value.type_field,
            id: value.id,
            name: value.name,
            preferred_username: value.preferred_username,
            domain: value.domain,
            summary: value.summary,
            inbox: value.inbox,
            outbox: value.outbox,
            followers: value.followers,
            following: value.following,
            liked: value.liked,
            public_key: value.public_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseActor {
    #[serde(skip)]
    pub ap_user_id: i64,
    #[serde(rename = "type")]
    pub type_field: OldActorType,
    pub id: String,
    pub name: Option<String>,
    pub preferred_username: String,
    #[serde(skip)]
    pub domain: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    #[serde(skip)]
    pub liked: Option<String>,

    pub public_key: PublicKey,
}
#[derive(Serialize, Deserialize)]
pub enum ObjectType {
    Note,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamObject {
    #[serde(rename = "type")]
    pub type_field: ObjectType,
    pub id: String,
    pub attributed_to: String,
    pub to: String,
    pub in_reply_to: Option<String>,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum ActivityType {
    Create,
    Like,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityObjType {
    Object(StreamObject),
    Link(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OldActivity {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: ActivityType,
    pub id: String,
    // pub to: Vec<String>,
    pub actor: String,
    pub object: StreamObject,
}
