//---------------Activities--------------
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    actors::RangeLinkActor,
    core_types::{ExtendsObject, RangeLinkExtendsObject},
    object::{Object, ObjectWrapper},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsIntransitive {
    ExtendsActivity(Activity),
    // IntransitiveActivity(IntransitiveActivity),
    Question(Question),
}

impl ExtendsIntransitive {
    pub fn get_actor(&self) -> &Url {
        match self {
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.extends_object.id.id,
            // ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.extends_object.id.id,
        }
    }
    pub fn get_id(&self) -> &Url {
        match self {
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.extends_object.id.id,
            // ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.extends_object.id.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// we are not using these for this project
pub enum IntransitiveType {
    IntransitiveActivity,
    /// An [`IntransitiveActivity`] that indicates that the actor has
    /// arrived at the location. The origin can be used to identify the
    /// context from which the actor originated. The target typically
    /// has no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive
    Arrive,
    /// Indicates that the actor is traveling to target from origin.
    /// Travel is an IntransitiveObject whose actor specifies the direct object.
    /// If the target or origin are not specified, either can be determined by context.  
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel
    Travel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntransitiveActivity {
    // #[serde(rename = "type")]
    // pub type_field: IntransitiveType,
    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: RangeLinkActor, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionType {
    Question,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceType {
    AnyOf(Vec<QuestionOption>),
    OneOf(Vec<QuestionOption>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionOptionType {
    Note,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: QuestionOptionType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Represents a question being asked.
/// Question objects are an extension of [`IntransitiveActivity`]. That is,
/// the Question object is an Activity, but the direct object is the question
/// itself and therefore it would not contain an object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers,
/// but a Question object MUST NOT have both properties.
///
/// Commonly used for polls
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question
pub struct Question {
    #[serde(rename = "type")]
    pub type_field: QuestionType,
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
    #[serde(flatten)]
    pub options: ChoiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// indicates that a poll can only be voted on by local users
    pub local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "type")]
    pub type_field: ActivityType,

    pub object: RangeLinkExtendsObject,

    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
}

impl Activity {
    pub fn new_create(object: ObjectWrapper) -> Self {
        let intransitive = IntransitiveActivity {
            extends_object: Object::new(
                Url::parse(&format!("{}/activity", object.object.id.id.as_str())).unwrap(),
            ),
            actor: RangeLinkActor::Link(
                object
                    .object
                    .get_attributed_to()
                    .expect("trying to make a create for an object without attribution")
                    .clone(),
            ),
            target: None,
            result: None,
            origin: None,
            instrument: None,
        };
        Activity {
            type_field: ActivityType::Create,
            object: RangeLinkExtendsObject::Object(ExtendsObject::Object(Box::new(object))),
            extends_intransitive: intransitive,
        }
    }
    // pub async fn verify_attribution(&self, cache: &Cache, conn: &Data<DbConn>) -> Result<(), ()> {
    //     match self.type_field {
    //         ActivityType::Create => {
    //             let object = self.object.get_concrete(cache, conn).await;
    //             let object = match object {
    //                 Ok(x) => x,
    //                 Err(x) => {
    //                     dbg!(x);
    //                     return Err(());
    //                 }
    //             };
    //             let object = match object.get_as_object() {
    //                 Some(x) => x,
    //                 None => {
    //                     return Err(());
    //                 }
    //             };

    //             if let Some(x) = &object.attributed_to {
    //                 if self.extends_intransitive.actor.get_id() == x.get_id() {
    //                     return Ok(());
    //                 }
    //             };

    //             return Err(());
    //         }
    //         // ActivityType::Add |
    //         // ActivityType::Remove |
    //         ActivityType::Undo | ActivityType::Update | ActivityType::Delete => {
    //             let Some(actor_domain) = self.extends_intransitive.actor.get_id().domain() else {
    //                 return Err(());
    //             };
    //             let Some(obj_domain) = self.object.get_id().domain() else {
    //                 return Err(());
    //             };
    //             if actor_domain == obj_domain {
    //                 return Ok(());
    //             }
    //             return Err(());
    //         }
    //         _ => return Ok(()),
    //     };
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    Activity,
    /// Indicates that the actor accepts the object. The target property
    /// can be used in certain circumstances to indicate the context into
    /// which the object has been accepted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept
    Accept,
    /// A specialization of [`Accept`] indicating that the acceptance is tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept
    TentativeAccept,
    /// Indicates that the actor has added the object to the target.
    /// If the target property is not explicitly specified, the target
    /// would need to be determined implicitly by context. The origin
    /// can be used to identify the context from which the object originated.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add
    Add,
    /// Indicates that the actor has created the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
    Create,
    /// Indicates that the actor has deleted the object. If specified,
    /// the origin indicates the context from which the object was deleted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete
    Delete,
    /// Indicates that the actor is "following" the object. Following
    /// is defined in the sense typically used within Social systems in
    /// which the actor is interested in any activity performed by or on
    /// the object. The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
    Follow,
    /// Indicates that the actor is ignoring the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore
    Ignore,
    /// Indicates that the actor has joined the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join
    Join,
    /// Indicates that the actor has left the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave
    Leave,
    /// Indicates that the actor likes, recommends or endorses the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like
    Like,
    /// Indicates that the actor is offering the object.
    /// If specified, the target indicates the entity to which the object is being offered.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer
    Offer,
    /// A specialization of [`Offer`] in which the actor is extending an invitation for the object to the target.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite
    Invite,
    /// Indicates that the actor is rejecting the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject
    Reject,
    /// A specialization of [`Reject`] in which the rejection is considered tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject
    TentativeReject,
    /// Indicates that the actor is removing the object.
    /// If specified, the origin indicates the context from which the object is being removed.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove
    Remove,
    /// Indicates that the actor is undoing the object. In most cases,
    /// the object will be an [`Activity`] describing some previously performed action (for instance,
    /// a person may have previously "liked" an article but, for whatever reason,
    /// might choose to undo that like at some later point in time).
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo
    Undo,
    /// Indicates that the actor has updated the object.
    /// Note, however, that this vocabulary does not define a mechanism for
    /// describing the actual set of modifications made to object.
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update
    Update,
    /// Indicates that the actor has viewed the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view
    View,
    /// Indicates that the actor has listened to the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen
    Listen,
    /// Indicates that the actor has read the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read
    Read,
    /// Indicates that the actor has moved object from origin to target.
    /// If the origin or target are not specified, either can be determined by context.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move
    Move,
    /// Indicates that the actor is calling the target's attention the object.
    /// The origin typically has no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
    Announce,
    /// Indicates that the actor is blocking the object.
    /// Blocking is a stronger form of [`Ignore`].
    /// The typical use is to support social systems that allow one user
    /// to block activities or content of other users.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block
    Block,
    /// Indicates that the actor is "flagging" the object.
    /// Flagging is defined in the sense common to many social platforms
    /// as reporting content as being inappropriate for any number of reasons.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag
    Flag,
    /// Indicates that the actor dislikes the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike
    Dislike,
}

#[cfg(test)]
mod tests {
    use crate::activitystream_objects::core_types::ActivityStream;

    #[test]
    fn test_deserialize_create() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon/statuses/112769333503182077/activity
        let test_create = r##"
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
	"id": "https://mastodon.social/users/Mastodon/statuses/112769333503182077/activity",
	"type": "Create",
	"actor": "https://mastodon.social/users/Mastodon",
	"published": "2024-07-11T18:44:32Z",
	"to": [
		"https://www.w3.org/ns/activitystreams#Public"
	],
	"cc": [
		"https://mastodon.social/users/Mastodon/followers",
		"https://mastodon.social/users/mellifluousbox",
		"https://mastodon.social/users/Gargron"
	],
	"object": {
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
}
        "##;
        let deserialized: Result<ActivityStream, serde_json::Error> =
            serde_json::from_str(test_create);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!(
                "create activity deserialize failed with response: {}",
                x
            )),
        }
    }
}
