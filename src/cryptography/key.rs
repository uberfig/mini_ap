#[derive(Debug, Clone)]
pub enum ParseErr {
    Failure,
}

pub trait PrivateKey: Clone {
    /// sign the provided content with this key
    fn sign(&self, content: &str) -> String;
    fn from_pem(pem: &str) -> Result<Self, ParseErr>;
    fn generate() -> Self;
    fn private_key_pem(&self) -> &str;
    fn public_key_pem(&self) -> &str;
}

pub trait PublicKey: Clone {
    /// verify that the provided content was signed with this key
    fn verify(&self, plain_content: &str, signature: &str) -> bool;
    fn from_pem(pem: &str) -> Result<Self, ParseErr>;
}
