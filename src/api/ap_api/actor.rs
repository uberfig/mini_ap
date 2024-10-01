use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpResponse, Result,
};

use crate::db::{conn::Conn, utility::new_actor::NewLocal};

#[get("/users/{preferred_username}/ap")]
pub async fn get_actor(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
    // request: HttpRequest,
    // body: web::Bytes,
) -> Result<HttpResponse> {
    // println!("getting the actor");

    // dbg!(request);
    // dbg!(&body);
    // dbg!(String::from_utf8(body.to_vec()).unwrap());

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
            &NewLocal::new(
            preferred_username,
            "filler".to_string(),
            None,
            None,
        ))
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

#[get("/actor/ap")]
pub async fn get_instance_actor(
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    println!("getting the instance actor");
    todo!()
    // Ok(HttpResponse::Ok()
    //     .content_type("application/activity+json; charset=utf-8")
    //     .body(
    //         serde_json::to_string(
    //             &conn
    //                 .get_instance_actor()
    //                 .await
    //                 .to_actor(&state.instance_domain)
    //                 .to_activitystream(),
    //         )
    //         .unwrap(),
    //     ))
}
