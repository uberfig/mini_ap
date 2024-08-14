use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::db::{Conn, NewLocal};

#[get("/users/{preferred_username}")]
pub async fn get_actor(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn>>,
    request: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse> {
    println!("getting the actor");

    dbg!(request);
    dbg!(&body);
    dbg!(String::from_utf8(body.to_vec()).unwrap());

    let preferred_username = path.into_inner();

    let actor = conn
        .get_local_user_actor(&preferred_username, &state.instance_domain)
        .await;

    let (actor, _) = match actor {
        Some(x) => x,
        None => {
            return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
        }
    };
    let actor = actor.to_activitystream();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}

#[get("/create_test/{preferred_username}")]
pub async fn create_test(
    path: web::Path<String>,
    conn: Data<Box<dyn Conn>>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();

    let x = conn
        .create_local_user(&NewLocal::new(
            preferred_username,
            "filler".to_string(),
            None,
            None,
            None,
        ))
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

#[get("/actor")]
pub async fn get_instance_actor(
    conn: Data<Box<dyn Conn>>,
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
                    .unwrap()
                    .to_actor(&state.instance_domain)
                    .to_activitystream(),
            )
            .unwrap(),
        ))
}
