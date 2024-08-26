use openssl::hash::MessageDigest;

pub fn generate_digest(body: &[u8]) -> String {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha256()).unwrap();
    hasher.update(body).unwrap();
    let digest: &[u8] = &hasher.finish().unwrap();

    //digest_base64
    openssl::base64::encode_block(digest)
}
