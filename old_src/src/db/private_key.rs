use sqlx::query;
use url::Url;

use crate::activitystream_objects::actors::PublicKey;

pub async fn get_private_key<'e, 'c: 'e, E>(
    executor: E,
    userid: &Url,
) -> Result<Option<PublicKey>, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = userid.as_str();

    let val = query!(
        r#"SELECT * FROM public_keys
            WHERE owner = $1        
        "#,
        actor_id,
    )
    .fetch_optional(executor)
    .await;

    match val {
        Ok(x) => match x {
            Some(x) => Ok(Some(PublicKey {
                id: x.id,
                owner: x.owner,
                public_key_pem: x.public_key_pem,
            })),
            None => Ok(None),
        },
        Err(x) => Err(x),
    }
}
