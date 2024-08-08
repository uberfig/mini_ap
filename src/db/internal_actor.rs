pub async fn get_actor_id_from_internal<'e, 'c: 'e, E>(
    executor: E,
    username: &str,
) -> Result<Option<i64>, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = sqlx::query!(
        "SELECT activitypub_actor FROM  internal_users WHERE preferred_username = $1",
        username
    )
    .fetch_optional(executor)
    .await;
    match val {
        Ok(x) => match x {
            Some(x) => Ok(Some(x.activitypub_actor)),
            None => Ok(None),
        },
        Err(x) => Err(x),
    }
}
