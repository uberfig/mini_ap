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
    pub algorithm: String,
    /// Instance public key, in SPKI-encoded base64 (from raw bytes, not a PEM format).
    pub key: String,
}
