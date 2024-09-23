use url::Url;

use crate::{
    cryptography::key::PublicKey,
    protocol::{errors::VerifyRequestErr, headers::Headers}, versia_types::entities::public_key::AlgorithmsPublicKey,
};

use super::signatures::{signature_string, HttpMethod};

/// note that the warning is junk as following it breaks everything with 
/// the use of async trait and trait objects with async 
#[allow(async_fn_in_trait)]
pub trait VersiaVerificationCache {
    async fn get_key(&self, signed_by: &Url) -> Option<AlgorithmsPublicKey>;
}

/// returns the signer if
pub async fn verify_request<H: Headers, V: VersiaVerificationCache>(
    headers: &H,
    method: HttpMethod,
    path: &str,
    hash: &str,
    conn: &V,
) -> Result<Url, VerifyRequestErr> {
    let Some(_content_type) = headers.get("Content-Type") else {
        return Err(VerifyRequestErr::MissingHeader("Content-Type".to_string()));
    };
    let Some(signature) = headers.get("X-Signature") else {
        return Err(VerifyRequestErr::MissingHeader("X-Signature".to_string()));
    };
    let Some(signed_by) = headers.get("X-Signed-By") else {
        return Err(VerifyRequestErr::MissingHeader("X-Signed-By".to_string()));
    };
    let Ok(signed_by) = Url::parse(&signed_by) else {
        return Err(VerifyRequestErr::InvalidSigner);
    };
    if signed_by.domain().is_none() {
        return Err(VerifyRequestErr::NoDomain);
    }
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

    let Some(verifying_key) = conn.get_key(&signed_by).await else {
        return Err(VerifyRequestErr::UnableToObtainKey);
    };

    let verify_string = signature_string(method, path, &nonce, hash, 0);
    if verifying_key.verify(&verify_string, &signature) {
        return Ok(signed_by);
    }
    Err(VerifyRequestErr::SignatureVerificationFailure)
}
