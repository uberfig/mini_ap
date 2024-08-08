use openssl::{pkey::Private, rsa::Rsa};
use url::Url;

use crate::{
    activitystream_objects::{
        actors::{Actor, ActorType, PublicKey},
        object::Object,
    },
    db::account_creation::UserLinks,
};

pub struct InstanceActor {
    pub actor: Actor,
    pub key_id: String,
    pub private_key: Rsa<Private>,
}

fn instance_actor_links(domain: &str) -> UserLinks {
    UserLinks {
        id: format!("https://{domain}/actor"),
        inbox: format!("https://{domain}/actor/inbox"),
        outbox: format!("https://{domain}/actor/outbox"),
        followers: format!("https://{domain}/actor/followers"),
        following: format!("https://{domain}/actor/following"),
        liked: format!("https://{domain}/actor/liked"),
    }
}

impl InstanceActor {
    pub fn new(private_key: Rsa<Private>, public_key_pem: String, domain: &str) -> InstanceActor {
        let links = instance_actor_links(domain);
        let object = Object::new(Url::parse(&links.id).unwrap());
        let public_key = PublicKey {
            id: format!("{}#main-key", &links.id),
            owner: links.id,
            public_key_pem,
        };
        let key_id = public_key.id.clone();
        let actor = Actor {
            type_field: ActorType::Application,
            preferred_username: "bayou.internal".to_owned(),
            extends_object: object,
            public_key: public_key,
            inbox: links.inbox,
            outbox: links.outbox,
            followers: links.followers,
            following: links.following,
            ap_user_id: None,
            domain: None,
            liked: None,
        };

        InstanceActor {
            actor,
            key_id: key_id,
            private_key,
        }
    }
}
