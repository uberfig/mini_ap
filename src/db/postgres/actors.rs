use tokio_postgres::Row;

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
