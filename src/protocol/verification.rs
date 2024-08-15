use std::{collections::HashMap, time::SystemTime};

use actix_web::HttpRequest;
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activitystream_objects::core_types::ActivityStream, protocol::fetch::authorized_fetch,
};

pub fn generate_digest(body: &[u8]) -> String {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha256()).unwrap();
    hasher.update(body).unwrap();
    let digest: &[u8] = &hasher.finish().unwrap();

    //digest_base64
    openssl::base64::encode_block(digest)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestVerificationError {
    NoMessageDigest,
    BadMessageDigest,
    BadMessageBody,
    DigestDoesNotMatch,
    NoMessageSignature,
    BadMessageSignature,
    NoSignatureKey,
    NoSignature,
    SignatureIncorrectBase64,
    ActorFetchFailed(String),
    ActorFetchBodyFailed,
    ActorDeserializeFailed,
    NoSignatureHeaders,
    SignatureVerifyFailed,
    NoDate,
    MissingSignedHeaderField(String),
    BodyDeserializeErr,
    ForgedAttribution,
    KeyOwnerDoesNotMatch,
    KeyLinkNotActor,
}

pub async fn post_to_inbox(
    // activity: &ActivityStream,
    activity: &str,
    from_id: &str,
    to_domain: &str,
    to_inbox: &str,
    keypair: &PKey<Private>,
) {
    // let keypair: PKey<Private> = PKey::from_rsa(private_key).unwrap();

    // let document = serde_json::to_string(activity).unwrap();

    let date = httpdate::fmt_http_date(SystemTime::now());

    let digest_base64 = &generate_digest(activity.as_bytes());

    //string to be signed
    let signed_string = format!("(request-target): post /inbox\nhost: {to_domain}\ndate: {date}\ndigest: SHA-256={digest_base64}");
    let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(signed_string.as_bytes()).unwrap();
    let signature = openssl::base64::encode_block(&signer.sign_to_vec().unwrap());

    // dbg!(&from_id);

    // let header: String = r#"keyId=""#.to_string()
    //     + from_id
    //     + r#"#main-key",headers="(request-target) host date digest",signature=""#
    //     + &signature
    //     + r#"""#;
    let header = format!(
        r#"keyId="{from_id}#main-key",headers="(request-target) host date digest",signature="{signature}""#
    );

    let client = reqwest::Client::new();
    let client = client
        .post(to_inbox)
        .header("Host", to_domain)
        .header("Date", date)
        .header("Signature", header)
        .header("Digest", "SHA-256=".to_owned() + digest_base64)
        .body(activity.to_string());

    dbg!(&client);

    let res = client.send().await;
    dbg!(&res);

    let response = res.unwrap().text().await;

    dbg!(&response);

    if let Ok(x) = response {
        println!("{}", x);
    }
}

///verifys a request and returns the message body if its valid
pub async fn verify_incoming(
    request: HttpRequest,
    body: &str,
    path: &str,
    instance_domain: &str,
    // instance_public_key_pem: String,
    instance_key_id: &str,
    instance_private_key: &Rsa<Private>,
) -> Result<ActivityStream, RequestVerificationError> {
    let request_headers = request.headers();

    //check digest matches

    let Some(digest) = request_headers.get("Digest") else {
        return Err(RequestVerificationError::NoMessageDigest);
    };

    let Ok(digest) = String::from_utf8(digest.as_bytes().to_vec()) else {
        return Err(RequestVerificationError::BadMessageDigest);
    };

    let object: Result<ActivityStream, _> = serde_json::from_str(&body);
    let Ok(object) = object else {
        println!("deserialize failure\n{}", body);
        return Err(RequestVerificationError::BodyDeserializeErr);
    };

    // if object.is_activity() {
    //     let Ok(_) = object.verify_attribution(cache, conn).await else {
    //         return Err(RequestVerificationError::ForgedAttribution);
    //     };
    // }

    let generated_digest = "SHA-256=".to_owned() + &generate_digest(body.as_bytes());

    if !digest.eq(&generated_digest) {
        return Err(RequestVerificationError::DigestDoesNotMatch);
    }

    //get the signature header

    let Some(signature_header) = request_headers.get("Signature") else {
        return Err(RequestVerificationError::NoMessageSignature);
    };

    let Ok(signature_header) = String::from_utf8(signature_header.as_bytes().to_vec()) else {
        return Err(RequestVerificationError::BadMessageSignature);
    };

    let signature_header: HashMap<String, String> = signature_header
        .split(',')
        .filter_map(|pair| {
            pair.split_once('=').map(|(key, value)| {
                (
                    key.replace("/[^A-Za-z]/", ""),
                    value.replace("/[^A-Za-z]/", ""),
                )
            })
        })
        .collect();

    let Some(key_id) = signature_header.get("keyId") else {
        return Err(RequestVerificationError::NoSignatureKey);
    };
    let key_id = key_id.replace('"', "");
    // println!("key id: \n{}\n\n", &key_id);

    let Some(signature) = signature_header.get("signature") else {
        return Err(RequestVerificationError::NoSignature);
    };
    let signature = signature.replace('"', "");

    // dbg!(&signature);

    let fetched = authorized_fetch(
        &Url::parse(&key_id).unwrap(),
        &instance_key_id,
        instance_private_key,
    )
    .await;

    // dbg!(&fetched);

    let fetched = match fetched {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ActorFetchFailed(x.to_string())),
    };

    let Some(actor) = fetched.get_actor() else {
        return Err(RequestVerificationError::KeyLinkNotActor);
    };

    // let client = reqwest::Client::new();
    // let client = client
    //     .get(key_id)
    //     .header("accept", "application/activity+json");

    // let Ok(res) = client.send().await else {
    //     return Err(RequestVerificationError::ActorFetchFailed);
    // };

    // let Ok(actor) = res.bytes().await else {
    //     return Err(RequestVerificationError::ActorFetchBodyFailed);
    // };

    // println!("actor:\n{}", String::from_utf8((&actor).to_vec()).unwrap());
    // let actor: Result<Actor, _> = serde_json::from_slice(&actor);
    // let Ok(actor) = actor else {
    //     dbg!(&actor);
    //     return Err(RequestVerificationError::ActorDeserializeFailed);
    // };

    if let Some(x) = object.get_owner() {
        if actor.get_id().domain().ne(&x.domain()) {
            println!(
                "KeyOwnerDoesNotMatch, \nobject owner: {} \nactor: {}",
                x.as_str(),
                actor.get_id()
            );
            return Err(RequestVerificationError::KeyOwnerDoesNotMatch);
        }
    }

    let key =
        openssl::rsa::Rsa::public_key_from_pem(actor.public_key.public_key_pem.as_bytes()).unwrap();

    let Some(headers) = signature_header.get("headers") else {
        return Err(RequestVerificationError::NoSignatureHeaders);
    };

    let Some(_) = request_headers.get("date") else {
        return Err(RequestVerificationError::NoDate);
    };

    //generate a sign string of the actual request's headers with the real header values mentoned in the provided sign string

    let comparison_string: Vec<String> = headers
        .replace('"', "")
        .split(' ')
        .filter_map(|signed_header_name| match signed_header_name {
            "(request-target)" => Some(format!("(request-target): post {path}")),
            "host" => Some(format!("host: {instance_domain}")),
            _ => {
                let value = request_headers.get(signed_header_name)?;

                let value = String::from_utf8(value.as_bytes().to_vec()).unwrap();
                let x = format!("{signed_header_name}: {value}",);
                dbg!(&x);
                Some(x)
            }
        })
        .collect();

    let comparison_string = comparison_string.join("\n");
    dbg!(&comparison_string);

    let pubkey = openssl::pkey::PKey::from_rsa(key).unwrap();

    let mut verifier =
        openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pubkey).unwrap();
    let input = &comparison_string;
    verifier.update(input.as_bytes()).unwrap();

    let signature = openssl::base64::decode_block(&signature).unwrap();
    let accepted = verifier.verify(&signature).unwrap();

    if !accepted {
        return Err(RequestVerificationError::SignatureVerifyFailed);
    }

    Ok(object)
}
