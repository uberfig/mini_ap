use std::time::SystemTime;

use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
};

use crate::{activitystream_objects::core_types::ActivityStream, db::Conn, protocol::verification::generate_digest};


// #[get("/actor")]
// pub async fn get_instance_actor(
//     cache: Data<Cache>,
//     path: web::Path<String>,
//     conn: Data<DbConn>,
//     request: HttpRequest,
//     body: web::Bytes,
// ) -> Result<HttpResponse> {
//     todo!()
// }

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
    dbg!(String::from_utf8(body.to_vec()));

    let preferred_username = path.into_inner();

    let actor = conn.get_local_user_actor(&preferred_username, &state.instance_domain).await;

    let actor = match actor {
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

// #[get("/create_test/{preferred_username}")]
// pub async fn create_test(
//     path: web::Path<String>,
//     state: Data<crate::config::Config>,
//     conn: Data<DbConn>,
// ) -> Result<HttpResponse> {
//     let preferred_username = path.into_inner();

//     let x = create_internal_actor(state, conn, preferred_username.clone(), preferred_username)
//         .await
//         .unwrap();

//     Ok(HttpResponse::Ok().body(format!("{x}")))
// }

// #[get("/post_test")]
// pub async fn post_test(
//     // state: Data<crate::config::Config>,
//     conn: Data<DbConn>,
// ) -> Result<HttpResponse> {
//     todo!()
//     let activity: ActivityStream = serde_json::from_str(activities::ACTIVITY).unwrap();

//     let val = sqlx::query!(
//         "SELECT private_key FROM  internal_users WHERE preferred_username = $1",
//         "test"
//     )
//     .fetch_one(&conn.db)
//     .await
//     .unwrap();

//     let key = openssl::rsa::Rsa::private_key_from_pem(val.private_key.as_bytes()).unwrap();

//     post_to_inbox(
//         &activity,
//         &"https://place.ivytime.gay/users/test".to_string(),
//         &"mastodon.social".to_string(),
//         &"https://mastodon.social/inbox".to_string(),
//         key,
//     )
//     .await;

//     Ok(HttpResponse::Ok().body(""))
// }


