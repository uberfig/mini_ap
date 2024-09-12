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
    ap_protocol::{
        fetch::FetchErr,
        incoming::{verify_incoming, RequestVerificationError},
    },
    db::{conn::Conn, incoming::process_incoming},
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
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    dbg!(&request);
    handle_inbox(request, "/inbox", inbox, body, conn, state).await
}

#[post("/users/{preferred_username}/inbox")]
pub async fn private_inbox(
    request: HttpRequest,
    path: web::Path<String>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    println!("private inbox");
    let preferred_username = path.into_inner();
    let path = format!("/users/{}/inbox", &preferred_username);

    handle_inbox(request, &path, inbox, body, conn, state).await
}

async fn handle_inbox(
    request: HttpRequest,
    path: &str,
    inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let instance_actor_key = conn.get_instance_actor().await.unwrap().get_rsa();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::Unauthorized()
            .body(serde_json::to_string(&RequestVerificationError::BadMessageBody).unwrap()));
    };

    // println!("{}", &body);

    let x = verify_incoming(
        request,
        &body,
        path,
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

            process_incoming(conn, state, x).await;

            return Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .body("OK".to_string()));
        }
        Err(x) => {
            if matches!(
                &x,
                RequestVerificationError::ActorFetchFailed(FetchErr::IsTombstone(_))
            ) {
                println!("another tombstone");
                return Ok(HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .body("OK".to_string()));
            }
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
