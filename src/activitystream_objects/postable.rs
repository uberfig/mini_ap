use serde::{Deserialize, Serialize};
use url::Url;

use super::{inboxable::InboxableVerifyErr, note::Note, question::Question};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Postable {
    Question(Question),
    Note(Note),
}

impl Postable {
    pub fn id(&self) -> &Url {
        match self {
            Postable::Question(question) => &question.id,
            Postable::Note(note) => &note.id,
        }
    }
    pub fn actor(&self) -> &Url {
        match self {
            Postable::Question(question) => &question.actor,
            Postable::Note(note) => &note.attributed_to,
        }
    }
    pub fn verify(self, origin_domain: &str) -> Result<Self, InboxableVerifyErr> {
        if self.id().domain().ne(&Some(origin_domain))
            || self.actor().domain().ne(&Some(origin_domain))
        {
            return Err(InboxableVerifyErr::ForgedAttribution);
        }
        Ok(self)
    }
}
