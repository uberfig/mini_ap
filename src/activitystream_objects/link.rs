use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkType {
    Link,
    /// A specialized Link that represents an @mention.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mention
    Mention,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(rename = "type")]
    pub type_field: LinkType,

    pub href: Url,
    pub hreflang: Option<String>,
    pub media_type: String,
    pub name: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub preview: Option<String>, //TODO
    pub rel: Option<String>,     //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LinkSimpleOrExpanded {
    Simple(Url),
    Expanded(Link),
}

impl LinkSimpleOrExpanded {
    pub fn get_id(&self) -> &Url {
        match self {
            LinkSimpleOrExpanded::Simple(x) => x,
            LinkSimpleOrExpanded::Expanded(x) => &x.href,
        }
    }
}
