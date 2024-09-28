use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::{
    activitystream_objects::{
        actors::Actor,
        inboxable::{Inboxable, InboxableVerifyErr, VerifiedInboxable},
    },
    cryptography::{
        digest::sha256_hash,
        key::{PrivateKey, PublicKey},
    },
    protocol::{errors::FetchErr, headers::Headers},
};

use super::fetch::authorized_fetch;

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
    ContentErr(InboxableVerifyErr),
    KeyOwnerDoesNotMatch,
    KeyLinkNotActor,
    CannotParseKeyUrl,
    KeyOwnerFromIP,
    InvalidKey,
}

/// verifys a request and returns the Inboxable if its valid
/// create activites are stripped and turned into their inner
/// postable so we don't have to deal with the added complexity
pub async fn verify_incoming<K: PrivateKey, H: Headers>(
    request_headers: &H,
    body: &str,
    path: &str,
    instance_domain: &str,
    instance_key_id: &str,
    instance_private_key: &mut K,
) -> Result<VerifiedInboxable, RequestVerificationError> {
    //check digest matches

    let Some(digest) = request_headers.get("Digest") else {
        return Err(RequestVerificationError::NoMessageDigest);
    };

    let object: Result<Inboxable, _> = serde_json::from_str(body);
    let Ok(object) = object else {
        println!("deserialize failure\n{}", body);
        return Err(RequestVerificationError::BodyDeserializeErr);
    };
    let generated_digest = "SHA-256=".to_owned() + &sha256_hash(body.as_bytes());

    if !digest.eq(&generated_digest) {
        return Err(RequestVerificationError::DigestDoesNotMatch);
    }

    //get the signature header

    let Some(signature_header) = request_headers.get("Signature") else {
        return Err(RequestVerificationError::NoMessageSignature);
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
    let Ok(key_id) = Url::parse(&key_id) else {
        return Err(RequestVerificationError::CannotParseKeyUrl);
    };
    let Some(domain) = key_id.domain() else {
        return Err(RequestVerificationError::KeyOwnerFromIP);
    };
    let Some(signature) = signature_header.get("signature") else {
        return Err(RequestVerificationError::NoSignature);
    };
    let signature = signature.replace('"', "");

    let fetched: Result<Actor, FetchErr> =
        authorized_fetch(&key_id, instance_key_id, instance_private_key).await;

    let actor = match fetched {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ActorFetchFailed(x)),
    };

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
                let x = format!("{signed_header_name}: {value}",);
                // dbg!(&x);
                Some(x)
            }
        })
        .collect();

    let comparison_string = comparison_string.join("\n");
    // dbg!(&comparison_string);

    let accepted = actor
        .public_key_object
        .public_key
        .verify(&comparison_string, &signature);

    if !accepted {
        return Err(RequestVerificationError::SignatureVerifyFailed);
    }

    let object = match object
        .verify(domain, instance_key_id, instance_private_key)
        .await
    {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ContentErr(x)),
    };

    Ok(object)
}
