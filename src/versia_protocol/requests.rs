use serde::Deserialize;
use textnonce::TextNonce;
use url::Url;

use crate::{
    ap_protocol::fetch::FetchErr,
    cryptography::{digest, key::PrivateKey},
};

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

fn signature_string(method: HttpMethod, path: &str, nonce: &str, hash: &str) -> String {
    format!("{} {} {} {}", method.stringify(), path, nonce, hash)
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

pub async fn versia_fetch<T: for<'a> Deserialize<'a>, K: PrivateKey>(
    target: Url,
    signing_key: K,
    signed_by: &str,
) -> Result<T, FetchErr> {
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash("".as_bytes());
    let signature_string = signature_string(HttpMethod::Get, path, &nonce, &hash);
    let signature = signing_key.sign(&signature_string).to_string();

    let client = reqwest::Client::new();
    let client = client
        .get(target.clone())
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by)
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .body("");

    let res = client.send().await;

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let response = res.text().await;
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    if response.eq(r#"{"error":"Gone"}"#) {
        return Err(FetchErr::IsTombstone(target.to_string()));
    }

    let object: Result<T, serde_json::Error> = serde_json::from_str(&response);
    let object = match object {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::DeserializationErr(x.to_string())),
    };

    Ok(object)
}

/// if signed by the instance, use instance for the signed by header
pub async fn versia_post<K: PrivateKey>(target: Url, content: &str, signing_key: K, signed_by: &str) -> Result<(), ()> {
    // generate a signing key
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash(content.as_bytes());
    // use signing key
    let signature_string = signature_string(HttpMethod::Post, path, &nonce, &hash);
    let signature = signing_key.sign(&signature_string).to_string();

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
