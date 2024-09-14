use serde::{Deserialize, Serialize};
use url::Url;

use crate::versia_types::entities::EntityArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    /// Author of the collection. Usually the user who owns the collection. 
    /// Can be set to null to represent the instance.
    pub author: Option<Url>,
    /// URI to the last page of the collection. Query parameters are allowed.
    pub first: Url,
    /// URI to the last page of the collection. Query parameters are allowed.
    /// 
    /// If the collection only has one page, this should be the same as first
    pub last: Url,
    pub total: u64,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub items: EntityArray,
}

