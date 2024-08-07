use std::sync::Mutex;

use actix_web::{
    // body,
    // cookie::{time::convert::Second, Cookie},
    error::Error,
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    HttpRequest,
    HttpResponse,
    Result,
};

use crate::{cache_and_fetch::Cache, db::conn::DbConn, protocol::verification::verify_incoming};
pub struct Inbox {
    pub inbox: Mutex<Vec<String>>,
}

#[get("/inspect")]
pub async fn inspect_inbox(inbox: Data<Inbox>) -> String {
    let mut guard = inbox.inbox.lock().unwrap();
    let data = &mut *guard;

    format!("inbox: \n{}", data.join("\n\n"))
}

#[post("/inbox")]
pub async fn shared_inbox(
    request: HttpRequest,
    // conn: Data<DbConn>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
) -> Result<HttpResponse, Error> {
    dbg!(&request);

    let x = verify_incoming(
        &cache,
        &conn,
        request,
        body,
        "/users/test/inbox",
        "place.ivytime.gay",
    )
    .await;

    match x {
        Ok(x) => {
            println!("{}", &x);

            let mut guard = inbox.inbox.lock().unwrap();
            let data = &mut *guard;
            data.push(x);

            return Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .body("OK".to_string()));
        }
        Err(x) => {
            dbg!(&x);
            Ok(HttpResponse::Unauthorized().body(serde_json::to_string(&x).unwrap()))
        }
    }
}

#[post("/users/{preferred_username}/inbox")]
pub async fn private_inbox(
    request: HttpRequest,
    // conn: Data<DbConn>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
) -> Result<HttpResponse, Error> {
    // let mut guard = inbox.inbox.lock().unwrap();
    // let data = &mut *guard;

    // let val = String::from_utf8(body.to_vec());

    // if let Ok(val) = val {
    //     data.push(val);
    // }
    // let path = "/users/test/inbox";
    // let x = request.cookie("example");

    dbg!(&request);

    let x = verify_incoming(
        &cache,
        &conn,
        request,
        body,
        "/users/test/inbox",
        "place.ivytime.gay",
    )
    .await;

    match x {
        Ok(x) => {
            println!("{}", &x);

            let mut guard = inbox.inbox.lock().unwrap();
            let data = &mut *guard;
            data.push(x);

            return Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .body("OK".to_string()));
        }
        Err(x) => {
            dbg!(&x);
            Ok(HttpResponse::Unauthorized().body(serde_json::to_string(&x).unwrap()))
        }
    }
}
