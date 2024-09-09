use crate::config::get_config;

use super::utility::new_actor::NewLocal;

#[actix_web::test]
#[ignore]
///requires a fresh db, doesn't clean up after itself yet
async fn create_and_retrieve_user() -> Result<(), String> {
    let config = get_config().unwrap();
    let conn = config.create_conn();
    conn.init().await.unwrap();

    let preferred_username = "create_and_retrieve_user".to_string();

    let uid = conn
        .create_local_user(&NewLocal::new(
            preferred_username.clone(),
            "filler".to_string(),
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let actor = conn.get_actor(uid, &config.instance_domain).await;
    let Some(actor) = actor else {
        return Err(
            "failed to retrieve actor with get_actor, may have failed to insert".to_string(),
        );
    };
    let second = conn
        .get_local_user_actor(&preferred_username, &config.instance_domain)
        .await;
    let Some((second_actor, second_uid)) = second else {
        return Err("failed to retrieve actor with get_local_user_actor".to_string());
    };

    if second_actor.id.ne(&actor.id) {
        return Err("id's of the retrieved actors don't match".to_string());
    }

    if uid != second_uid {
        return Err(
            "get_local_user_actor uid doesn't match uid returned from inserting".to_string(),
        );
    }

    let deleted = conn.delete_actor(uid, None).await;
    if deleted.is_err() {
        return Err("failed to delete the test user".to_string());
    }

    Ok(())
}
