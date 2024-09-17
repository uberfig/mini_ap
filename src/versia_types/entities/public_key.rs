use base64::Engine;
use serde::ser::SerializeStruct;
use serde::{de::Error as DeError, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmsPublicKey {
    Ed25519(ed25519_dalek::VerifyingKey),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "algorithm", content = "key")]
pub enum DeserializePublicKey {
    #[serde(rename = "ed25519")]
    Ed25519(String),
}

pub struct SerializePublicKey {
    pub algorithm: String,
    pub key: String,
}

/// The user's public key. Must follow the Versia Public Key format.
/// actor may be a URI to another user's profile, in which case this
/// key may allow the other user act on behalf of this user (see delegation).
/// - algorithm: Must be ed25519 for now.
/// - key: The public key in SPKI-encoded base64 (from raw bytes, not a PEM format). Must be the key associated with the actor URI.
/// - actor: URI to a user's profile, most often the user's own profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    pub actor: Option<Url>,
    /// algorithm used for the public key. Can only be ed25519 for now.
    ///
    /// public key, in SPKI-encoded base64 (from raw bytes, not a PEM format).
    #[serde(deserialize_with = "deserialize_key")]
    #[serde(serialize_with = "serialize_key")]
    #[serde(flatten)]
    pub key: AlgorithmsPublicKey,
}

pub fn deserialize_key<'de, D>(deserializer: D) -> Result<AlgorithmsPublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let input = DeserializePublicKey::deserialize(deserializer)?;

    match input {
        DeserializePublicKey::Ed25519(x) => {
            let binary = match base64::prelude::BASE64_STANDARD.decode(x) {
                Ok(ok) => ok,
                Err(err) => return Err(D::Error::custom(err)),
            };
            let Ok(binary) = binary.try_into() else {
                return Err(D::Error::custom("invalid binary length"));
            };
            let key = ed25519_dalek::VerifyingKey::from_bytes(&binary);
            let key = match key {
                Ok(ok) => ok,
                Err(err) => return Err(D::Error::custom(err)),
            };
            Ok(AlgorithmsPublicKey::Ed25519(key))
        }
    }
}

pub fn serialize_key<S>(x: &AlgorithmsPublicKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        AlgorithmsPublicKey::Ed25519(verifying_key) => {
            let binary = verifying_key.as_bytes();
            let base64_val = base64::prelude::BASE64_STANDARD.encode(binary);

            let mut test = s.serialize_struct("SerializePublicKey", 2)?;
            test.serialize_field("algorithm", "ed25519")?;
            test.serialize_field("key", &base64_val)?;
            test.end()
        }
    }
}
