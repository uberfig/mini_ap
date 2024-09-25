use serde::{Deserialize, Serialize};

use super::{note::Note, question::Question};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Postable {
    Question(Question),
    Note(Note),
}