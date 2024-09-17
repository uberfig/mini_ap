use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ParseErr {
    Failure,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyType {
    Ed25519,
}

pub trait PrivateKey: Clone {
    /// sign the provided content with this key
    fn sign(&self, content: &str) -> String;
    fn from_pem(pem: &str) -> Result<Self, ParseErr>;
    fn generate(algorithm: KeyType) -> Self;
    fn private_key_pem(&self) -> String;
    fn public_key_pem(&self) -> String;
}

pub trait PublicKey: Clone {
    /// verify that the provided content was signed with this key
    fn verify(&self, plain_content: &str, signature: &str) -> bool;
    fn from_pem(pem: &str, algorithm: KeyType) -> Result<Self, ParseErr>;
}
