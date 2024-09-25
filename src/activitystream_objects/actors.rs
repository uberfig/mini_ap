use std::collections::HashMap;
use base64::Engine;
use serde::{de::Error as DeError, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    cryptography::{key::KeyType, private_key::AlgorithmsPrivateKey},
    versia_types::entities::public_key::AlgorithmsPublicKey,
};

use super::{core_types::*, link::RangeLinkItem};

impl RangeLinkItem<Actor> {
    pub fn get_id(&self) -> &Url {
        match self {
            RangeLinkItem::Item(x) => x.get_id(),
            RangeLinkItem::Link(x) => x.get_id(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorType {
    // Actor,
    /// Describes a software application.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application
    Application,
    /// Represents a formal or informal collective of Actors.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group
    Group,
    /// Represents an organization.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization
    Organization,
    /// Represents an individual person. The most
    /// common type of actor on the fedi
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person
    Person,
    /// Represents a service of any kind.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service
    Service,
}

//-------------------types--------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: Url,    //https://my-example.com/actor#main-key
    pub owner: Url, //"https://my-example.com/actor"
    pub public_key_pem: String,
}
impl From<String> for PublicKey {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Actor types are [`Object`] types that are capable of performing activities
///
/// core types:
/// - [`ActorType::Application`]
/// - [`ActorType::Group`]
/// - [`ActorType::Organization`]
/// - [`ActorType::Person`]
/// - [`ActorType::Service`]
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub id: Url,
    pub preferred_username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    pub public_key: PublicKey,

    pub inbox: Url,
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,

    // #[serde(skip)]
    // pub ap_user_id: Option<i64>,
    #[serde(skip)]
    pub domain: Option<String>,
    #[serde(skip)]
    pub liked: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApPublicKey {
	
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Context {
    Array(Vec<ContextItem>),
    Single(Url),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextItem {
    String(String),
    Map(HashMap<String, ContextMapItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextMapItem {
    String(String),
    Map(HashMap<String, String>),
}

impl Actor {
    // pub fn to_activitystream(self) -> ActivityStream {
    //     // let mut test: HashMap<String, ContextMapItem> = HashMap::new();
    //     // let mut item: HashMap<String, String> = HashMap::new();
    //     // item.insert("@id".to_string(), "toot:featuredTags".to_string());
    //     // item.insert("@type".to_string(), "@id".to_string());
    //     // test.insert("featuredTags".to_string(), ContextMapItem::Map(item));
    //     // test.insert("manuallyApprovesFollowers".to_string(), ContextMapItem::String("as:manuallyApprovesFollowers".to_string()));
    //     ActivityStream {
    //         content: ContextWrap {
    //             context: Context::Array(vec![
    //                 ContextItem::String("https://www.w3.org/ns/activitystreams".to_owned()),
    //                 ContextItem::String("https://w3id.org/security/v1".to_owned()),
    //                 // ContextItem::Map(test)
    //             ]),
    //             activity_stream: ExtendsObject::Actor(Box::new(self)),
    //         },
    //     }
    // }
    pub fn get_id(&self) -> &Url {
        &self.id
    }
	pub fn get_key_type(&self) -> Option<KeyType> {
		let input = match &self.context {
			Context::Array(vec) => vec,
			Context::Single(_) => return None,
		};
		for i in input.iter() {
			if let ContextItem::Map(map) = i {
				let val = map.get("Ed25519Key");
				if val.is_some() {
					return Some(KeyType::Ed25519);
				}
			}
		}
		None
	}
    pub fn get_public_key(&self) -> Result<AlgorithmsPublicKey, ()> {
		use crate::cryptography::key::PublicKey;
		let Some(keytype) = self.get_key_type() else {
			return Err(());
		};
		match keytype {
			KeyType::Ed25519 => {
				match AlgorithmsPublicKey::from_pem(&self.public_key.public_key_pem, KeyType::Ed25519) {
						Ok(x) => Ok(x),
						Err(_) => Err(()),
					}
			},
		}
    }
}

// impl From<Actor> for ActivityStream {
//     fn from(value: Actor) -> ActivityStream {
//         value.to_activitystream()
//     }
// }

// impl From<Box<Actor>> for ActivityStream {
//     fn from(value: Box<Actor>) -> ActivityStream {
//         ActivityStream {
//             content: ContextWrap {
//                 context: Context::Array(vec![
//                     ContextItem::String("https://www.w3.org/ns/activitystreams".to_owned()),
//                     ContextItem::String("https://w3id.org/security/v1".to_owned()),
//                 ]),
//                 // activity_stream: RangeLinkExtendsObject::Object(ExtendsObject::Actor(value)),
//                 activity_stream: ExtendsObject::Actor(value),
//             },
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon
        let mastodon_account = r#"
{
	"@context": [
		"https://www.w3.org/ns/activitystreams",
		"https://w3id.org/security/v1",
		{
			"manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
			"toot": "http://joinmastodon.org/ns#",
			"featured": {
				"@id": "toot:featured",
				"@type": "@id"
			},
			"featuredTags": {
				"@id": "toot:featuredTags",
				"@type": "@id"
			},
			"alsoKnownAs": {
				"@id": "as:alsoKnownAs",
				"@type": "@id"
			},
			"movedTo": {
				"@id": "as:movedTo",
				"@type": "@id"
			},
			"schema": "http://schema.org#",
			"PropertyValue": "schema:PropertyValue",
			"value": "schema:value",
			"discoverable": "toot:discoverable",
			"Device": "toot:Device",
			"Ed25519Signature": "toot:Ed25519Signature",
			"Ed25519Key": "toot:Ed25519Key",
			"Curve25519Key": "toot:Curve25519Key",
			"EncryptedMessage": "toot:EncryptedMessage",
			"publicKeyBase64": "toot:publicKeyBase64",
			"deviceId": "toot:deviceId",
			"claim": {
				"@type": "@id",
				"@id": "toot:claim"
			},
			"fingerprintKey": {
				"@type": "@id",
				"@id": "toot:fingerprintKey"
			},
			"identityKey": {
				"@type": "@id",
				"@id": "toot:identityKey"
			},
			"devices": {
				"@type": "@id",
				"@id": "toot:devices"
			},
			"messageFranking": "toot:messageFranking",
			"messageType": "toot:messageType",
			"cipherText": "toot:cipherText",
			"suspended": "toot:suspended",
			"memorial": "toot:memorial",
			"indexable": "toot:indexable",
			"focalPoint": {
				"@container": "@list",
				"@id": "toot:focalPoint"
			}
		}
	],
	"id": "https://mastodon.social/users/Mastodon",
	"type": "Person",
	"following": "https://mastodon.social/users/Mastodon/following",
	"followers": "https://mastodon.social/users/Mastodon/followers",
	"inbox": "https://mastodon.social/users/Mastodon/inbox",
	"outbox": "https://mastodon.social/users/Mastodon/outbox",
	"featured": "https://mastodon.social/users/Mastodon/collections/featured",
	"featuredTags": "https://mastodon.social/users/Mastodon/collections/tags",
	"preferredUsername": "Mastodon",
	"name": "Mastodon",
	"summary": "<p>Free, open-source decentralized social media platform.</p>",
	"url": "https://mastodon.social/@Mastodon",
	"manuallyApprovesFollowers": false,
	"discoverable": true,
	"indexable": false,
	"published": "2016-11-23T00:00:00Z",
	"memorial": false,
	"devices": "https://mastodon.social/users/Mastodon/collections/devices",
	"publicKey": {
		"id": "https://mastodon.social/users/Mastodon#main-key",
		"owner": "https://mastodon.social/users/Mastodon",
		"publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtpNfuGPl/WTnSq3dTurF\nMRelAIdvGVkO/VKYZJvIleYA27/YTnpmlY2g+0az4xEhOBtVNA1cTpS63CdXRyNz\ncH/GZtzxkdxN91vZSw0JVy+wG34dzwcq1KWFDz9D/5Tqf16KUJH+TDTlxdOBds91\nIZg+TTkiT+xfnSiC5SLMnn1dTzCW9P0yNJxpn37z7p6pEs63X1wstEEX1qGDUQTO\n1JICpKDjuQZMlioAAA5KG25tg2f+zKlv5M/NI33DblquyJ7TYvIpDN8hsFCRjuvA\nmjtKz/1XIRvQkeKND3UkqX8s6qTGyNOjcT86qt9BqYHYGuppjpRG/QNGoKYalio1\nwwIDAQAB\n-----END PUBLIC KEY-----\n"
	},
	"tag": [],
	"attachment": [
		{
			"type": "PropertyValue",
			"name": "Homepage",
			"value": "<a href=\"https://joinmastodon.org\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">joinmastodon.org</span><span class=\"invisible\"></span></a>"
		},
		{
			"type": "PropertyValue",
			"name": "Patreon",
			"value": "<a href=\"https://patreon.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span></a>"
		},
		{
			"type": "PropertyValue",
			"name": "GitHub",
			"value": "<a href=\"https://github.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">github.com/mastodon</span><span class=\"invisible\"></span></a>"
		}
	],
	"endpoints": {
		"sharedInbox": "https://mastodon.social/inbox"
	},
	"icon": {
		"type": "Image",
		"mediaType": "image/png",
		"url": "https://files.mastodon.social/accounts/avatars/000/013/179/original/b4ceb19c9c54ec7e.png"
	},
	"image": {
		"type": "Image",
		"mediaType": "image/png",
		"url": "https://files.mastodon.social/accounts/headers/000/013/179/original/1375be116fbe0f1d.png"
	}
}
        "#;
        let deserialized: Result<Actor, serde_json::Error> = serde_json::from_str(mastodon_account);

        match deserialized {
            Ok(x) => {
                dbg!(x);
                // Ok(())
                Err("hi".to_string())
            }
            Err(x) => Err(format!("actor deserialize failed with response: {}", x)),
        }
    }
}
