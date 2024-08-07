use std::collections::HashMap;

use actix_web::{
    web::{self, Data},
    HttpRequest,
};
use openssl::hash::MessageDigest;
use serde::{Deserialize, Serialize};

use crate::{
    activitystream_objects::{core_types::ActivityStream, VerificationActor},
    cache_and_fetch::Cache,
    db::conn::DbConn,
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
    ActorFetchFailed,
    ActorFetchBodyFailed,
    ActorDeserializeFailed,
    NoSignatureHeaders,
    SignatureVerifyFailed,
    NoDate,
    MissingSignedHeaderField(String),
    BodyDeserializeErr,
    ForgedAttribution,
    KeyOwnerDoesNotMatch,
}

///verifys a request and returns the message body if its valid
pub async fn verify_incoming(
    cache: &Cache,
    conn: &Data<DbConn>,
    request: HttpRequest,
    body: web::Bytes,
    path: &str,
    instance_domain: &str,
) -> Result<String, RequestVerificationError> {
    let request_headers = request.headers();

    //check digest matches

    let Some(digest) = request_headers.get("Digest") else {
        return Err(RequestVerificationError::NoMessageDigest);
    };

    let Ok(digest) = String::from_utf8(digest.as_bytes().to_vec()) else {
        return Err(RequestVerificationError::BadMessageDigest);
    };

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Err(RequestVerificationError::BadMessageBody);
    };

    let object: Result<ActivityStream, _> = serde_json::from_str(&body);
    let Ok(object) = object else {
        return Err(RequestVerificationError::BodyDeserializeErr);
    };

    if object.is_activity() {
        let Ok(_) = object.verify_attribution(cache, conn).await else {
            return Err(RequestVerificationError::ForgedAttribution);
        };
    }

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
    println!("key id: \n{}\n\n", &key_id);

    let Some(signature) = signature_header.get("signature") else {
        return Err(RequestVerificationError::NoSignature);
    };
    let signature = signature.replace('"', "");

    dbg!(&signature);

    let client = reqwest::Client::new();
    let client = client
        .get(key_id)
        .header("accept", "application/activity+json");

    let Ok(res) = client.send().await else {
        return Err(RequestVerificationError::ActorFetchFailed);
    };

    let Ok(actor) = res.bytes().await else {
        return Err(RequestVerificationError::ActorFetchBodyFailed);
    };

    let actor: Result<VerificationActor, _> = serde_json::from_slice(&actor);
    let Ok(actor) = actor else {
        dbg!(&actor);
        return Err(RequestVerificationError::ActorDeserializeFailed);
    };

    if let Some(x) = object.get_owner() {
        if actor.id.ne(x) {
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

    Ok(body)
}
