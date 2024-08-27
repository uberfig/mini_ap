use std::collections::HashMap;

use actix_web::HttpRequest;
use openssl::{pkey::Private, rsa::Rsa};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activitystream_objects::core_types::ActivityStream,
    protocol::{
        digest::generate_digest, fetch::authorized_fetch, verification::verify_attribution,
    },
};

use super::fetch::FetchErr;

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
    ActorFetchFailed(FetchErr),
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
    CannotParseKeyUrl,
    KeyOwnerFromIP,
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

    let object: Result<ActivityStream, _> = serde_json::from_str(body);
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

    let Ok(key_id) = Url::parse(&key_id) else {
        return Err(RequestVerificationError::CannotParseKeyUrl);
    };

    let Some(domain) = key_id.domain() else {
        return Err(RequestVerificationError::KeyOwnerFromIP);
    };

    let Ok(_) = verify_attribution(&object, domain).await else {
        return Err(RequestVerificationError::ForgedAttribution);
    };

    let Some(signature) = signature_header.get("signature") else {
        return Err(RequestVerificationError::NoSignature);
    };
    let signature = signature.replace('"', "");

    // dbg!(&signature);

    let fetched = authorized_fetch(&key_id, instance_key_id, instance_private_key).await;

    // dbg!(&fetched);

    let fetched = match fetched {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ActorFetchFailed(x)),
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
                // dbg!(&x);
                Some(x)
            }
        })
        .collect();

    let comparison_string = comparison_string.join("\n");
    // dbg!(&comparison_string);

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
