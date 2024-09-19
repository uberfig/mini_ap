use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::web::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use textnonce::TextNonce;
use url::Url;

use crate::{
    ap_protocol::fetch::FetchErr, cryptography::{digest, key::{PrivateKey, PublicKey}}, db::conn::Conn, versia_types::entities::public_key::AlgorithmsPublicKey
};

pub enum HttpMethod {
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

pub struct ReqwestHeaders {
    pub headermap: reqwest::header::HeaderMap,
}

impl Headers for ReqwestHeaders {
    fn get(&self, key: &str) -> Option<&str> {
        let val = self.headermap.get(key).map(|x| x.to_str())?;
        match val {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}

pub enum Signer {
    User(Url),
    Instance(String),
}

fn signature_string(
    method: HttpMethod,
    path: &str,
    nonce: &str,
    hash: &str,
    // currently versia has made the decision to not include timestamps.
    // I have left this here because I feel that is a design mistake. it
    // can be enabled at any time if they decide to change their decision
    _timestamp: i64, 
) -> String {
    format!(
        "{} {} {} {}",
        method.stringify(),
        path,
        nonce,
        hash,
        // timestamp
    )
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VerifyRequestErr {
    MissingHeader(String),
    InvalidTimestamp,
    SignatureVerificationFailure,
    TooOld,
    UnableToObtainKey,
}

impl std::fmt::Display for VerifyRequestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyRequestErr::MissingHeader(x) => write!(f, "MissingHeader: {}", x),
            VerifyRequestErr::InvalidTimestamp => write!(f, "InvalidTimestamp"),
            VerifyRequestErr::SignatureVerificationFailure => write!(f, "SignatureVerificationFailure"),
            VerifyRequestErr::TooOld => write!(f, "TooOld"),
            VerifyRequestErr::UnableToObtainKey => write!(f, "UnableToObtainKey"),
        }
    }
}

pub async fn verify_request<T: Headers>(headers: &T, method: HttpMethod, path: &str, hash: &str, conn: &Data<Box<dyn Conn + Sync>>) -> Result<(), VerifyRequestErr> {
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

    // see the comment on signature_string 
    // let Some(signed_milis) = headers.get("Signed-milis") else {
    //     return Err(VerifyRequestErr::MissingHeader("Signed-milis".to_string()));
    // };
    // let signed_milis: Result<i64, _> = signed_milis.parse();
    // let signed_milis = match signed_milis {
    //     Ok(x) => x,
    //     Err(_) => return Err(VerifyRequestErr::InvalidTimestamp),
    // };
    // let Some(provided_time) = DateTime::from_timestamp_millis(signed_milis) else {
    //     return Err(VerifyRequestErr::InvalidTimestamp);
    // };
    // let current_time = Utc::now();
    // let duration = current_time - provided_time;
    // if duration.num_minutes() > 3 {
    //     return Err(VerifyRequestErr::TooOld);
    // }

    let Some(verifying_key) = conn.get_key(signed_by).await else {
        return Err(VerifyRequestErr::UnableToObtainKey);
    };

    let verify_string = signature_string(method, path, nonce, hash, 0);
    if verifying_key.verify(&verify_string, signature) {
        return Ok(());
    }
    Err(VerifyRequestErr::SignatureVerificationFailure)
}

pub async fn versia_fetch<T: for<'a> Deserialize<'a>, K: PrivateKey>(
    target: Url,
    signing_key: K,
    signed_by: &str,
    conn: &Data<Box<dyn Conn + Sync>>,
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
        headermap: res.headers().clone(),
    };

    let response = res.text().await;
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let hash = digest::sha256_hash(response.as_bytes());

    if let Err(val) = verify_request(&headers, HttpMethod::Get, path, &hash, conn).await {
        return Err(FetchErr::VerifyErr(val));
    }

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
