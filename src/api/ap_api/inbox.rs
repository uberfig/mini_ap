use std::sync::Mutex;

use actix_web::{
    error::{Error, ErrorUnauthorized},
    get,
    http::StatusCode,
    post,
    rt::spawn,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::{
    db::{
        conn::{Conn, EntityOrigin},
        utility::instance_actor::InstanceActor,
    },
    protocols::{
        protocol::{
            ap_protocol::verification::{verify_post, RequestVerificationError},
            headers::ActixHeaders,
        },
        types::activitystream_objects::inboxable::VerifiedInboxable,
    },
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
    // inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    dbg!(&request);
    inbox(request, body, conn, state).await
}

#[post("/users/{preferred_username}/inbox")]
pub async fn private_inbox(
    request: HttpRequest,
    path: web::Path<String>,
    // inbox: Data<Inbox>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    println!("private inbox");
    let preferred_username = path.into_inner();
    let path = format!("/ap/users/{}/inbox", &preferred_username);

    inbox(request, body, conn, state).await
}

async fn inbox(
    request: HttpRequest,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::Unauthorized()
            .body(serde_json::to_string(&RequestVerificationError::BadMessageBody).unwrap()));
    };
    let mut instance_actor_key = conn.get_instance_actor().await.get_private_key();

    let headers = ActixHeaders {
        headermap: request.headers().clone(),
    };
    let verified = match verify_post(
        &headers,
        &body,
        request.path(),
        &state.instance_domain,
        &InstanceActor::get_key_id(&state.instance_domain),
        &mut instance_actor_key,
    )
    .await
    {
        Ok(ok) => ok,
        Err(err) => return Err(ErrorUnauthorized(serde_json::to_string(&err).unwrap())),
    };

    spawn(handle_inbox(conn, state, verified));

    Ok(HttpResponse::Ok().status(StatusCode::ACCEPTED).body(""))
}

#[allow(unused_variables)]
async fn handle_inbox(
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
    item: VerifiedInboxable,
) {
    match item {
        VerifiedInboxable::Postable(postable) => {
            let id = postable.id().clone();
            let _ = conn
                .create_ap_post(
                    postable,
                    &EntityOrigin::Federated(id.domain().expect("verified post missing domain")),
                )
                .await;
        }
        VerifiedInboxable::Delete(delete) => todo!(),
        VerifiedInboxable::Follow(follow) => todo!(),
        VerifiedInboxable::FollowResponse(follow_response) => todo!(),
    }
}
