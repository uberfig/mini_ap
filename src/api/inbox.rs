use std::sync::Mutex;

use actix_web::{
    error::Error,
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::{
    db::Conn,
    protocol::verification::{verify_incoming, RequestVerificationError},
};
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
    inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    dbg!(&request);
    let instance_actor_key = conn.get_instance_actor().await.unwrap().get_rsa();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::Unauthorized()
            .body(serde_json::to_string(&RequestVerificationError::BadMessageBody).unwrap()));
    };

    let x = verify_incoming(
        request,
        &body,
        "/inbox",
        &state.instance_domain,
        &format!("https://{}/actor#main-key", &state.instance_domain),
        &instance_actor_key,
    )
    .await;

    match x {
        Ok(x) => {
            {
                let mut guard = inbox.inbox.lock().unwrap();
                let data = &mut *guard;
                let deserialized = serde_json::to_string(&x).unwrap();
                data.push(format!("Success:\n{}", deserialized));
            }

            return Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .body("OK".to_string()));
        }
        Err(x) => {
            // dbg!(&x);
            {
                let mut guard = inbox.inbox.lock().unwrap();
                let data = &mut *guard;
                let deserialized = serde_json::to_string(&x).unwrap();
                data.push(format!("failure:{}\n{}", deserialized, body));
            }
            Ok(HttpResponse::Unauthorized().body(serde_json::to_string(&x).unwrap()))
        }
    }
}

#[post("/users/{preferred_username}/inbox")]
pub async fn private_inbox(
    request: HttpRequest,
    path: web::Path<String>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let preferred_username = path.into_inner();
    let path = format!("/users/{}/inbox", &preferred_username);

    let instance_actor_key = conn.get_instance_actor().await.unwrap().get_rsa();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::Unauthorized()
            .body(serde_json::to_string(&RequestVerificationError::BadMessageBody).unwrap()));
    };

    let x = verify_incoming(
        request,
        &body,
        &path,
        &state.instance_domain,
        &format!("https://{}/actor#main-key", &state.instance_domain),
        &instance_actor_key,
    )
    .await;

    match x {
        Ok(x) => {
            // println!("{}", &x);

            {
                let mut guard = inbox.inbox.lock().unwrap();
                let data = &mut *guard;
                let deserialized = serde_json::to_string(&x).unwrap();
                data.push(format!("Success:\n{}", deserialized));
            }

            return Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .body("OK".to_string()));
        }
        Err(x) => {
            // dbg!(&x);
            {
                let mut guard = inbox.inbox.lock().unwrap();
                let data = &mut *guard;
                let deserialized = serde_json::to_string(&x).unwrap();
                data.push(format!("failure:{}\n{}", deserialized, body));
                // let _hi = "IsTombstone".to_string();
                if matches!(&x, RequestVerificationError::ActorFetchFailed(body)) {
                    if body.starts_with("IsTombstone") {
                        dbg!("another tombstone");
                        return Ok(HttpResponse::Ok()
                            .status(StatusCode::OK)
                            .body("OK".to_string()));
                    }
                }
            }
            Ok(HttpResponse::Unauthorized().body(serde_json::to_string(&x).unwrap()))
        }
    }
}
