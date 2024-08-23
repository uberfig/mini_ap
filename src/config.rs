use config::ConfigError;
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

pub fn get_config() -> Result<Config, ConfigError> {
    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("ap_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::default())
        .build();

    let settings = match settings {
        Ok(x) => x,
        Err(x) => {
            return Err(x);
        }
    };

    let config = match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(error) => {
            return Err(error);
        }
    };
    Ok(config)
}
