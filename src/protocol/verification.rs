use crate::activitystream_objects::core_types::ActivityStream;

/// authoratative_domain is the domain of the actor who signed the oject or activity.
/// if they do not match this will return an err
pub async fn verify_attribution(
    activitystream: &ActivityStream,
    authoratative_domain: &str,
) -> Result<(), ()> {
    match &activitystream.content.activity_stream {
        crate::activitystream_objects::core_types::ExtendsObject::Object(x) => {
            if x.object.get_attributed_to().domain().is_none()
                || x.object
                    .get_attributed_to()
                    .domain()
                    .unwrap()
                    .ne(authoratative_domain)
            {
                return Err(());
            }
        }
        crate::activitystream_objects::core_types::ExtendsObject::ExtendsIntransitive(x) => {
            if x.get_actor().domain().unwrap().ne(authoratative_domain) {
                return Err(());
            }
        }
        crate::activitystream_objects::core_types::ExtendsObject::ExtendsCollection(_) => todo!(),
        crate::activitystream_objects::core_types::ExtendsObject::Actor(_) => todo!(),
    }

    return Ok(());
}
