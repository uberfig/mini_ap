use actix_web::{
    get,
    http::Error,
    post,
    web::{self, Data},
    HttpResponse,
};

use crate::db::conn::Conn;

#[get("/users/{preferred_username}/outbox")]
pub async fn ap_outbox(
    path: web::Path<String>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    todo!()
}

#[post("/users/{preferred_username}/outbox")]
pub async fn create_ap_post(
    path: web::Path<String>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    todo!()
}
