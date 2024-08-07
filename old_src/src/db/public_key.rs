use sqlx::query;

use crate::activitystream_objects::actors::{Actor, PublicKey};

pub async fn insert_public_key<'e, 'c: 'e, E>(
    executor: E,
    id: &str,
    actor_id: &str,
    public_key_pem: &str,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"INSERT INTO public_keys 
            (id, owner, public_key_pem)
        VALUES
            ($1, $2, $3)
        RETURNING pub_key_id
        "#,
        id,
        actor_id,
        public_key_pem
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.pub_key_id),
        Err(x) => Err(x),
    }
}

/// Note don't forget to insert the actor first
pub async fn insert_actor_public_key<'e, 'c: 'e, E>(
    executor: E,
    actor: &Actor,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = actor.extends_object.id.as_str();

    insert_public_key(
        executor,
        &actor.public_key.id,
        actor_id,
        &actor.public_key.public_key_pem,
    )
    .await
}

pub async fn get_actor_public_key<'e, 'c: 'e, E>(
    executor: E,
    owner: &str,
) -> Result<PublicKey, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let key = sqlx::query!("SELECT * FROM public_keys WHERE owner = $1", owner)
        .fetch_one(executor)
        .await
        .unwrap();

    Ok(PublicKey {
        id: key.id,
        owner: key.owner,
        public_key_pem: key.public_key_pem,
    })
}
