use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    activities::*,
    actors::Actor,
    collections::ExtendsCollection,
    object::{Object, ObjectWrapper},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityStream {
    #[serde(flatten)]
    pub content: ContextWrap,
}

impl std::fmt::Display for ActivityStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deserialized = serde_json::to_string(self).unwrap();
        write!(f, "{}", deserialized)
    }
}

impl ActivityStream {
    // pub fn get_actor(self) -> Option<Box<Actor>> {
    //     match self.content.activity_stream {
    //         ExtendsObject::Actor(x) => Some(x),
    //         _ => None,
    //     }
    // }
    // pub fn get_activity(self) -> Option<Box<ExtendsIntransitive>> {
    //     match self.content.activity_stream {
    //         ExtendsObject::ExtendsIntransitive(x) => Some(x),
    //         _ => None,
    //     }
    // }
    // pub fn get_object(self) -> Option<Box<ObjectWrapper>> {
    //     match self.content.activity_stream {
    //         ExtendsObject::Object(x) => Some(x),
    //         _ => None,
    //     }
    // }
    // pub fn get_extends_object(self) -> ExtendsObject {
    //     self.content.activity_stream
    // }
    // pub fn is_activity(&self) -> bool {
    //     matches!(
    //         &self.content.activity_stream,
    //         ExtendsObject::ExtendsIntransitive(_)
    //     )
    // }
    // pub fn get_owner(&self) -> Option<&Url> {
    //     match &self.content.activity_stream {
    //         ExtendsObject::Object(x) => Some(x.object.get_attributed_to()),
    //         ExtendsObject::ExtendsIntransitive(x) => Some(x.get_actor()),
    //         ExtendsObject::ExtendsCollection(_) => None,
    //         ExtendsObject::Actor(x) => Some(x.get_id()),
    //     }
    // }
}

//-------------------glue--------------
#[derive(Serialize, Deserialize, Debug, Clone)]
/// wraps base object to include context
pub struct ContextWrap {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: ExtendsObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Context {
    Array(Vec<ContextItem>),
    Single(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextItem {
    String(String),
    Map(HashMap<String, ContextMapItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextMapItem {
    String(String),
    Map(HashMap<String, String>),
}

//--------------------inheritence---------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsObject {
    Object(Box<ObjectWrapper>),
    ExtendsIntransitive(Box<ExtendsIntransitive>),
    ExtendsCollection(Box<ExtendsCollection>),
    Actor(Box<Actor>),
}

impl ExtendsObject {
    pub fn get_as_object(&self) -> Option<&Object> {
        let ExtendsObject::Object(object) = self else {
            return None;
        };
        Some(&object.object)
    }
    pub fn get_as_activity(&self) -> Option<&ExtendsIntransitive> {
        let ExtendsObject::ExtendsIntransitive(activity) = self else {
            return None;
        };
        Some(activity)
    }
    pub fn get_id(&self) -> &Url {
        match self {
            ExtendsObject::Object(x) => &x.object.id,
            ExtendsObject::ExtendsIntransitive(x) => x.get_id(),
            ExtendsObject::ExtendsCollection(_x) => todo!(),
            ExtendsObject::Actor(x) => &x.id,
        }
    }
}

//--------------primitive-----------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be a single item or array of items
pub enum OptionalArray<T> {
    Single(T),
    Multiple(Vec<T>),
}
