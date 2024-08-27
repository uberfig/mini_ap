use actix_web::web::Data;

use crate::activitystream_objects::{activities::ExtendsIntransitive, core_types::ActivityStream};

use super::conn::Conn;

pub async fn process_incoming(
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
    activitystream: ActivityStream,
) {
    match activitystream.content.activity_stream {
        crate::activitystream_objects::core_types::ExtendsObject::Object(_) => todo!(),
        crate::activitystream_objects::core_types::ExtendsObject::ExtendsIntransitive(x) => {
            process_intransitive(conn, state, x).await;
        }
        crate::activitystream_objects::core_types::ExtendsObject::ExtendsCollection(_) => todo!(),
        crate::activitystream_objects::core_types::ExtendsObject::Actor(_) => todo!(),
    }
}

pub async fn process_intransitive(
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
    intransitive: Box<ExtendsIntransitive>,
) {
    match *intransitive {
        ExtendsIntransitive::ExtendsActivity(activity) => {
            match activity.type_field {
                crate::activitystream_objects::activities::ActivityType::Like => todo!(),

                crate::activitystream_objects::activities::ActivityType::Create => todo!(),
                crate::activitystream_objects::activities::ActivityType::Delete => todo!(),

                crate::activitystream_objects::activities::ActivityType::Follow => {
                    let from_id = activity.extends_intransitive.actor.get_id();
                    let from = conn.get_federated_user_db_id(from_id.as_str()).await;
                    let from = match from {
                        Some(x) => Ok(x),
                        None => {
                            conn.load_federated_actor(from_id, &state.instance_domain)
                                .await
                        }
                    };

                    let from = match from {
                        Ok(x) => x,
                        Err(x) => {
                            dbg!(x);
                            return ();
                        }
                    };

                    //should be the preferred username
                    let Some(to) = activity
                        .object
                        .get_id()
                        .as_str()
                        .strip_prefix(&format!("https://{}/users/", &state.instance_domain))
                    else {
                        println!("invalid username: {}", activity.object.get_id().as_str());
                        return ();
                    };
                    let to = conn.get_local_user_db_id(to).await;
                    match to {
                        Some(to) => {
                            conn.create_follow_request(
                                super::UserRef::Activitypub(from),
                                super::UserRef::Local(to),
                            )
                            .await
                            .unwrap();
                        }
                        None => return,
                    }
                }
                crate::activitystream_objects::activities::ActivityType::Accept => todo!(),
                crate::activitystream_objects::activities::ActivityType::Reject => todo!(),

                crate::activitystream_objects::activities::ActivityType::Activity => todo!(),

                crate::activitystream_objects::activities::ActivityType::TentativeAccept => todo!(),
                crate::activitystream_objects::activities::ActivityType::Add => todo!(),

                crate::activitystream_objects::activities::ActivityType::Ignore => todo!(),
                crate::activitystream_objects::activities::ActivityType::Join => todo!(),
                crate::activitystream_objects::activities::ActivityType::Leave => todo!(),

                crate::activitystream_objects::activities::ActivityType::Offer => todo!(),
                crate::activitystream_objects::activities::ActivityType::Invite => todo!(),

                crate::activitystream_objects::activities::ActivityType::TentativeReject => todo!(),
                crate::activitystream_objects::activities::ActivityType::Remove => todo!(),
                crate::activitystream_objects::activities::ActivityType::Undo => todo!(),
                crate::activitystream_objects::activities::ActivityType::Update => todo!(),
                crate::activitystream_objects::activities::ActivityType::View => todo!(),
                crate::activitystream_objects::activities::ActivityType::Listen => todo!(),
                crate::activitystream_objects::activities::ActivityType::Read => todo!(),
                crate::activitystream_objects::activities::ActivityType::Move => todo!(),
                crate::activitystream_objects::activities::ActivityType::Announce => todo!(),
                crate::activitystream_objects::activities::ActivityType::Block => todo!(),
                crate::activitystream_objects::activities::ActivityType::Flag => todo!(),
                crate::activitystream_objects::activities::ActivityType::Dislike => todo!(),
            }
        }
        ExtendsIntransitive::Question(_) => todo!(),
    }
}
