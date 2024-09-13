use serde::{Deserialize, Serialize};
use serde::Deserializer;
use url::Url;

use super::{
    content_format::{ImageContentFormat, TextContentFormat},
    public_key::PublicKey,
};

/// Users are identified by their id property, which is unique within the instance.
///
/// https://versia.pub/entities/user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: UserType,
    pub uri: Url,
    pub created_at: String,

    /// The user's avatar. Must be an image format (image/*).
    pub avatar: Option<ImageContentFormat>,
    /// Short description of the user. Must be text format (text/*).
    pub bio: Option<TextContentFormat>,
    /// Display name, as shown to other users.
    /// May contain emojis and any Unicode character.
    pub display_name: Option<String>,
    /// Custom key/value pairs. For example, metadata like socials or pronouns.
    /// Must be text format (text/*).
    pub fields: Option<Vec<Field>>,
    /// Alpha-numeric username. Must be unique within the instance.
    /// **Must** be treated as changeable by the user.
    ///
    /// Can only contain the following characters:
    /// - `a-z` (lowercase),
    /// - `0-9`, `_` and `-`
    /// - Should be limited to reasonable lengths.
    #[serde(deserialize_with = "de_username")]
    pub username: String,
    /// A header image for the user's profile.
    /// Also known as a cover photo or a banner.
    /// Must be an image format (image/*).
    pub header: Option<ImageContentFormat>,
    /// see [`PublicKey`]
    pub public_key: PublicKey,
    /// If true, the user must approve any new followers manually.
    /// If false, followers are automatically approved.
    /// This does not affect federation, and is meant to be used for
    /// clients to display correct UI. Defaults to false.
    #[serde(default = "default_false")]
    pub manually_approves_followers: bool,
    /// User consent to be indexed by search engines. If false, the
    /// user's profile should not be indexed. Defaults to true.
    #[serde(default = "default_true")]
    pub indexable: bool,
    /// The user's federation inbox. Refer to the federation documentation.
    /// Some instances may also have a shared inbox.
    /// Refer to [Instance Metadata](https://versia.pub/entities/instance-metadata)
    /// for more information.
    pub inbox: Url,
    /// Collections related to the user.
    /// Must contain at least `outbox`, `followers`, `following`, and `featured`.
    pub collections: UserCollections,
    // pub extensions: Extensions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    User,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn de_username<'de, D>(deserializer: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    let v = String::deserialize(deserializer)?;
    /* identical to check_above_2 body */
    todo!()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCollections {
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,
    pub featured: Url,

    #[serde(rename = "pub.versia:likes/Dislikes")]
    pub pub_versia_likes_dislikes: Option<Url>,
    #[serde(rename = "pub.versia:likes/Likes")]
    pub pub_versia_likes_likes: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub key: TextContentFormat,
    pub value: TextContentFormat,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Extensions {
//     #[serde(rename = "pub.versia:custom_emojis")]
//     pub pub_versia_custom_emojis: PubVersiaCustomEmojis,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PubVersiaCustomEmojis {
//     pub emojis: Vec<Value>,
// }
