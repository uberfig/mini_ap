use actix_web::web::Data;
use sqlx::query;

use crate::activitystream_objects::{
    actors::{Actor, ActorType},
    core_types::ActivityStream,
    object::Object,
};

use super::{
    conn::DbConn,
    public_key::{get_actor_public_key, insert_actor_public_key},
};

///inserts an actor and its public key
pub async fn create_ap_actor(actor: &Actor, conn: &Data<DbConn>) -> Result<i64, InsertErr> {
    let mut transaction = conn.db.begin().await.unwrap();

    let ap_id = insert_actor_into_ap_users(&mut *transaction, actor).await;

    let ap_id = match ap_id {
        Ok(x) => x,
        Err(x) => {
            transaction.rollback().await.unwrap();
            return Err(x);
        }
    };

    let key_id = insert_actor_public_key(&mut *transaction, actor).await;

    let _key_id = match key_id {
        Ok(x) => x,
        Err(x) => {
            transaction.rollback().await.unwrap();
            return Err(InsertErr::DbErr(x));
        }
    };

    transaction.commit().await.unwrap();

    Ok(ap_id)
}

pub enum InsertErr {
    NoDomain,
    DbErr(sqlx::Error),
}

/// only inserts the actor don't forget to insert the public key
pub async fn insert_actor_into_ap_users<'e, 'c: 'e, E>(
    executor: E,
    actor: &Actor,
) -> Result<i64, InsertErr>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = actor.extends_object.id.as_str();
    let Some(domain) = actor.extends_object.id.domain() else {
        return Err(InsertErr::NoDomain);
    };

    let val = query!(
        r#"INSERT INTO activitypub_users 
            (id, preferred_username, domain, inbox, outbox, followers, following, liked)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8 )
        RETURNING ap_user_id
        "#,
        actor_id,
        actor.preferred_username,
        domain,
        actor.inbox,
        actor.outbox,
        actor.followers,
        actor.following,
        actor.liked
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => return Ok(x.ap_user_id),
        Err(x) => return Err(InsertErr::DbErr(x)),
    }
}

pub async fn get_ap_actor_by_db_id(id: i64, conn: &Data<DbConn>) -> Actor {
    let actor = sqlx::query!("SELECT * FROM activitypub_users WHERE ap_user_id = $1", id)
        .fetch_one(&conn.db)
        .await
        .unwrap();
    // let test = actor.type_field;
    let type_field: Result<ActorType, _> = serde_json::from_str(&actor.type_field);
    let type_field = type_field.expect("somehow an invalid actor type got into the db");

    let object = Object::new(url::Url::parse(&actor.id).unwrap());

    let public_key = get_actor_public_key(&conn.db, &actor.id).await.unwrap();

    Actor {
        type_field,
        preferred_username: actor.preferred_username,
        extends_object: object,
        public_key,
        inbox: actor.inbox,
        outbox: actor.outbox,
        followers: actor.followers,
        following: actor.following,
        ap_user_id: Some(actor.ap_user_id),
        domain: Some(actor.domain),
        liked: actor.liked,
    }
}
