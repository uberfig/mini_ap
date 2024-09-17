use base64::Engine;
use serde::{de::Error as DeError, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

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
    #[serde(flatten)]
    pub key: AlgorithmsPublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "algorithm", content = "key")]
pub enum AlgorithmsPublicKey {
    #[serde(rename = "ed25519")]
    Ed25519(ed25519_dalek::VerifyingKey),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519Public {
    pub key: ed25519_dalek::VerifyingKey,
}

impl Serialize for Ed25519Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let binary = self.key.as_bytes();
        let base64_val = base64::prelude::BASE64_STANDARD.encode(binary);
        serializer.serialize_str(&base64_val)
    }
}

impl<'de> Deserialize<'de> for Ed25519Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = <&str>::deserialize(deserializer)?;
        let binary = match base64::prelude::BASE64_STANDARD.decode(input) {
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
        Ok(Ed25519Public { key })
    }
}
