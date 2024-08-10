use chrono::{DateTime, NaiveDateTime, SecondsFormat};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    activities::{Activity, ExtendsIntransitive},
    actors::RangeLinkActor,
    collections::ExtendsCollection,
    core_types::{
        ActivityStream, Context, ContextWrap, ExtendsObject, LinkOrArray, RangeLinkExtendsObject,
        RangeLinkObjOrArray, SimpleLinkOrArray,
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
            id: Url::parse("https://invalid.com").unwrap(),
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

impl ObjectWrapper {
    pub fn to_activitystream(self) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Single("https://www.w3.org/ns/activitystreams".to_string()),
                // activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Object(Box::new(
                //     self,
                // ))),
                activity_stream: ExtendsObject::Object(Box::new(self)),
            },
        }
    }
    pub fn to_create_activitystream(self) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Single("https://www.w3.org/ns/activitystreams".to_string()),
                // activity_stream: RangeLinkExtendsObject::Object(
                //     ExtendsObject::ExtendsIntransitive(Box::new(
                //         ExtendsIntransitive::ExtendsActivity(Activity::new_create(self)),
                //     )),
                // ),
                activity_stream: ExtendsObject::ExtendsIntransitive(Box::new(
                    ExtendsIntransitive::ExtendsActivity(Activity::new_create(self)),
                )),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
    pub in_reply_to: Option<RangeLinkExtendsObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Box<ExtendsCollection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<SimpleLinkOrArray>,

    //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<SimpleLinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<LinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the private primary audience of this Object.
    pub bto: Option<SimpleLinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the public secondary audience of this Object.
    pub cc: Option<SimpleLinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<RangeLinkExtendsObject>,
}

impl Object {
    pub fn new(id: Url) -> Object {
        Object {
            id: ID { id },
            attributed_to: None,
            ..Default::default()
        }
    }
    pub fn get_attributed_to(&self) -> Option<&Url> {
        match &self.attributed_to {
            Some(x) => match x {
                RangeLinkActor::Actor(x) => Some(x.get_id()),
                RangeLinkActor::Link(x) => Some(x),
            },
            None => None,
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
    pub fn content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }
    pub fn set_id(mut self, id: Url) -> Self {
        self.id.id = id;
        self
    }
    pub fn in_reply_to(mut self, in_reply_to: Option<RangeLinkExtendsObject>) -> Self {
        self.in_reply_to = in_reply_to;
        self
    }
    pub fn published_milis(mut self, published: i64) -> Self {
        let test = DateTime::from_timestamp_millis(published).unwrap();
        let time = test.to_rfc3339_opts(SecondsFormat::Secs, true);
        self.published = Some(time);
        self
    }
    pub fn to_public(mut self) -> Self {
        self.to = Some(SimpleLinkOrArray::Multiple(vec![Url::parse(
            "https://www.w3.org/ns/activitystreams#Public",
        )
        .unwrap()]));
        self
    }
    pub fn wrap(self, obj_type: ObjectType) -> ObjectWrapper {
        ObjectWrapper {
            type_field: obj_type,
            object: self.to_public(),
        }
    }
    pub fn to_activitystream(self, obj_type: ObjectType) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Single("https://www.w3.org/ns/activitystreams".to_string()),
                // activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Object(Box::new(
                //     ObjectWrapper {
                //         type_field: obj_type,
                //         object: self,
                //     },
                // ))),
                activity_stream: ExtendsObject::Object(Box::new(ObjectWrapper {
                    type_field: obj_type,
                    object: self,
                })),
            },
        }
    }
}
