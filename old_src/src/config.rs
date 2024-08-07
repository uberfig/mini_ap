use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub instance_domain: String,
    pub bind_address: String,
    pub contact_email: String,
    pub port: u16,
}
