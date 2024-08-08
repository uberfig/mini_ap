use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, RwLock},
};

use actix_web::web::Data;
use url::Url;

use crate::{
    activitystream_objects::core_types::ActivityStream,
    db::conn::DbConn,
    protocol::{fetch::authorized_fetch, instance_actor::InstanceActor},
};

// const MAX_AGE: std::time::Duration = Duration::from_secs(40);

// const MAX_ADVERSE: i32 = 6;

// const base: i64 = 2;

#[derive(Debug, Clone)]
pub struct DomainRequest {
    pub last_adverse: u64,
    pub adverse_events: u64,
}

#[derive(Debug)]
pub struct CachedItem<T: Clone> {
    pub item: RwLock<T>,
    // pub fetched_at: SystemTime,
    // pub fetched_at: SystemTime,
    /// mark as non existent or no longer existing
    pub tombstone: AtomicBool,
    /// set when the item is changed in the database
    pub stale: AtomicBool,
}

pub struct Cache {
    pub state: crate::config::Config,
    pub instance_actor: InstanceActor,
    pub domains: RwLock<HashMap<String, DomainRequest>>,
    pub outgoing_cache: RwLock<HashMap<String, String>>, //cache of objects being externally requested
    pub fetch: RwLock<HashMap<String, CachedItem<ActivityStream>>>, //cache of objects being fetched
}

impl Cache {
    pub fn new(instance_actor: InstanceActor, state: crate::config::Config) -> Cache {
        Cache {
            state,
            instance_actor,
            domains: RwLock::new(HashMap::new()),
            outgoing_cache: RwLock::new(HashMap::new()),
            fetch: RwLock::new(HashMap::new()),
        }
    }
}

pub async fn get_local_object(id: &Url) -> ActivityStream {
    todo!()
}

#[derive(Debug, Clone)]
pub enum FetchErr {
    MaxAdverse,
    DoesNotExist,
}

pub async fn get_federated_object(
    id: &Url,
    cache: &Cache,
    conn: &Data<DbConn>,
) -> Result<ActivityStream, FetchErr> {
    {
        let read_lock = cache.fetch.read().unwrap();
        let cached = read_lock.get(id.as_str());

        if let Some(x) = &cached {
            dbg!(x);

            if x.tombstone.load(std::sync::atomic::Ordering::Acquire) {
                return Err(FetchErr::DoesNotExist);
            }

            if x.stale.load(std::sync::atomic::Ordering::Acquire) {
                //get from db
            } else {
                return Ok(x.item.read().unwrap().clone());
            }

            // let time = SystemTime::now();
            // let elapsed = time.duration_since(x.fetched_at);

            // let elapsed = match elapsed {
            //     Ok(x) => x,
            //     Err(x) => x.duration(),
            // };

            // if elapsed.as_secs() > MAX_AGE.as_secs() {
            //     //get from database, it may have had an update activity or smth
            //     todo!()
            // } else {
            //     return Ok(x.item.clone());
            // }
        }
    }

    let object = authorized_fetch(
        id,
        &cache.instance_actor.key_id,
        &cache.instance_actor.private_key,
    )
    .await;
    let object = match object {
        Ok(x) => x,
        Err(x) => todo!(),
    };

    // let time = SystemTime::now();

    {
        let mut write_lock = cache.fetch.write().unwrap();
        write_lock.insert(
            id.as_str().to_owned(),
            CachedItem {
                item: RwLock::new(object.clone()),
                tombstone: AtomicBool::new(false),
                stale: AtomicBool::new(false),
                // fetched_at: time,
            },
        );
    }

    Ok(object)
}

pub async fn fetch_object(
    id: &Url,
    cache: &Cache,
    conn: &Data<DbConn>,
) -> Result<ActivityStream, FetchErr> {
    if let Some(x) = id.domain() {
        if x.eq_ignore_ascii_case(&cache.state.instance_domain) {
            return Ok(get_local_object(id).await);
        }
        return get_federated_object(id, cache, conn).await;
    }

    todo!()
}
