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
            RangeLinkActor::Actor(x) => &x.extends_object.id.id,
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
    #[serde(flatten)]
    pub preferred_username: String,
    pub extends_object: Object,
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
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec![
                    "https://www.w3.org/ns/activitystreams".to_owned(),
                    "https://w3id.org/security/v1".to_owned(),
                ]),
                activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Actor(Box::new(
                    self,
                ))),
            },
        }
    }
    pub fn get_id(&self) -> &Url {
        &self.extends_object.id.id
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
                    "https://www.w3.org/ns/activitystreams".to_owned(),
                    "https://w3id.org/security/v1".to_owned(),
                ]),
                activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Actor(value)),
            },
        }
    }
}
