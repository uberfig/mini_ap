use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::{
    db::{
        conn::Conn,
        utility::{instance_actor::InstanceActor, new_actor::NewLocal},
    },
    protocol::{ap_protocol::verification::verify_get, headers::ActixHeaders},
};

#[get("/users/{preferred_username}")]
pub async fn get_actor(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    dbg!(&request);

    if state.force_auth_fetch {
        let headers = ActixHeaders {
            headermap: request.headers().clone(),
        };
        let instance_key = conn.get_instance_actor().await;
        let verified = verify_get(
            &headers,
            path.as_str(),
            &state.instance_domain,
            &InstanceActor::get_key_id(&state.instance_domain),
            &mut instance_key.get_private_key(),
        )
        .await;

        if let Err(err) = verified {
            return Err(ErrorUnauthorized(serde_json::to_string(&err).unwrap()));
        }
    }

    let preferred_username = path.into_inner();

    let actor = conn
        .get_local_actor(&preferred_username, &state.instance_domain)
        .await;

    let Some(actor) = actor else {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    };
    // let actor = actor.to_activitystream();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}

#[get("/create_test/{preferred_username}")]
pub async fn create_test(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();

    let x = conn
        .create_user(
            &state.instance_domain,
            &NewLocal::new(preferred_username, "filler".to_string(), None, None),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

#[get("/actor")]
pub async fn get_instance_actor(
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    println!("getting the instance actor");
    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(
            serde_json::to_string(
                &conn
                    .get_instance_actor()
                    .await
                    .to_actor(&state.instance_domain)
                    .wrap_context(),
            )
            .unwrap(),
        ))
}
