use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    /// algorithm used for the public key. Can only be ed25519 for now.
    pub algorithm: String,
    /// Instance public key, in SPKI-encoded base64 (from raw bytes, not a PEM format).
    pub key: String,
}
