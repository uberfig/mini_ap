use std::time::{SystemTime, UNIX_EPOCH};

use tokio_postgres::Row;

use crate::db::UserRef;

use super::pg_conn::PgConn;

fn to_follower(row: Row) -> UserRef {
    let fedi_from: Option<i64> = row.get("fedi_from");
    match fedi_from {
        Some(x) => UserRef::Activitypub(x),
        None => {
            let local_from: Option<i64> = row.get("local_from");
            UserRef::Local(local_from.unwrap())
        }
    }
}

pub async fn create_follow_request(
    conn: &PgConn,
    from_id: UserRef,
    to_id: UserRef,
) -> Result<(), ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        INSERT INTO following 
        (
            fedi_from, local_from,
            target_fedi, target_local,
            pending, published
        )
        VALUES
        (
            $1, $2, 
            $3, $4,
            $5, $6
        );
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let (fedi_from, local_from) = from_id.parts();
    let (target_fedi, target_local) = to_id.parts();
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let _result = client
        .query(
            &stmt,
            &[
                &fedi_from,
                &local_from,
                &target_fedi,
                &target_local,
                &false,
                &created,
            ],
        )
        .await
        .expect("failed to create follow");

    Ok(())
}

pub async fn approve_follow_request(
    conn: &PgConn,
    from_id: UserRef,
    to_id: UserRef,
) -> Result<(), ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        UPDATE following 
        SET pending = false
        WHERE
        fedi_from = $1
        local_from = $2
        target_fedi = $3
        target_local = $4;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let (fedi_from, local_from) = from_id.parts();
    let (target_fedi, target_local) = to_id.parts();

    let _result = client
        .query(
            &stmt,
            &[&fedi_from, &local_from, &target_fedi, &target_local],
        )
        .await
        .expect("failed to approve follow");

    Ok(())
}

pub async fn get_followers(conn: &PgConn, user: UserRef) -> Result<Vec<UserRef>, ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = match user {
        UserRef::Local(_) => {
            r#"
            SELECT * FROM following 
            WHERE
            target_local = $1;
            "#
        }
        UserRef::Activitypub(_) => {
            r#"
            SELECT * FROM following 
            WHERE
            target_fedi = $1;
            "#
        }
    };
    let stmt = client.prepare(stmt).await.unwrap();

    let id = user.id();

    // let (target_fedi, target_local) = user.parts();

    let result = client.query(&stmt, &[&id]).await;
    // .expect("failed to get followers");
    dbg!(&result);
    let result = result.unwrap();

    let x = result.into_iter().map(to_follower);

    Ok(x.collect())
}

pub async fn get_follower_count(conn: &PgConn, user: UserRef) -> Result<i64, ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        SELECT COUNT(*) FROM following 
        WHERE
        target_fedi = $1
        target_local = $1;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let (target_fedi, target_local) = user.parts();

    let result: i64 = client
        .query(&stmt, &[&target_fedi, &target_local])
        .await
        .expect("failed to get follow count")
        .pop()
        .expect("did not return row for follow count")
        .get("count");

    Ok(result)
}
