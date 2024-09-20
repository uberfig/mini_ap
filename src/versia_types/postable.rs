use serde::{Deserialize, Serialize};

use super::{entities::notes::Note, extensions::share::Share};

/// entities that can be "posted" by a user, ie what can be
/// in a users timeline. includes notes, shares, and polls
///
/// note that polls are contained within extensions to note
/// and not their own type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Postable {
    Note(Box<Note>),
    Share(Share),
}

impl Postable {
    pub fn get_user(&self) -> &url::Url {
        match self {
            Postable::Note(note) => &note.author,
            Postable::Share(share) => &share.author,
        }
    }
    pub fn get_id(&self) -> &str {
        match self {
            Postable::Note(note) => &note.id,
            Postable::Share(share) => &share.id,
        }
    }
}
