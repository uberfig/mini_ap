use chrono::{DateTime, SecondsFormat};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    activities::{Activity, ExtendsIntransitive, Question},
    actors::Actor,
    collections::ExtendsCollection,
    core_types::{ActivityStream, Context, ContextWrap, ExtendsObject, OptionalArray},
    link::{LinkSimpleOrExpanded, RangeLinkItem},
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PostType {
    Object(RangeLinkItem<ObjectWrapper>),
    Question(RangeLinkItem<Question>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_to: Option<RangeLinkItem<Actor>>,
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
    pub in_reply_to: Option<RangeLinkItem<ExtendsObject>>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub replies: Option<Box<ExtendsCollection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<OptionalArray<RangeLinkItem<Actor>>>,

    //TODO
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub attachment: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub start_time: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tag: Option<SimpleLinkOrArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the private primary audience of this Object.
    pub bto: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the public secondary audience of this Object.
    pub cc: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<RangeLinkItem<ExtendsObject>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<ExtendsCollection>,
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
                RangeLinkItem::Item(x) => Some(x.get_id()),
                RangeLinkItem::Link(x) => Some(x.get_id()),
            },
            None => None,
        }
    }
    pub fn attributed_to_link(mut self, attributed_to: Option<Url>) -> Self {
        match attributed_to {
            Some(x) => {
                self.attributed_to = Some(RangeLinkItem::Link(LinkSimpleOrExpanded::Simple(x)));
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
    pub fn in_reply_to(mut self, in_reply_to: Option<RangeLinkItem<ExtendsObject>>) -> Self {
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
        self.to = Some(OptionalArray::Multiple(vec![RangeLinkItem::Link(
            super::link::LinkSimpleOrExpanded::Simple(
                Url::parse("https://www.w3.org/ns/activitystreams#Public").unwrap(),
            ),
        )]));
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

#[cfg(test)]
mod tests {
    use crate::activitystream_objects::{core_types::ActivityStream, object::ObjectType};

    #[test]
    fn test_deserialize_note() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon/statuses/112769333503182077
        let test_note = r##"
{
	"@context": [
		"https://www.w3.org/ns/activitystreams",
		{
			"ostatus": "http://ostatus.org#",
			"atomUri": "ostatus:atomUri",
			"inReplyToAtomUri": "ostatus:inReplyToAtomUri",
			"conversation": "ostatus:conversation",
			"sensitive": "as:sensitive",
			"toot": "http://joinmastodon.org/ns#",
			"votersCount": "toot:votersCount",
			"Hashtag": "as:Hashtag"
		}
	],
	"id": "https://mastodon.social/users/Mastodon/statuses/112769333503182077",
	"type": "Note",
	"summary": null,
	"inReplyTo": null,
	"published": "2024-07-11T18:44:32Z",
	"url": "https://mastodon.social/@Mastodon/112769333503182077",
	"attributedTo": "https://mastodon.social/users/Mastodon",
	"to": [
		"https://www.w3.org/ns/activitystreams#Public"
	],
	"cc": [
		"https://mastodon.social/users/Mastodon/followers",
		"https://mastodon.social/users/mellifluousbox",
		"https://mastodon.social/users/Gargron"
	],
	"sensitive": false,
	"atomUri": "https://mastodon.social/users/Mastodon/statuses/112769333503182077",
	"inReplyToAtomUri": null,
	"conversation": "tag:mastodon.social,2024-07-11:objectId=749871110:objectType=Conversation",
	"content": "<p>We’re hiring again! The Mastodon team is looking for a part-time <a href=\"https://mastodon.social/tags/Finance\" class=\"mention hashtag\" rel=\"tag\">#<span>Finance</span></a> / <a href=\"https://mastodon.social/tags/Ops\" class=\"mention hashtag\" rel=\"tag\">#<span>Ops</span></a> Associate to support <span class=\"h-card\" translate=\"no\"><a href=\"https://mastodon.social/@mellifluousbox\" class=\"u-url mention\">@<span>mellifluousbox</span></a></span> + <span class=\"h-card\" translate=\"no\"><a href=\"https://mastodon.social/@Gargron\" class=\"u-url mention\">@<span>Gargron</span></a></span>.</p><p>This is a <a href=\"https://mastodon.social/tags/remote\" class=\"mention hashtag\" rel=\"tag\">#<span>remote</span></a> position and requires working proficiency in <a href=\"https://mastodon.social/tags/German\" class=\"mention hashtag\" rel=\"tag\">#<span>German</span></a>. Ideally:</p><p>› You have experience in <a href=\"https://mastodon.social/tags/accounting\" class=\"mention hashtag\" rel=\"tag\">#<span>accounting</span></a> + <a href=\"https://mastodon.social/tags/bookkeeping\" class=\"mention hashtag\" rel=\"tag\">#<span>bookkeeping</span></a><br />› Understand German <a href=\"https://mastodon.social/tags/legal\" class=\"mention hashtag\" rel=\"tag\">#<span>legal</span></a> frameworks + systems<br />› Are great with MS <a href=\"https://mastodon.social/tags/Excel\" class=\"mention hashtag\" rel=\"tag\">#<span>Excel</span></a>!</p><p>Could also work as a long-term paid <a href=\"https://mastodon.social/tags/internship\" class=\"mention hashtag\" rel=\"tag\">#<span>internship</span></a>. Can you refer anyone to us? More info/to apply:</p><p><a href=\"https://jobs.ashbyhq.com/mastodon/f38df483-da29-4bab-9f0c-5d1b11e7c1d0\" target=\"_blank\" rel=\"nofollow noopener noreferrer\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"ellipsis\">jobs.ashbyhq.com/mastodon/f38d</span><span class=\"invisible\">f483-da29-4bab-9f0c-5d1b11e7c1d0</span></a></p><p><a href=\"https://mastodon.social/tags/FediHire\" class=\"mention hashtag\" rel=\"tag\">#<span>FediHire</span></a> <a href=\"https://mastodon.social/tags/GetFediHired\" class=\"mention hashtag\" rel=\"tag\">#<span>GetFediHired</span></a> <a href=\"https://mastodon.social/tags/hiring\" class=\"mention hashtag\" rel=\"tag\">#<span>hiring</span></a></p>",
	"contentMap": {
		"en": "<p>We’re hiring again! The Mastodon team is looking for a part-time <a href=\"https://mastodon.social/tags/Finance\" class=\"mention hashtag\" rel=\"tag\">#<span>Finance</span></a> / <a href=\"https://mastodon.social/tags/Ops\" class=\"mention hashtag\" rel=\"tag\">#<span>Ops</span></a> Associate to support <span class=\"h-card\" translate=\"no\"><a href=\"https://mastodon.social/@mellifluousbox\" class=\"u-url mention\">@<span>mellifluousbox</span></a></span> + <span class=\"h-card\" translate=\"no\"><a href=\"https://mastodon.social/@Gargron\" class=\"u-url mention\">@<span>Gargron</span></a></span>.</p><p>This is a <a href=\"https://mastodon.social/tags/remote\" class=\"mention hashtag\" rel=\"tag\">#<span>remote</span></a> position and requires working proficiency in <a href=\"https://mastodon.social/tags/German\" class=\"mention hashtag\" rel=\"tag\">#<span>German</span></a>. Ideally:</p><p>› You have experience in <a href=\"https://mastodon.social/tags/accounting\" class=\"mention hashtag\" rel=\"tag\">#<span>accounting</span></a> + <a href=\"https://mastodon.social/tags/bookkeeping\" class=\"mention hashtag\" rel=\"tag\">#<span>bookkeeping</span></a><br />› Understand German <a href=\"https://mastodon.social/tags/legal\" class=\"mention hashtag\" rel=\"tag\">#<span>legal</span></a> frameworks + systems<br />› Are great with MS <a href=\"https://mastodon.social/tags/Excel\" class=\"mention hashtag\" rel=\"tag\">#<span>Excel</span></a>!</p><p>Could also work as a long-term paid <a href=\"https://mastodon.social/tags/internship\" class=\"mention hashtag\" rel=\"tag\">#<span>internship</span></a>. Can you refer anyone to us? More info/to apply:</p><p><a href=\"https://jobs.ashbyhq.com/mastodon/f38df483-da29-4bab-9f0c-5d1b11e7c1d0\" target=\"_blank\" rel=\"nofollow noopener noreferrer\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"ellipsis\">jobs.ashbyhq.com/mastodon/f38d</span><span class=\"invisible\">f483-da29-4bab-9f0c-5d1b11e7c1d0</span></a></p><p><a href=\"https://mastodon.social/tags/FediHire\" class=\"mention hashtag\" rel=\"tag\">#<span>FediHire</span></a> <a href=\"https://mastodon.social/tags/GetFediHired\" class=\"mention hashtag\" rel=\"tag\">#<span>GetFediHired</span></a> <a href=\"https://mastodon.social/tags/hiring\" class=\"mention hashtag\" rel=\"tag\">#<span>hiring</span></a></p>"
	},
	"attachment": [],
	"tag": [
		{
			"type": "Mention",
			"href": "https://mastodon.social/users/mellifluousbox",
			"name": "@mellifluousbox"
		},
		{
			"type": "Mention",
			"href": "https://mastodon.social/users/Gargron",
			"name": "@Gargron"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/finance",
			"name": "#finance"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/ops",
			"name": "#ops"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/remote",
			"name": "#remote"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/german",
			"name": "#german"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/accounting",
			"name": "#accounting"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/bookkeeping",
			"name": "#bookkeeping"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/legal",
			"name": "#legal"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/excel",
			"name": "#excel"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/internship",
			"name": "#internship"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/FediHire",
			"name": "#FediHire"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/getfedihired",
			"name": "#getfedihired"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/hiring",
			"name": "#hiring"
		}
	],
	"replies": {
		"id": "https://mastodon.social/users/Mastodon/statuses/112769333503182077/replies",
		"type": "Collection",
		"first": {
			"type": "CollectionPage",
			"next": "https://mastodon.social/users/Mastodon/statuses/112769333503182077/replies?only_other_accounts=true&page=true",
			"partOf": "https://mastodon.social/users/Mastodon/statuses/112769333503182077/replies",
			"items": []
		}
	}
}
        "##;
        let deserialized: Result<ActivityStream, serde_json::Error> =
            serde_json::from_str(test_note);
        let deserialized = match deserialized {
            Ok(x) => x,
            Err(x) => {
                return Err(format!(
                    "create activity deserialize failed with response: {}",
                    x
                ))
            }
        };

        let deserialized = match deserialized.content.activity_stream {
            crate::activitystream_objects::core_types::ExtendsObject::Object(x) => x,
            _ => return Err(format!("not of type object")),
        };

        if !matches!(deserialized.type_field, ObjectType::Note) {
            return Err(format!(
                "incorrect object type, givent type: {}",
                serde_json::to_string(&deserialized.type_field).unwrap()
            ));
        }

        // dbg!(&deserialized.object.replies);

        Ok(())
    }
}
