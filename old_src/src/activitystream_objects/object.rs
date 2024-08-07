use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    actors::RangeLinkActor,
    collections::ExtendsCollection,
    core_types::{
        ActivityStream, Context, ContextWrap, ExtendsObject, LinkOrArray, RangeLinkExtendsObject,
        RangeLinkObjOrArray,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ID {
    pub id: Url,
}

impl From<Url> for ID {
    fn from(value: Url) -> Self {
        ID { id: value }
    }
}

impl ID {
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
    pub fn domain(&self) -> Option<&str> {
        self.id.domain()
    }
}

impl From<ID> for Url {
    fn from(val: ID) -> Self {
        val.id
    }
}

impl Default for ID {
    fn default() -> Self {
        Self {
            id: Url::parse("invalid").unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    #[serde(rename = "text/html")]
    Html,
    #[serde(rename = "text/markdown")]
    Markdown,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ObjectType {
    #[default]
    Object,
    Relationship, //adds properties: subject | object | relationship
    /// Represents any kind of multi-paragraph written work.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-article
    Article,

    /// Represents a short written work typically less than a single paragraph in length.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note
    Note,

    Document,
    Audio, //document type
    Image, //document type
    Video, //document type
    Page,  //document type

    Event,
    Place, //not used, adds  accuracy | altitude | latitude | longitude | radius | units

    Profile,   // adds describes
    Tombstone, // adds formerType | deleted
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectWrapper {
    #[serde(rename = "type")]
    pub type_field: ObjectType,
    #[serde(flatten)]
    pub object: Object,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_to: Option<RangeLinkActor>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies the entity (e.g. an application) that generated the object
    pub generator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<RangeLinkExtendsObject>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<RangeLinkExtendsObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<xsd_types::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Box<ExtendsCollection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<xsd_types::DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<xsd_types::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<LinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the private primary audience of this Object.
    pub bto: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the public secondary audience of this Object.
    pub cc: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl Object {
    pub fn new(id: Url) -> Object {
        Object {
            id: ID { id },
            attributed_to: None,
            ..Default::default()
        }
    }
    pub fn attributed_to_link(mut self, attributed_to: Option<Url>) -> Self {
        match attributed_to {
            Some(x) => {
                self.attributed_to = Some(RangeLinkActor::Link(x));
                self
            }
            None => {
                self.attributed_to = None;
                self
            }
        }
    }
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }
    pub fn to_activitystream(self) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec!["test1".to_string(), "test2".to_string()]),
                activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Object(Box::new(
                    ObjectWrapper {
                        type_field: ObjectType::Object,
                        object: self,
                    },
                ))),
            },
        }
    }
}
