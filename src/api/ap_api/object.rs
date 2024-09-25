use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::db::conn::Conn;

#[get("/users/{preferred_username}/statuses/{id}/ap")]
pub async fn get_object(
    path: web::Path<(String, i64)>,
    conn: Data<Box<dyn Conn + Sync>>,
    // request: HttpRequest,
    // body: web::Bytes,
    // state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    todo!()
    // dbg!(request);
    // dbg!(&body);

    // let (_preferred_username, object_id) = path.into_inner();
    // // println!("getting an object, {}", object_id);

    // let object = conn.get_post(object_id).await;

    // let object = match object {
    //     Some(x) => x,
    //     None => {
    //         return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    //     }
    // };
    // // let object: ActivityStream = object.into();

    // Ok(HttpResponse::Ok()
    //     .content_type("application/activity+json; charset=utf-8")
    //     .body(serde_json::to_string(&object).unwrap()))
}

#[get("/users/{preferred_username}/statuses/{id}/activity/ap")]
pub async fn get_object_create(
    path: web::Path<(String, i64)>,
    conn: Data<Box<dyn Conn + Sync>>,
    request: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse> {
    todo!()
    // println!("getting an object");

    // dbg!(request);
    // dbg!(&body);

    // let (_preferred_username, object_id) = path.into_inner();

    // let object = conn.get_post(object_id).await;

    // let object = match object {
    //     Some(x) => x,
    //     None => {
    //         return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    //     }
    // };
    // let object: ActivityStream = object.to_create_activitystream();

    // Ok(HttpResponse::Ok()
    //     .content_type("application/activity+json; charset=utf-8")
    //     .body(serde_json::to_string(&object).unwrap()))
}
