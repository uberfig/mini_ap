use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};

use super::key::{KeyType, ParseErr, PrivateKey, PublicKey};

#[derive(Debug, Clone)]
pub struct OpenSSLPrivate {
    pub key: PKey<Private>,
    private_key_pem: String,
    public_key_pem: String,
}

impl PrivateKey for OpenSSLPrivate {
    fn sign(&self, content: &str) -> String {
        let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), &self.key).unwrap();
        signer.update(content.as_bytes()).unwrap();
        openssl::base64::encode_block(&signer.sign_to_vec().unwrap())
    }

    fn from_pem(pem: &str) -> Result<Self, ParseErr> {
        let Ok(key) = openssl::rsa::Rsa::private_key_from_pem(pem.as_bytes()) else {
            return Err(ParseErr::Failure);
        };
        let private_key_pem = pem.to_string();
        let public_key_pem = String::from_utf8(key.public_key_to_pem().unwrap()).unwrap();
        Ok(OpenSSLPrivate {
            key: PKey::from_rsa(key).unwrap(),
            private_key_pem,
            public_key_pem,
        })
    }

    fn generate(algorithm: KeyType) -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        let private_key_pem = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
        OpenSSLPrivate {
            key: PKey::from_rsa(rsa).unwrap(),
            private_key_pem,
            public_key_pem,
        }
    }

    fn private_key_pem(&self) -> String {
        self.private_key_pem.to_string()
    }

    fn public_key_pem(&self) -> String {
        self.public_key_pem.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct OpenSSLPublic {
    pub key: PKey<Public>,
}

impl PublicKey for OpenSSLPublic {
    fn verify(&self, plain_content: &str, signature: &str) -> bool {
        let mut verifier =
            openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &self.key)
                .unwrap();
        let input = &plain_content;
        verifier.update(input.as_bytes()).unwrap();

        let signature = openssl::base64::decode_block(signature).unwrap();
        verifier.verify(&signature).unwrap()
    }

    fn from_pem(pem: &str, algorithm: KeyType) -> Result<Self, ParseErr> {
        let Ok(key) = openssl::rsa::Rsa::public_key_from_pem(pem.as_bytes()) else {
            return Err(ParseErr::Failure);
        };
        Ok(OpenSSLPublic {
            key: openssl::pkey::PKey::from_rsa(key).unwrap(),
        })
    }
}
