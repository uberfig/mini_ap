pub mod conn;
// pub mod incoming;
pub mod postgres;
#[cfg(test)]
pub mod tests;
pub mod utility;

use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use utility::{permission::PermissionLevel, post_types::PostType};

use crate::protocol::protocols::Protocols;

#[derive(Debug, Clone, Copy)]
pub struct Follower {
    pub uid: i64,
    pub is_local: bool,
    pub protocol: Protocols,
}

#[derive(Debug, Clone, Copy)]
pub struct Like {
    pub uid: i64,
    pub is_local: bool,
    pub obj_id: i64,
}

// pub fn get_published(is_local: bool, post: &PostType) -> i64 {
//     match is_local {
//         true => SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_millis() as i64,
//         false => {
//             let time = match post.get_published() {
//                 Some(x) => {
//                     let parsed = DateTime::parse_from_rfc3339(x);
//                     match parsed {
//                         Ok(x) => x.timestamp_millis(),
//                         Err(_) => SystemTime::now()
//                             .duration_since(UNIX_EPOCH)
//                             .unwrap()
//                             .as_millis() as i64,
//                     }
//                 }
//                 None => SystemTime::now()
//                     .duration_since(UNIX_EPOCH)
//                     .unwrap()
//                     .as_millis() as i64,
//             };
//             time
//         }
//     }
// }
