use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use super::{core_types::*, object::Object};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be an actor or a link
pub enum RangeLinkActor {
    Actor(Box<Actor>),
    Link(Url),
}

impl Default for RangeLinkActor {
    fn default() -> Self {
        RangeLinkActor::Link(Url::parse("invalid").unwrap())
    }
}

impl RangeLinkActor {
    pub fn get_id(&self) -> &Url {
        match self {
            RangeLinkActor::Actor(x) => &x.id,
            RangeLinkActor::Link(x) => x,
        }
    }
}

impl PartialEq for RangeLinkActor {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorType {
    Actor,
    Application,
    Group,
    Organization,
    Person,
    Service,
}

//-------------------types--------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,    //https://my-example.com/actor#main-key
    pub owner: String, //"https://my-example.com/actor"
    pub public_key_pem: String,
}
impl From<String> for PublicKey {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// summary, id, and name are inherited from [`Object`]
pub struct Actor {
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub id: Url,
    pub preferred_username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    pub public_key: PublicKey,

    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,

    #[serde(skip)]
    pub ap_user_id: Option<i64>,
    #[serde(skip)]
    pub domain: Option<String>,
    #[serde(skip)]
    pub liked: Option<String>,
}

impl Actor {
    pub fn to_activitystream(self) -> ActivityStream {
        // let mut test: HashMap<String, ContextMapItem> = HashMap::new();
        // let mut item: HashMap<String, String> = HashMap::new();
        // item.insert("@id".to_string(), "toot:featuredTags".to_string());
        // item.insert("@type".to_string(), "@id".to_string());
        // test.insert("featuredTags".to_string(), ContextMapItem::Map(item));
        // test.insert("manuallyApprovesFollowers".to_string(), ContextMapItem::String("as:manuallyApprovesFollowers".to_string()));
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec![
                    ContextItem::String("https://www.w3.org/ns/activitystreams".to_owned()),
                    ContextItem::String("https://w3id.org/security/v1".to_owned()),
                    // ContextItem::Map(test)
                ]),
                // activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Actor(Box::new(
                //     self,
                // ))),
                activity_stream: ExtendsObject::Actor(Box::new(self)),
            },
        }
    }
    pub fn get_id(&self) -> &Url {
        &self.id
    }
}

impl From<Actor> for ActivityStream {
    fn from(value: Actor) -> ActivityStream {
        value.to_activitystream()
    }
}

impl From<Box<Actor>> for ActivityStream {
    fn from(value: Box<Actor>) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec![
                    ContextItem::String("https://www.w3.org/ns/activitystreams".to_owned()),
                    ContextItem::String("https://w3id.org/security/v1".to_owned()),
                ]),
                // activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Actor(value)),
                activity_stream: ExtendsObject::Actor(value),
            },
        }
    }
}
