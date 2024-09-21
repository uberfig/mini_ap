use crate::{
    cryptography::digest::sha256_hash,
    db::conn::{Conn, EntityOrigin},
    protocol::{
        headers::ActixHeaders,
        versia_protocol::{signatures::HttpMethod, verify::verify_request},
    },
    versia_types::{
        entities::{
            change_follow::ChangeFollowing, delete::Delete, follow_response::FollowResponse,
            instance_metadata::InstanceMetadata, user::User,
        },
        postable::Postable,
    },
};
use actix_web::{error::ErrorBadRequest, rt::spawn};

use actix_web::{
    dev::ResourcePath, error::ErrorUnauthorized, post, web::Data, HttpRequest, HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VersiaInboxItem {
    Post(Postable),
    Delete(Delete),
    ChangeFollowing(ChangeFollowing),
    FollowResponse(FollowResponse),
    /// used when a user updates their profile
    User(Box<User>),
    InstanceMetadata(Box<InstanceMetadata>),
}

#[post("/users/{uuid}/inbox/versia")]
pub async fn versia_user_inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    inbox(request, body, actix_path, state, conn).await
}
#[post("/inbox/versia")]
pub async fn versia_shared_inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    inbox(request, body, actix_path, state, conn).await
}

pub async fn inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    let path = actix_path.path().to_string();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Err(ErrorUnauthorized("bad request body"));
    };
    let hash = sha256_hash(body.as_bytes());

    let headers = ActixHeaders {
        headermap: request.headers().clone(),
    };

    let authorized = verify_request(&headers, HttpMethod::Get, &path, &hash, &conn).await;

    let signer = match authorized {
        Ok(x) => x,
        Err(err) => return Err(ErrorUnauthorized(err)),
    };

    let deserialized: Result<VersiaInboxItem, _> = serde_json::from_str(&body);

    match deserialized {
        Ok(x) => {
            spawn(handle_inbox(signer, x, state, conn));
            Ok(HttpResponse::Ok()
                .content_type("application/json; charset=UTF-8")
                .body(""))
        }
        Err(x) => Err(ErrorBadRequest(x)),
    }
}

#[allow(unused_variables)]
pub async fn handle_inbox(
    signer: Url,
    entity: VersiaInboxItem,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) {
    // all signers should have a domain. federation with an ip address will
    // never be supported as they can 1. be dynamic, 2. be used to skirt defeds
    let Some(authoratative_domain) = signer.domain() else {
        return;
    };
    match entity {
        VersiaInboxItem::Post(postable) => {
            // another instance is trying to impersonate this user
            // we could log this in the future
            if postable.get_author().domain().ne(&signer.domain()) {
                return;
            }
            let post = conn
                .create_versia_post(postable, &EntityOrigin::Federated(authoratative_domain))
                .await
                .expect("failed to insert post");
        }
        VersiaInboxItem::Delete(delete) => match delete.deleted_type {
            crate::versia_types::entities::delete::DeletedType::Note
            | crate::versia_types::entities::delete::DeletedType::Share => {
                conn.delete_post(&delete.id, &EntityOrigin::Federated(authoratative_domain))
                    .await
                    .expect("failed to delete post");
            }
            crate::versia_types::entities::delete::DeletedType::User => todo!(),
        },
        VersiaInboxItem::ChangeFollowing(change_following) => todo!(),
        VersiaInboxItem::FollowResponse(follow_response) => todo!(),
        VersiaInboxItem::User(user) => todo!(),
        VersiaInboxItem::InstanceMetadata(instance_metadata) => todo!(),
    }
}
