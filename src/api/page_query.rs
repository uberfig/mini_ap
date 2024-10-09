use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Page {
    pub page: u64,
}
