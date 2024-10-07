use super::super::errors::FetchErr;
use serde::Deserialize;
use std::time::SystemTime;
use url::Url;

use crate::cryptography::key::PrivateKey;

/// key_id and private_key are the properties of the key
/// being used to perform the fetch. usually done by the
/// instance actor
pub async fn authorized_fetch<T: PrivateKey, F: for<'a> Deserialize<'a>>(
    object_id: &Url,
    key_id: &str,
    private_key: &mut T,
) -> Result<F, FetchErr> {
    let path = object_id.path();
    let Some(fetch_domain) = object_id.host_str() else {
        return Err(FetchErr::InvalidUrl(object_id.as_str().to_string()));
    };

    let date = httpdate::fmt_http_date(SystemTime::now());

    //string to be signed
    let signed_string = format!("(request-target): get {path}\nhost: {fetch_domain}\ndate: {date}\naccept: application/activity+json");
    let signature = private_key.sign(&signed_string);

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

    // dbg!(&client);

    let res = client.send().await;
    // dbg!(&res);

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let response = res.text().await;
    // dbg!(&response);
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    if response.eq(r#"{"error":"Gone"}"#) {
        return Err(FetchErr::IsTombstone(object_id.to_string()));
    }
    // println!("auth fetch got:\n{}", &response);

    let object: Result<F, serde_json::Error> = serde_json::from_str(&response);
    let object = match object {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::DeserializationErr(x.to_string())),
    };

    Ok(object)
}
