use serde::{Deserialize, Serialize};

use crate::{
    activitystream_objects::{
        actors::Actor,
        inboxable::{Inboxable, InboxableVerifyErr, VerifiedInboxable},
    },
    cryptography::{
        digest::sha256_hash,
        key::{PrivateKey, PublicKey},
    },
    protocol::{errors::FetchErr, headers::Headers, http_method::HttpMethod},
};

use super::{
    fetch::authorized_fetch,
    signature::{Signature, SignatureErr},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestVerificationError {
    NoMessageDigest,
    BadMessageDigest,
    BadMessageBody,
    DigestDoesNotMatch,
    NoSignatureHeader,
    ActorFetchFailed(FetchErr),
    ActorFetchBodyFailed,
    SignatureVerifyFailed,
    BodyDeserializeErr,
    ContentErr(InboxableVerifyErr),
    SignatureErr(SignatureErr),
}

/// verifys a request and returns the Inboxable if its valid
/// create activites are stripped and turned into their inner
/// postable so we don't have to deal with the added complexity
pub async fn verify_post<K: PrivateKey, H: Headers>(
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
    if !digest.starts_with("SHA-256=") {}

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
        return Err(RequestVerificationError::NoSignatureHeader);
    };

    let signature = match Signature::from_request(
        HttpMethod::Post,
        instance_domain.to_string(),
        path.to_string(),
        &signature_header,
    ) {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::SignatureErr(x)),
    };

    let fetched: Result<Actor, FetchErr> = authorized_fetch(
        &signature.signature_header.key_id,
        instance_key_id,
        instance_private_key,
    )
    .await;

    let actor = match fetched {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ActorFetchFailed(x)),
    };

    let Some(_) = request_headers.get("date") else {
        return Err(RequestVerificationError::SignatureErr(SignatureErr::NoDate));
    };

    //generate a sign string of the actual request's headers with the real header values mentoned in the provided sign string
    let comparison_string = match signature.generate_sign_string(request_headers) {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::SignatureErr(x)),
    };

    let accepted = actor.public_key.public_key_pem.verify(
        comparison_string.as_bytes(),
        &signature.signature_header.signature,
    );

    if !accepted {
        return Err(RequestVerificationError::SignatureVerifyFailed);
    }

    let object = match object
        .verify(
            &signature.signature_header.key_domain,
            instance_key_id,
            instance_private_key,
        )
        .await
    {
        Ok(x) => x,
        Err(x) => return Err(RequestVerificationError::ContentErr(x)),
    };

    Ok(object)
}
