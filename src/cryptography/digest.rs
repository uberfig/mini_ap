use openssl::hash::MessageDigest;

/// generates an sha256 digest of the provided buffer encoded in base64
pub fn sha256_hash(body: &[u8]) -> String {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha256()).unwrap();
    hasher.update(body).unwrap();
    let digest: &[u8] = &hasher.finish().unwrap();

    //digest_base64
    openssl::base64::encode_block(digest)
}
