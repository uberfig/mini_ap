use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The ContentFormat structure is used to represent content with metadata.
/// It supports multiple content types for the same file, such as a PNG
/// image and a WebP image.
///
/// https://versia.pub/structures/content-format
pub struct ContentFormat {
    #[serde(rename = "image/png")]
    pub image_png: ImagePng,
    #[serde(rename = "image/webp")]
    pub image_webp: ImageWebp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagePng {
    pub content: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageWebp {
    pub content: Url,
}
