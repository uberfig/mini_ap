use serde::{Deserialize, Serialize};
use url::Url;

use super::{actors::Actor, link::RangeLinkItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteType {
    Delete,
}

/// Indicates that the actor has deleted the object. If specified,
/// the origin indicates the context from which the object was deleted.
///
/// an example of a delete:
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally deleted a note",
///   "type": "Delete",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1",
///   "origin": {
///     "type": "Collection",
///     "name": "Sally's Notes"
///   }
/// }
/// ```
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete
///
/// The Delete activity is used to delete an already existing object.
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    #[serde(rename = "type")]
    pub type_field: DeleteType,
    pub actor: RangeLinkItem<Actor>, //TODO

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, //TODO

    pub object: Url,
}
