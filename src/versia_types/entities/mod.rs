use change_follow::ChangeFollowing;
use follow_response::FollowResponse;
use group::Group;
use instance_metadata::InstanceMetadata;
use notes::Note;
use serde::{Deserialize, Serialize};
use user::User;

pub mod change_follow;
pub mod delete;
pub mod follow_response;
pub mod group;
pub mod instance_metadata;
pub mod notes;
pub mod public_key;
pub mod user;

/// array of a contigous type of entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityArray {
    FollowResponse(Vec<FollowResponse>),
    ChangeFollowing(Vec<ChangeFollowing>),
    Group(Vec<Group>),
    InstanceMetadata(Vec<InstanceMetadata>),
    Note(Vec<Note>),
    User(Vec<User>),
}
