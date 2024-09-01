//---------------Activities--------------
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    actors::Actor,
    core_types::{ActivityStream, Context, ContextWrap, ExtendsObject},
    link::{LinkSimpleOrExpanded, RangeLinkItem},
    object::ObjectWrapper,
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
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.id,
            // ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.id,
        }
    }
    pub fn get_id(&self) -> &Url {
        match self {
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.id,
            // ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.id,
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
    pub id: Url,
    // #[serde(rename = "type")]
    // pub type_field: IntransitiveType,
    // #[serde(flatten)]
    // pub extends_object: Object,
    pub actor: RangeLinkItem<Actor>, //TODO
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

    ///gets id from intransitive
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,

    pub object: RangeLinkItem<ExtendsObject>,
}

impl Activity {
    pub fn new_create(object: ObjectWrapper) -> Self {
        let intransitive = IntransitiveActivity {
            id: Url::parse(&format!("{}/activity", object.object.id.as_str())).unwrap(),
            actor: RangeLinkItem::Link(LinkSimpleOrExpanded::Simple(
                object.object.get_attributed_to().clone(),
            )),
            target: None,
            result: None,
            origin: None,
            instrument: None,
        };
        Activity {
            type_field: ActivityType::Create,
            object: RangeLinkItem::Item(ExtendsObject::Object(Box::new(object))),
            extends_intransitive: intransitive,
        }
    }
    pub fn new_accept(actor: Url, object: Url, domain: &str) -> Self {
        let intransitive = IntransitiveActivity {
            id: Url::parse(&format!("https://{}/accept", domain)).unwrap(),
            actor: RangeLinkItem::Link(LinkSimpleOrExpanded::Simple(actor)),
            target: None,
            result: None,
            origin: None,
            instrument: None,
        };
        Activity {
            type_field: ActivityType::Accept,
            object: RangeLinkItem::Link(LinkSimpleOrExpanded::Simple(object)),
            extends_intransitive: intransitive,
        }
    }
    pub fn to_activitystream(self) -> ActivityStream {
        ActivityStream { content: ContextWrap {
            context: Context::Single("https://www.w3.org/ns/activitystreams".to_string()),
            activity_stream: ExtendsObject::ExtendsIntransitive(Box::new(ExtendsIntransitive::ExtendsActivity(self))),
        } }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    Activity,
    /// Indicates that the actor accepts the object. The target property
    /// can be used in certain circumstances to indicate the context into
    /// which the object has been accepted.
    ///
    /// example of an [`ActivityType::Follow`] accept
    ///
    /// ```json
    /// {
    ///   "@context": "https://www.w3.org/ns/activitystreams",
    ///   "summary": "sally accepts john's follow request",
    ///   "type": "Accept",
    ///   "actor": {
    ///     "type": "Person",
    ///     "name": "Sally"
    ///   },
    ///   "object": {
    ///     "type": "Follow",
    ///     "actor": "http://john.example.org",
    ///     "object": {
    ///       "id": "https://example.com",
    ///       "type": "Person",
    ///       "name": "Sally"
    ///     }
    ///   }
    /// }
    ///
    /// ```
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
    ///
    ///  The Delete activity is used to delete an already existing object.
    /// The side effect of this is that the server MAY replace the object
    /// with a [`super::object::ObjectType::Tombstone`] of the object that
    /// will be displayed in activities which reference the deleted object.
    /// If the deleted object is requested the server SHOULD respond with
    /// either the HTTP 410 Gone status code if a Tombstone object is presented
    /// as the response body, otherwise respond with a HTTP 404 Not Found.
    ///
    /// A deleted object:
    ///
    /// ```json
    /// {
    ///   "@context": "https://www.w3.org/ns/activitystreams",
    ///   "id": "https://example.com/~alice/note/72",
    ///   "type": "Tombstone",
    ///   "published": "2015-02-10T15:04:55Z",
    ///   "updated": "2015-02-10T15:04:55Z",
    ///   "deleted": "2015-02-10T15:04:55Z"
    /// }
    /// ```
    ///
    /// https://www.w3.org/TR/activitypub/#delete-activity-outbox
    Delete,
    /// Indicates that the actor is "following" the object. Following
    /// is defined in the sense typically used within Social systems in
    /// which the actor is interested in any activity performed by or on
    /// the object. The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
    ///
    /// The side effect of receiving this in an inbox is that the server
    /// SHOULD generate either an [`ActivityType::Accept`] or
    /// [`ActivityType::Reject`] activity with the Follow as the object
    /// and deliver it to the actor of the Follow.
    ///
    /// The Accept or Reject MAY be generated automatically, or MAY be the result of
    /// user input (possibly after some delay in which the user reviews).
    /// Servers MAY choose to not explicitly send a Reject in response to
    /// a Follow, this would typically be represented as pending
    ///
    /// https://www.w3.org/TR/activitypub/#follow-activity-inbox
    ///
    /// example from activitystreams:
    ///
    /// ```json
    /// {
    ///   "@context": "https://www.w3.org/ns/activitystreams",
    ///   "summary": "Sally followed John",
    ///   "type": "Follow",
    ///   "actor": {
    ///     "id": "https://example.com",
    ///     "type": "Person",
    ///     "name": "Sally"
    ///   },
    ///   "object": {
    ///     "id": "https://example.com",
    ///     "type": "Person",
    ///     "name": "John"
    ///   }
    /// }
    /// ```
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
    ///
    /// The Undo activity is used to undo a previous activity. See the
    /// Activity Vocabulary documentation on Inverse Activities and "Undo".
    /// For example, Undo may be used to undo a previous [`ActivityType::Like`],
    /// [`ActivityType::Follow`], or [`ActivityType::Block`]. The undo activity
    /// and the activity being undone MUST both have the same actor. Side effects
    /// should be undone, to the extent possible. For example, if undoing a Like,
    /// any counter that had been incremented previously should be decremented appropriately.
    ///
    /// There are some exceptions where there is an existing and explicit
    /// "inverse activity" which should be used instead.
    /// [`ActivityType::Create`] based activities should instead use
    /// [`ActivityType::Delete`], and [`ActivityType::Add`] activities
    /// should use [`ActivityType::Remove`].
    ///
    /// https://www.w3.org/TR/activitypub/#undo-activity-outbox
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#inverse
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
    fn deserialize_delete() -> Result<(), String> {
        let example = r##"
{
  "@context": "https://www.w3.org/ns/activitystreams",
  "id": "https://mastodon.social/users/Hibur#delete",
  "type": "Delete",
  "actor": "https://mastodon.social/users/Hibur",
  "to": [
    "https://www.w3.org/ns/activitystreams#Public"
  ],
  "object": "https://mastodon.social/users/Hibur",
  "signature": {
    "type": "RsaSignature2017",
    "creator": "https://mastodon.social/users/Hibur#main-key",
    "created": "2024-08-15T00:55:36Z",
    "signatureValue": "r9mo33vwMJND1gBqULuMQkwq2bXPGn8ZguiCDAASMNTBuJUjfch+pqx4KtibaEw5gRFrIRfCeQesOL+MzPJB2toMS1OOmuJjUcNibDJWb9EmYgQ+Mcmc5K+eVwviV7u/3t2v7LAwSNtLZVRzoo2R770p45TRRvZUxFWK//l3KcnfQMTqr19dap+6krRr6pzuI2UQC+htHvkIK2bqMh+ddtXUCCndVv01VQM01R+BKPvzP3iGXd6wTbGpXKLPeRWDDyLG2U3vjs/ixEHej4ycJXG2iljbxOZbaj6TjlAKpJBnkuy0ZTEf91CPpCytFRsqtCmb5KcmYdw2wlBLfVc0FQ=="
  }
}
        "##;

        let deserialized: Result<ActivityStream, serde_json::Error> = serde_json::from_str(example);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!(
                "Delete activity deserialize failed with response: {}",
                x
            )),
        }
    }

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
