//---------------Activities--------------

use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{activitystream_objects::object, cache_and_fetch::Cache, db::conn::DbConn};

use super::{
    actors::RangeLinkActor,
    core_types::{RangeLinkExtendsObject, RangeLinkObject},
    object::Object,
};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// /// not used for deserialization, just for database storage
// pub enum AllTypes {
//     //intransitive types
//     Arrive, //not used
//     Travel, //not used
//     Question,
//     //normal activities
//     Accept,
//     TentativeAccept,
//     Add,
//     Create,
//     Delete,
//     Follow,
//     Ignore,
//     Join,
//     Leave,
//     Like,
//     Offer,
//     Invite,
//     Reject,
//     TentativeReject,
//     Remove,
//     Undo,
//     Update,
//     View,
//     Listen,
//     Read,
//     Move,
//     Announce,
//     Block,
//     Flag,
//     Dislike,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsIntransitive {
    ExtendsActivity(Activity),
    IntransitiveActivity(IntransitiveActivity),
    Question(Question),
}

impl ExtendsIntransitive {
    pub fn get_actor(&self) -> &Url {
        match self {
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.extends_object.id.id,
            ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.extends_object.id.id,
        }
    }
    pub fn get_id(&self) -> &Url {
        match self {
            ExtendsIntransitive::ExtendsActivity(x) => &x.extends_intransitive.extends_object.id.id,
            ExtendsIntransitive::IntransitiveActivity(x) => &x.extends_object.id.id,
            ExtendsIntransitive::Question(x) => &x.extends_intransitive.extends_object.id.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(rename = "type")]
    pub type_field: IntransitiveType,

    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: RangeLinkActor,      //TODO
    pub target: Option<String>,     //TODO
    pub result: Option<String>,     //TODO
    pub origin: Option<String>,     //TODO
    pub instrument: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionType {
    Question,
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
    pub one_of: Option<String>, //TODO
    pub any_of: Option<String>, //TODO
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
    pub async fn verify_attribution(&self, cache: &Cache, conn: &Data<DbConn>) -> Result<(), ()> {
        match self.type_field {
            ActivityType::Create => {
                let object = self.object.get_concrete(cache, conn).await;
                let object = match object {
                    Ok(x) => x,
                    Err(x) => {
                        dbg!(x);
                        return Err(());
                    }
                };
                let object = match object.get_object() {
                    Some(x) => x,
                    None => {
                        return Err(());
                    }
                };

                if let Some(x) = &object.attributed_to {
                    if self.extends_intransitive.actor.get_id() == x.get_id() {
                        return Ok(());
                    }
                };

                return Err(());
            }
            // ActivityType::Add |
            // ActivityType::Remove |
            ActivityType::Undo | ActivityType::Update | ActivityType::Delete => {
                let Some(actor_domain) = self.extends_intransitive.actor.get_id().domain() else {
                    return Err(());
                };
                let Some(obj_domain) = self.object.get_id().domain() else {
                    return Err(());
                };
                if actor_domain == obj_domain {
                    return Ok(());
                }
                return Err(());
            }
            _ => return Ok(()),
        };
    }
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
