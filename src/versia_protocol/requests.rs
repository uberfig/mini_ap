use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use textnonce::TextNonce;
use url::Url;
use xsd_types::lexical::duration;

use crate::{
    ap_protocol::fetch::FetchErr,
    cryptography::{digest, key::{PrivateKey, PublicKey}}, versia_types::entities::public_key::AlgorithmsPublicKey,
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

pub trait Headers {
    fn get(&self, key: &str) -> Option<&str>;
}

pub struct ReqwestHeaders<'a> {
    pub headermap: &'a reqwest::header::HeaderMap,
}

impl Headers for ReqwestHeaders<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        let val = self.headermap.get(key).map(|x| x.to_str())?;
        match val {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}

enum Signer {
    User(Url),
    Instance(String),
}

fn signature_string(
    method: HttpMethod,
    path: &str,
    nonce: &str,
    hash: &str,
    timestamp: i64,
) -> String {
    format!(
        "{} {} {} {} {}",
        method.stringify(),
        path,
        nonce,
        hash,
        timestamp
    )
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

pub enum VerifyRequestErr {
    MissingHeader(String),
    InvalidTimestamp,
    SignatureVerificationFailure,
    TooOld,
    UnableToObtainKey,
}

pub fn get_key(signed_by: &str) -> Option<AlgorithmsPublicKey> {
    todo!()
}

pub fn verify_request<T: Headers>(headers: T, method: HttpMethod, path: &str, hash: &str) -> Result<(), VerifyRequestErr> {
    let Some(_content_type) = headers.get("Content-Type") else {
        return Err(VerifyRequestErr::MissingHeader("Content-Type".to_string()));
    };
    let Some(signature) = headers.get("X-Signature") else {
        return Err(VerifyRequestErr::MissingHeader("X-Signature".to_string()));
    };
    let Some(signed_by) = headers.get("X-Signed-By") else {
        return Err(VerifyRequestErr::MissingHeader("X-Signed-By".to_string()));
    };
    let Some(nonce) = headers.get("X-Nonce") else {
        return Err(VerifyRequestErr::MissingHeader("X-Nonce".to_string()));
    };
    let Some(signed_milis) = headers.get("Signed-milis") else {
        return Err(VerifyRequestErr::MissingHeader("Signed-milis".to_string()));
    };
    let signed_milis: Result<i64, _> = signed_milis.parse();
    let signed_milis = match signed_milis {
        Ok(x) => x,
        Err(_) => return Err(VerifyRequestErr::InvalidTimestamp),
    };

    let Some(provided_time) = DateTime::from_timestamp_millis(signed_milis) else {
        return Err(VerifyRequestErr::InvalidTimestamp);
    };

    let current_time = Utc::now();

    let duration = current_time - provided_time;
    if duration.num_minutes() > 3 {
        return Err(VerifyRequestErr::TooOld);
    }

    let Some(verifying_key) = get_key(signed_by) else {
        return Err(VerifyRequestErr::UnableToObtainKey);
    };

    let verify_string = signature_string(method, path, nonce, hash, signed_milis);
    if verifying_key.verify(&verify_string, signature) {
        return Ok(());
    }
    return Err(VerifyRequestErr::SignatureVerificationFailure);
}

pub async fn versia_fetch<T: for<'a> Deserialize<'a>, K: PrivateKey>(
    target: Url,
    signing_key: K,
    signed_by: &str,
) -> Result<T, FetchErr> {
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash("".as_bytes());
    let signed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let signature_string = signature_string(HttpMethod::Get, path, &nonce, &hash, signed_at);
    let signature = signing_key.sign(&signature_string).to_string();

    let client = reqwest::Client::new();
    let client = client
        .get(target.clone())
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by)
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .header("Signed-milis", signed_at)
        .body("");

    let res = client.send().await;

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let headers = ReqwestHeaders {
        headermap: res.headers(),
    };

    let response = res.text().await;
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    // perform verification upon the response here

    let object: Result<T, serde_json::Error> = serde_json::from_str(&response);
    let object = match object {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::DeserializationErr(x.to_string())),
    };

    Ok(object)
}

/// if signed by the instance, use instance for the signed by header
pub async fn versia_post<K: PrivateKey>(
    target: Url,
    content: &str,
    signing_key: K,
    signed_by: &str,
) -> Result<(), ()> {
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash(content.as_bytes());
    let signed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let signature_string = signature_string(HttpMethod::Post, path, &nonce, &hash, signed_at);
    let signature = signing_key.sign(&signature_string).to_string();

    let client = reqwest::Client::new();
    let client = client
        .post(target)
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by)
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .header("Signed-milis", signed_at)
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
