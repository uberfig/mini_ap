use serde::{Deserialize, Serialize};
use url::Url;

use super::content_format::ContentFormat;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoteTyoe {
    Note,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Microblog,
    Forum,
    Blog,
    Image,
    Video,
    Audio,
    Messaging,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Groups {
    /// The note is visible to anyone.
    Public,
    /// The note is visible only to the author's followers.
    Followers,
    /// The note is visible only to users on the author's instance.
    Local,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupType {
    Group(Url),
    Simple(Groups),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: NoteTyoe,
    pub uri: Url,
    pub created_at: String,
    /// Media attachments to the note. May be any format. Must be remote.
    pub attachments: Vec<ContentFormat>,
    /// URI of the User considered the author of the note.
    pub author: Url,
    /// Category of the note. Useful for clients to render 
    /// notes differently depending on their intended purpose.
    pub category: Option<Category>,
    /// The content of the note. Must be text format 
    /// (text/html, text/markdown, etc). Must not be remote.
    pub content: Option<ContentFormat>,
    /// Device used to post the note. Useful for functionality such as Twitter's "posted via" feature.
    pub device: Option<Device>,
    pub extensions: NoteExtensions,
    /// URI of a Group that the note is only visible in or one of the provided types
    /// 
    /// If not provided, the note is only visible to the author and those mentioned in the note.
    pub group: Option<GroupType>,
    /// Whether the note contains "sensitive content". 
    /// This can be used with [`Note::subject`] as a "content warning" feature.
    pub is_sensitive: Option<bool>,
    /// URIs of [Users](https://versia.pub/entities/user) that should be notified of the note. 
    /// Similar to Twitter's @ mentions. The note may also 
    /// contain mentions in the content, however only the 
    /// mentions in this field should trigger notifications.
    pub mentions: Vec<Url>,
    /// used with [`Note::is_sensitive`] as a "content warning" feature.
    pub subject: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub version: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteExtensions {
    #[serde(rename = "pub.versia:custom_emojis")]
    pub pub_versia_custom_emojis: PubVersiaCustomEmojis,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubVersiaCustomEmojis {
    // pub emojis: Vec<Value>,
}
