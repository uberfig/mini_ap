use tokio_postgres::Row;
use url::Url;

use crate::{
    activitystream_objects::actors::{Actor, ActorType, PublicKey},
    db::generate_links,
};

use super::pg_conn::PgConn;

fn local_user_from_row(result: Row, instance_domain: &str) -> Actor {
    let preferred_username: String = result.get("preferred_username");
    let links = generate_links(instance_domain, &preferred_username);

    let key = PublicKey {
        id: links.pub_key_id,
        owner: links.id.clone(),
        public_key_pem: result.get("public_key_pem"),
    };

    Actor {
        type_field: ActorType::Person,
        id: links.id,
        preferred_username,
        summary: result.get("summary"),
        name: result.get("display_name"),
        url: Some(links.url),
        public_key: key,
        inbox: links.inbox,
        outbox: links.outbox,
        followers: links.followers,
        following: links.following,
        domain: Some(instance_domain.to_string()),
        liked: Some(links.liked),
    }
}

fn fedi_user_from_row(result: Row) -> Actor {
    let id: String = result.get("id");
    let id = Url::parse(&id).unwrap();
    let type_field: String = result.get("type_field");
    let type_field: ActorType = serde_json::from_str(&type_field).unwrap();
    let preferred_username: String = result.get("preferred_username");
    let url: String = result.get("url");
    let url = Url::parse(&url).unwrap();
    let public_key_id: String = result.get("public_key_id");
    let inbox: String = result.get("inbox");
    let outbox: String = result.get("outbox");
    let followers: String = result.get("followers");
    let following: String = result.get("following");
    
    let key = PublicKey {
        id: Url::parse(&public_key_id).unwrap(),
        owner: id.clone(),
        public_key_pem: result.get("public_key_pem"),
    };

    Actor {
        type_field,
        id,
        preferred_username,
        summary: result.get("summary"),
        name: result.get("name"),
        url: Some(url),
        public_key: key,
        inbox: Url::parse(&inbox).unwrap(),
        outbox: Url::parse(&outbox).unwrap(),
        followers: Url::parse(&followers).unwrap(),
        following: Url::parse(&following).unwrap(),
        domain: None,
        liked: None,
    }
}

pub async fn get_local_user_actor(
    conn: &PgConn,
    preferred_username: &str,
    instance_domain: &str,
) -> Option<(Actor, i64)> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        SELECT * FROM internal_users JOIN unified_users local_id = local_id WHERE preferred_username = $1;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&preferred_username])
        .await
        .expect("failed to get local user")
        .pop();

    let result = match result {
        Some(x) => x,
        None => return None,
    };
    let id: i64 = result.get("uid");

    Some((local_user_from_row(result, instance_domain), id))
}

pub async fn get_actor(conn: &PgConn, uid: i64, instance_domain: &str) -> Option<Actor> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        SELECT * FROM unified_users JOIN internal_users local_id = local_id JOIN federated_ap_users fedi_id = fedi_id WHERE uid = $1;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&uid])
        .await
        .expect("failed to get actor")
        .pop();

    let result = match result {
        Some(x) => x,
        None => return None,
    };

    let is_local: bool = result.get("is_local");

    match is_local {
        true => Some(local_user_from_row(result, instance_domain)),
        false => Some(fedi_user_from_row(result)),
    }
}
