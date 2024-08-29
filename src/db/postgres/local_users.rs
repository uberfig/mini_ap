use super::pg_conn::PgConn;

pub async fn create_local_user(conn: &PgConn, user: &crate::db::NewLocal) -> Result<i64, ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    INSERT INTO internal_users 
    (
        password, preferred_username, email, private_key_pem,
        public_key_pem, permission_level, custom_domain
    )
    VALUES
    (
        $1, $2, $3, $4,
        $5, $6, $7
    )
    RETURNING local_id;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let permission: i16 = user.permission_level.into();

    let local_id: i64 = client
        .query(
            &stmt,
            &[
                &user.password,
                &user.username,
                &user.email,
                &user.private_key_pem,
                &user.public_key_pem,
                &permission,
                &user.custom_domain,
            ],
        )
        .await
        .expect("failed to insert user")
        .pop()
        .expect("did not return uid")
        .get("local_id");

    let stmt = r#"
    INSERT INTO unified_users 
    (
        is_local, local_id
    )
    VALUES
    (
        $1, $2
    )
    RETURNING uid;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&true, &local_id])
        .await
        .expect("failed to insert user")
        .pop()
        .expect("did not return uid")
        .get("uid");

    Ok(result)
}
