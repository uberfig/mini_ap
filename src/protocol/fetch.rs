use std::{fmt::Display, time::SystemTime};

use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
};
use url::Url;

use crate::activitystream_objects::core_types::ActivityStream;

#[derive(Debug)]
pub enum FetchErr {
    IsTombstone(String),
    RequestErr(reqwest::Error),
    DeserializationErr(serde_json::Error),
}

impl Display for FetchErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchErr::IsTombstone(x) => write!(f, "IsTombstone: {}", x),
            FetchErr::RequestErr(x) => write!(f, "RequestErr: {}", x),
            FetchErr::DeserializationErr(x) => write!(f, "DeserializationErr: {}", x),
        }
    }
}

pub async fn authorized_fetch(
    object_id: &Url,
    key_id: &str,
    private_key: &Rsa<Private>,
) -> Result<ActivityStream, FetchErr> {
    let path = object_id.path();
    let fetch_domain = object_id.domain().unwrap();

    let keypair = PKey::from_rsa(private_key.clone()).unwrap();

    let date = httpdate::fmt_http_date(SystemTime::now());

    //string to be signed
    let signed_string = format!("(request-target): get {path}\nhost: {fetch_domain}\ndate: {date}\naccept: application/activity+json");
    let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(signed_string.as_bytes()).unwrap();
    let signature = openssl::base64::encode_block(&signer.sign_to_vec().unwrap());

    let header = format!(
        r#"keyId="{key_id}",headers="(request-target) host date accept",signature="{signature}""#
    );

    let client = reqwest::Client::new();
    let client = client
        .get(object_id.clone())
        .header("Host", fetch_domain)
        .header("Date", date)
        .header("Signature", header)
        .header("accept", "application/activity+json")
        .body("");

    dbg!(&client);

    let res = client.send().await;
    dbg!(&res);

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x)),
    };

    let response = res.text().await;
    // dbg!(&response);
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x)),
    };

    if response.eq(r#"{"error":"Gone"}"#) {
        return Err(FetchErr::IsTombstone(object_id.to_string()));
    }
    println!("auth fetch got:\n{}", &response);

    let object: Result<ActivityStream, serde_json::Error> = serde_json::from_str(&response);
    let object = match object {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::DeserializationErr(x)),
    };

    Ok(object)
}
