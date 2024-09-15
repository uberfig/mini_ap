use crate::versia_types::structures::content_format::ImageContentFormat;
use regex::Regex;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Emoji {
    #[serde(deserialize_with = "deserialize_name")]
    #[serde(serialize_with = "serialize_name")]
    pub name: EmojiName,
    pub content: ImageContentFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiName {
    pub shortcode: String,
    pub identifyer: char,
}

fn deserialize_name<'de, D>(deserializer: D) -> Result<EmojiName, D::Error>
where
    D: Deserializer<'de>,
{
    let input = String::deserialize(deserializer)?;
    if input.is_empty() {
        return Err(D::Error::custom("emoji name is empty"));
    }
    let mut chars = input.chars();

    let Some(first) = chars.next() else {
        return Err(D::Error::custom("emoji first identifyer missing"));
    };
    if first.is_ascii_whitespace() {
        return Err(D::Error::custom("emoji identifyer is whitespace"));
    }
    if first.is_ascii_alphanumeric() {
        return Err(D::Error::custom("emoji identifyer is alphanumeric"));
    }

    let mut shortcode: Vec<char> = chars.collect();
    let Some(last) = shortcode.pop() else {
        return Err(D::Error::custom("emoji last identifyer missing"));
    };
    if first.ne(&last) {
        return Err(D::Error::custom("emoji first identifyers don't match"));
    }
    if shortcode.is_empty() {
        return Err(D::Error::custom("emoji shortcode missing"));
    }

    let shortcode: String = shortcode.into_iter().collect();

    let re = Regex::new(r"[^\da-zA-Z_-]").unwrap();

    if re.is_match(&shortcode) {
        return Err(D::Error::custom(
            "shortcode contains invalid characters",
        ));
    }
    
    Ok(EmojiName { shortcode, identifyer: first })
}

pub fn serialize_name<S>(x: &EmojiName, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{}{}{}", x.identifyer, x.shortcode, x.identifyer))
}