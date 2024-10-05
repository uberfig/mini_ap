use actix_web::{
    http::Error,
    post,
    web::{self, Data},
    HttpResponse,
};

use crate::db::conn::Conn;

#[post("/users/{preferred_username}/outbox")]
pub async fn create_post(
    path: web::Path<String>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    todo!()
}
