use crate::db::InstanceActor;

use super::pg_conn::PgConn;

pub async fn get_instance_actor(conn: &PgConn) -> Option<crate::db::InstanceActor> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    SELECT * FROM ap_instance_actor;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await
        .expect("failed to get instance actor")
        .pop();
    match result {
        Some(result) => Some(InstanceActor {
            private_key_pem: result.get("private_key_pem"),
            public_key_pem: result.get("public_key_pem"),
        }),
        None => None,
    }
}

pub async fn create_instance_actor(conn: &PgConn, private_key_pem: String, public_key_pem: String) {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    INSERT INTO ap_instance_actor 
    (private_key_pem, public_key_pem)
    VALUES
    ($1, $2);
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&private_key_pem, &public_key_pem])
        .await;
    result.unwrap();
}
