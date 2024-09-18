use ed25519::pkcs8::{spki::der::pem::LineEnding, EncodePublicKey, EncodePrivateKey};
use ed25519_dalek::SigningKey;

use rand::rngs::OsRng;

// ed25519_dalek::pkcs8::spki::der::EncodePem;
use super::key::{KeyType, PrivateKey};

#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmsPrivateKey {
    Ed25519(ed25519_dalek::SigningKey),
}

impl PrivateKey for AlgorithmsPrivateKey {
    fn sign(&self, content: &str) -> String {
        todo!()
    }

    fn from_pem(pem: &str) -> Result<Self, super::key::ParseErr> {
        todo!()
    }

    fn generate(algorithm: KeyType) -> Self {
        match algorithm {
            KeyType::Ed25519 => {
                let mut csprng = OsRng;
                AlgorithmsPrivateKey::Ed25519(SigningKey::generate(&mut csprng))
            }
        }
    }

    fn private_key_pem(&self) -> String {
        match self {
            AlgorithmsPrivateKey::Ed25519(signing_key) => {
                let test = signing_key.to_pkcs8_pem(LineEnding::default());
                test.expect("for some reason the private key failed to encode").to_string()
            }
        }
    }

    fn public_key_pem(&self) -> String {
        match self {
            AlgorithmsPrivateKey::Ed25519(signing_key) => {
                let val = signing_key.verifying_key();
                let test = val.to_public_key_pem(LineEnding::default());
                test.expect("for some reason the verifying key failed to encode as a public key pem. please send this along to the bayou project for further investigation")
            }
        }
    }
}
