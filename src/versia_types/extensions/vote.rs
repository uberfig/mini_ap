use crate::versia_types::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoteType {
    #[serde(rename = "pub.versia:polls/Vote")]
    Vote,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vote {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: VoteType,
    pub uri: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,

    pub author: Url,
    pub poll: Url,
    pub option: u64,
}
