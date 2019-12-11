
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub port_tls: u16,
}

pub fn default_config() -> config::Config {
    let mut config = config::Config::default();
    config
        .set_default("host", "127.0.0.1").unwrap()
        .set_default("port", 80).unwrap()
        .set_default("port_tls", 443).unwrap();
    config
}

pub fn get_config() -> ServerConfig {
    let path = match std::env::var_os("ACEERI_SERVER_CONFIG") {
        Some(s) => s.into_string().expect("failed to convert config OsString to String"),
        None => "/etc/server_config.yaml".to_owned(),
    };

    let mut config = default_config();
    // TODO: error handling here, just ignore errors if we can't find a config.
    config.merge(config::File::with_name(&path)).unwrap();
    config.merge(config::Environment::with_prefix("ACEERI")).unwrap();
    config.try_into::<ServerConfig>().unwrap()
}
