use crate::db::conn::Conn;
use actix_web::{error::ErrorNotFound, get, web::Data, HttpResponse, Result};

#[get("/users/{uuid}/versia")]
pub async fn versia_user(
    path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    let uuid = path.into_inner();
    let user = conn.get_local_versia_user(&uuid, &state.instance_domain).await;

    match user {
        Some(x) => Ok(HttpResponse::Ok()
            .content_type("application/json; charset=UTF-8")
            .body(serde_json::to_string(&x).unwrap())),
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}
