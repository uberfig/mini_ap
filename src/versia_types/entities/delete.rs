use crate::versia_types::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteType {
    Delete,
}

/// Signals the deletion of an entity.
///
/// Implementations must ensure that the author of the Delete entity has the authorization to delete the target entity.
/// - The author is the creator of the target entity (including [delegation](https://versia.pub/federation/delegation)).
/// - The author is the instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "type")]
    pub type_field: DeleteType,
    pub id: String,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    /// URI of the User who is deleting the entity. Can be set to null to represent the instance.
    ///
    /// https://versia.pub/entities/instance-metadata#the-null-author
    pub author: Option<Url>,
    pub deleted_type: String,
    pub target: Url,
}
