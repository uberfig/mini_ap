use url::Url;

use crate::{
    activitystream_objects::object::{Object, ObjectType},
    db::{get_post_id_and_published, PostSupertype, UserRef},
};

use super::pg_conn::PgConn;

pub async fn create_new_post(
    conn: &PgConn,
    post: &crate::db::PostType,
    instance_domain: &str,
    uid: UserRef,
    in_reply_to: Option<i64>,
) -> i64 {
    let (post_id, published) = get_post_id_and_published(uid.is_local(), post);
    let (fedi_actor, local_actor) = uid.parts();
    match &post {
        crate::db::PostType::Object(x) => {
            let client = conn.db.get().await.expect("failed to get client");

            let stmt = r#"
INSERT INTO posts 
(
is_local, fedi_id, surtype, subtype,
local_only, published, in_reply_to,
content, domain,
fedi_actor, local_actor
)
VALUES
(
$1, $2, $3, $4,
$5, $6, $7,
$8, $9,
$10, $11
)
RETURNING obj_id;
    "#;
            let stmt = client.prepare(stmt).await.unwrap();

            let result: i64 = client
                .query(
                    &stmt,
                    &[
                        &uid.is_local(),
                        &post_id,
                        &post.get_surtype(),
                        &post.get_subtype(),
                        &false,
                        &published,
                        &in_reply_to,
                        &x.object.content,
                        &instance_domain,
                        &fedi_actor,
                        &local_actor,
                    ],
                )
                .await
                .expect("failed to insert post")
                .pop()
                .expect("did not return obj_id")
                .get("obj_id");
            result
        }
        crate::db::PostType::Question(_x) => todo!(),
    }
}

pub async fn get_post(conn: &PgConn, object_id: i64) -> Option<crate::db::PostType> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    SELECT * FROM posts JOIN internal_users ON local_actor = uid WHERE obj_id = $1;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&object_id])
        .await
        .expect("failed to get post")
        .pop();
    let result = result?;

    let supertype: String = result.get("surtype");
    let supertype = PostSupertype::from_str(&supertype).expect("unkown supertype in posts");
    match supertype {
        PostSupertype::Object => {
            let is_local: bool = result.get("is_local");
            let (id, attributed_to) = match is_local {
                true => {
                    let preferred_username: String = result.get("preferred_username");
                    let domain: String = result.get("domain");
                    (
                        format!(
                            "https://{}/users/{}/statuses/{}",
                            &domain, &preferred_username, object_id
                        ),
                        format!("https://{}/users/{}", &domain, &preferred_username),
                    )
                }
                false => {
                    // (result.get("posts.fedi_id"), )
                    todo!()
                }
            };
            let subtype: String = result.get("subtype");
            let subtype: ObjectType =
                serde_json::from_str(&subtype).expect("unkown object type stored in db");
            // dbg!(&attributed_to);
            let attributed_to = Url::parse(&attributed_to).expect("invalid attributed to");

            let replied_obj: Option<i64> = result.get("in_reply_to");
            let replied_obj: Option<String> = match replied_obj {
                Some(_x) => {
                    todo!()
                }
                None => None,
            };

            let object = Object::new(Url::parse(&id).unwrap(), attributed_to)
                .content(result.get("content"))
                .in_reply_to(replied_obj)
                .published_milis(result.get("published"))
                .to_public()
                .wrap(subtype);

            Some(crate::db::PostType::Object(object))
        }
        PostSupertype::Question => todo!(),
    }
}
