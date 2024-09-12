use actix_web::{
    http::Error,
    post,
    web::{self, Data},
    HttpResponse,
};
use openssl::pkey::PKey;
use url::Url;

use crate::{
    activitystream_objects::object::{Object, ObjectType},
    ap_protocol::outgoing::post_to_inbox,
    db::conn::Conn,
};

#[post("/users/{preferred_username}/outbox")]
pub async fn create_post(
    path: web::Path<String>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let preferred_username = path.into_inner();
    let (actor, uid) = conn
        .get_local_user_actor(&preferred_username, &state.instance_domain)
        .await
        .unwrap();
    // let user_id = format!(
    //     "https://{}/users/{}",
    //     &state.instance_domain, &preferred_username
    // );

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::BadRequest().body("invalid body"));
    };

    // dbg!(&user_id);

    let object = Object::new(
        Url::parse("https://temp.com").unwrap(),
        Url::parse("https://temp.com").unwrap(),
    )
    .content(Some(body))
    .set_attributed_to(Url::parse(actor.get_id().as_str()).unwrap())
    .wrap(ObjectType::Note);
    let obj_id = conn
        .create_new_post(
            &crate::db::utility::post_types::PostType::Object(object),
            &state.instance_domain,
            uid,
            true,
            None,
        )
        .await;

    // let id_link = format!(
    //     "https://{}/users/{}/statuses/{}",
    //     &state.instance_domain, preferred_username, obj_id
    // );

    let object = conn.get_post(obj_id).await;

    let key = conn.get_local_user_private_key_db_id(uid).await;

    let key = openssl::rsa::Rsa::private_key_from_pem(key.as_bytes()).unwrap();
    let key = PKey::from_rsa(key).unwrap();

    let activity = object.unwrap().to_create_activitystream();
    let activity_str = serde_json::to_string(&activity).unwrap();

    let followers = conn.get_followers(uid).await.unwrap();

    let from_id = actor.get_id().as_str();

    for follower in followers {
        if !follower.is_local {
            let actor = conn
                .get_actor(follower.uid, &state.instance_domain)
                .await
                .unwrap();
            let domain = actor.get_id().domain().unwrap();
            println!("posting to inbox");
            post_to_inbox(&activity_str, from_id, domain, actor.inbox.as_str(), &key).await;
        }
    }

    Ok(HttpResponse::Created().body(activity_str.to_string()))
}

// #[get("/users/{preferred_username}/outbox")]
// pub async fn private_outbox(
//     request: HttpRequest,
//     path: web::Path<String>,
//     // conn: Data<DbConn>,
//     body: web::Bytes,
//     conn: Data<Box<dyn Conn>>,
//     state: Data<crate::config::Config>,
// ) -> Result<HttpResponse, Error> {
//     let preferred_username = path.into_inner();
//     return Ok(HttpResponse::NotFound().body(format!("")));
// }
