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

#[cfg(test)]
mod tests {
    use crate::activitystream_objects::core_types::ActivityStream;

    #[test]
    fn test_deserialize_index() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon/followers
        let index_page = r#"
{
	"@context": "https://www.w3.org/ns/activitystreams",
	"id": "https://mastodon.social/users/Mastodon/followers",
	"type": "OrderedCollection",
	"totalItems": 819527,
	"first": "https://mastodon.social/users/Mastodon/followers?page=1"
}
        "#;
        let deserialized: Result<ActivityStream, serde_json::Error> =
            serde_json::from_str(&index_page);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!(
                "collection index deserialize failed with response: {}",
                x
            )),
        }
    }
    #[test]
    fn test_deserialize_first_page() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon/followers
        let index_page = r#"
{
	"@context": "https://www.w3.org/ns/activitystreams",
	"id": "https://mastodon.social/users/Mastodon/followers?page=1",
	"type": "OrderedCollectionPage",
	"totalItems": 819527,
	"next": "https://mastodon.social/users/Mastodon/followers?page=2",
	"partOf": "https://mastodon.social/users/Mastodon/followers",
	"orderedItems": [
		"https://mastodon.social/users/paranoiaAgent",
		"https://social.treehouse.systems/users/sertonix",
		"https://mastodon.social/users/hmkm",
		"https://mastodon.0819870.xyz/users/ns",
		"https://mastodon.social/users/doc0",
		"https://mastodon.social/users/sadness42",
		"https://mastodon.social/users/SWAG_INSTINCT",
		"https://mastodon.social/users/QUWALENTNOST",
		"https://uwu.social/users/katakislives",
		"https://mastodon.social/users/James__Unsy",
		"https://mastodon.social/users/Naji7664",
		"https://mastodon.social/users/phoenixse"
	]
}
        "#;
        let deserialized: Result<ActivityStream, serde_json::Error> =
            serde_json::from_str(&index_page);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!(
                "collection index deserialize failed with response: {}",
                x
            )),
        }
    }
}
