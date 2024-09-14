use serde::{Deserialize, Serialize};
use url::Url;
use super::serde_fns::{serialize_time, deserialize_time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FollowType {
    /// https://versia.pub/entities/follow
    Follow,
    /// https://versia.pub/entities/unfollow
    Unfollow,
}

/// the author requests to follow or unfollow the followee
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeFollowing {
    #[serde(rename = "type")]
    pub type_field: FollowType,
    pub id: String,
    pub author: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub followee: Url,
}
