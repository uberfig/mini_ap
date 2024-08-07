// --------------collections----------------

use serde::{Deserialize, Serialize};

use super::object::Object;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsCollection {
    Collection(Collection),
    CollectionPage(CollectionPage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CollectionType {
    Collection,
    OrderedCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "type")]
    pub type_field: CollectionType,
    #[serde(flatten)]
    pub extends_object: Object,
    pub total_items: u32,
    pub current: Option<String>, //TODO
    pub first: Option<String>,   //TODO
    pub last: Option<String>,    //TODO
    pub items: Option<String>,   //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PageType {
    CollectionPage,
    OrderedCollectionPage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(rename = "type")]
    pub type_field: PageType,

    #[serde(flatten)]
    pub extends_collection: Collection,
    pub part_of: Option<String>, //TODO
    pub next: Option<String>,    //TODO
    pub prev: Option<String>,    //TODO
}
