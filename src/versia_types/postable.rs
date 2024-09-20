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
