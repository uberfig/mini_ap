use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // pub database_url: String,
    pub instance_domain: String,
    pub bind_address: String,
    pub contact_email: String,
    pub port: u16,

    pub pg_user: String,
    pub pg_password: String,
    pub pg_host: String,
    pub pg_port: u16,
    pub pg_dbname: String,
}
