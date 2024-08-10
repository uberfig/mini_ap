use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use super::core_types::*;

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
    // Actor,
    /// Describes a software application.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application
    Application,
    /// Represents a formal or informal collective of Actors.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group
    Group,
    /// Represents an organization.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization
    Organization,
    /// Represents an individual person. The most
    /// common type of actor on the fedi
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person
    Person,
    /// Represents a service of any kind.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service
    Service,
}

//-------------------types--------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: Url,    //https://my-example.com/actor#main-key
    pub owner: Url, //"https://my-example.com/actor"
    pub public_key_pem: String,
}
impl From<String> for PublicKey {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Actor types are [`Object`] types that are capable of performing activities
///
/// core types:
/// - [`ActorType::Application`]
/// - [`ActorType::Group`]
/// - [`ActorType::Organization`]
/// - [`ActorType::Person`]
/// - [`ActorType::Service`]
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
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

    pub inbox: Url,
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,

    // #[serde(skip)]
    // pub ap_user_id: Option<i64>,
    #[serde(skip)]
    pub domain: Option<String>,
    #[serde(skip)]
    pub liked: Option<Url>,
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
