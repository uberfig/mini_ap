use crate::{
    cryptography::digest::sha256_hash,
    db::conn::{Conn, EntityOrigin, VersiaConn},
    protocol::{
        headers::ActixHeaders, http_method::HttpMethod, versia_protocol::verify::verify_request
    },
};
use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::Data,
    HttpRequest, HttpResponse, Result,
};

#[get("/users/@{uname}/statuses/{pid}/versia")]
pub async fn versia_posts(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<(String, String)>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    let (uname, pid) = actix_path.into_inner();
    let path = format!("/users/@{}/statuses/{}/versia", &uname, &pid);

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Err(ErrorUnauthorized("bad request body"));
    };
    let hash = sha256_hash(body.as_bytes());

    let authorized = verify_request(
        &ActixHeaders {
            headermap: request.headers().clone(),
        },
        HttpMethod::Get,
        &path,
        &hash,
        &VersiaConn { conn: &conn },
    )
    .await;

    if let Err(err) = authorized {
        return Err(ErrorUnauthorized(err));
    }

    let post = conn
        .get_versia_post(&pid, &EntityOrigin::Local(&state.instance_domain))
        .await;

    match post {
        Some(x) => Ok(HttpResponse::Ok()
            .content_type("application/json; charset=UTF-8")
            .body(serde_json::to_string(&x).unwrap())),
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}
