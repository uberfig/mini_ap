use serde::{Deserialize, Serialize};
use url::Url;

use super::{actors::Actor, link::RangeLinkItem, question::Question};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateType {
    Create,
}

/// Indicates that the actor has created the object.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally created a note",
///   "type": "Create",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Note",
///     "name": "A Simple Note",
///     "content": "This is a simple note"
///   }
/// }
/// ```
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    #[serde(rename = "type")]
    pub type_field: CreateType,
    pub id: Url,
    pub actor: RangeLinkItem<Actor>,
    pub object: RangeLinkItem<Question>,
}
