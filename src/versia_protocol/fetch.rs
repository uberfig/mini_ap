use ed25519::signature::SignerMut;
use serde::Deserialize;

enum HttpMethod {
    Get,
    Post,
}
impl HttpMethod {
    fn stringify(&self) -> &str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Post => "post",
        }
    }
}

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use textnonce::TextNonce;
use url::Url;

use crate::cryptography::digest;

fn generate_signing_key() -> ed25519_dalek::SigningKey {
    let mut csprng = OsRng;
    SigningKey::generate(&mut csprng)
}

fn signature_string(method: HttpMethod, path: &str, nonce: &str, hash: &str) -> String {
    format!("{} {} {} {}", method.stringify(), path, nonce, hash)
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

pub async fn versia_fetch<T: for<'a> Deserialize<'a>>(target: Url) -> T {
    // generate a signing key
    let key = generate_signing_key();
    // use signing key

    let client = reqwest::Client::new();
    let client = client
        .get(target)
        .header("Accept", "application/json")
        .header("X-Signature", "application/json")
        .header("X-Signed-By", "application/json")
        .header("X-Nonce", "application/json")
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .body("");

    // dbg!(&client);

    let res = client.send().await;
    // dbg!(&res);

    // let res = match res {
    //     Ok(x) => x,
    //     Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    // };

    todo!()
}

/// if signed by the instance, use instance for the signed by header
pub async fn versia_post(target: Url, content: &str, signed_by: &str) -> Result<(), ()> {
    // generate a signing key
    let nonce = TextNonce::new().into_string();
    let mut key = generate_signing_key();
    let path = target.path();
    let hash = digest::sha256_hash(content.as_bytes());
    // use signing key
    let signature_string = signature_string(HttpMethod::Post, path, &nonce, &hash);
    let signature = key.sign(signature_string.as_bytes()).to_string();

    let client = reqwest::Client::new();
    let client = client
        .post(target)
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by)
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .body("");

    // dbg!(&client);

    let res = client.send().await;
    // dbg!(&res);

    // let res = match res {
    //     Ok(x) => x,
    //     Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    // };

    todo!()
}
