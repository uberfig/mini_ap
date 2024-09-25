use serde::{Deserialize, Serialize};
use url::Url;

use crate::versia_types::serde_fns::{deserialize_time, serialize_time};

use super::{
    actors::Actor,
    collections::ExtendsCollection,
    core_types::OptionalArray,
    link::{LinkSimpleOrExpanded, RangeLinkItem},
};

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

    Profile, // adds describes
    /// A Tombstone represents a content object that has
    /// been deleted. It can be used in Collections to
    /// signify that there used to be an object at this
    /// position, but it has been deleted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone
    Tombstone, // adds formerType | deleted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "type")]
    pub type_field: ObjectType,
    pub id: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub attributed_to: RangeLinkItem<Actor>,
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
    pub in_reply_to: Option<Url>,

    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub published: i64,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<OptionalArray<LinkSimpleOrExpanded>>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<ExtendsCollection>,
}

#[cfg(test)]
mod tests {
    use crate::activitystream_objects::{context::ContextWrap, object::ObjectType};

    use super::Object;

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
        let deserialized: Result<ContextWrap<Object>, serde_json::Error> =
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

        if !matches!(deserialized.item.type_field, ObjectType::Note) {
            return Err(format!(
                "incorrect object type, givent type: {}",
                serde_json::to_string(&deserialized.item.type_field).unwrap()
            ));
        }

        // dbg!(&deserialized.object.replies);

        Ok(())
    }
}
