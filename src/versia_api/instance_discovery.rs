use actix_web::{
    get, web::Data, HttpRequest, HttpResponse, Result
};
use crate::db::conn::Conn;

#[get("/.well-known/versia")]
pub async fn versia_metadata(
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let request_headers = request.headers();
    let _accept = request_headers.get("Accept").map(|x| String::from_utf8(x.as_bytes().to_vec()));

    let metadata = conn.get_versia_instance_metadata(&state.instance_domain).await;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .body(serde_json::to_string(&metadata).unwrap()))
}