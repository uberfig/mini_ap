use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use openssl::rsa::Rsa;
use sqlx::query;

use crate::db::{internal_actor::get_actor_id_from_internal, public_key::insert_public_key};

use super::conn::DbConn;

pub struct UserLinks {
    pub id: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub liked: String,
}

fn generate_links(domain: &str, uname: &str) -> UserLinks {
    UserLinks {
        id: format!("https://{domain}/users/{uname}"),
        inbox: format!("https://{domain}/users/{uname}/inbox"),
        outbox: format!("https://{domain}/users/{uname}/outbox"),
        followers: format!("https://{domain}/users/{uname}/followers"),
        following: format!("https://{domain}/users/{uname}/following"),
        liked: format!("https://{domain}/users/{uname}/liked"),
    }
}

pub async fn create_internal_actor(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    username: String,
    password: String,
) -> Result<i64, ()> {
    let mut transaction = conn.db.begin().await.unwrap();

    //confirm that the username is not taken
    let val = get_actor_id_from_internal(&mut *transaction, "test").await;

    if val.unwrap().is_some() {
        return Err(());
    };

    let links = generate_links(&state.instance_domain, &username);

    let rsa = Rsa::generate(2048).unwrap();

    let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();

    let public = rsa.public_key_to_pem().unwrap();

    let key_id = format!(
        "https://{}/users/{}#main-key",
        &state.instance_domain, &username
    );
    dbg!(&key_id);

    let x =
        insert_into_ap_users(&mut *transaction, &username, &state.instance_domain, &links).await;

    let _key_id = insert_public_key(
        &mut *transaction,
        &key_id,
        &links.id,
        &String::from_utf8(public).unwrap(),
    )
    .await
    .unwrap();

    let actor = x.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);

    if password_hash.is_err() {
        return Err(());
    }

    let pass = password_hash.unwrap().to_string();

    let uid =
        insert_into_local_users(&mut *transaction, &pass, &username, actor, &private_key).await;

    transaction.commit().await.unwrap();

    Ok(uid.unwrap())
}

pub async fn insert_into_ap_users<'e, 'c: 'e, E>(
    executor: E,
    username: &str,
    domain: &str,
    links: &UserLinks,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"INSERT INTO activitypub_users
            (id, preferred_username, domain, inbox, outbox, followers, following, liked)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING ap_user_id
        "#,
        links.id,
        username,
        domain,
        links.inbox,
        links.outbox,
        links.followers,
        links.following,
        links.liked,
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.ap_user_id),
        Err(x) => Err(x),
    }
}

pub async fn insert_into_local_users<'e, 'c: 'e, E>(
    executor: E,
    pass: &str,
    username: &str,
    actor: i64,
    private_key: &str,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"INSERT INTO internal_users 
            (password, preferred_username, activitypub_actor, private_key )
        VALUES
            ($1, $2, $3, $4)
        RETURNING uid
        "#,
        pass,
        &username,
        actor,
        private_key
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.uid),
        Err(x) => Err(x),
    }
}
