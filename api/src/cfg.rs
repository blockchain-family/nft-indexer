use serde::Deserialize;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use config::{self, Environment, ConfigError};
use storage::cfg::DatabaseConfig;


fn default_http_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8080))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    #[serde(default = "default_http_address")]
    pub http_address: SocketAddr,

    pub database: DatabaseConfig,
}

impl ApiConfig {
    pub fn new() -> Result<ApiConfig, ConfigError> {
        let prefix = std::env::var("PREFIX").unwrap_or_else(|_| String::from("indexer_api"));
        config::Config::builder()
            .add_source(config::File::with_name("./Settings.toml"))
            .add_source(Environment::with_prefix(&prefix).separator("_"))
            .build()?
            .try_deserialize()
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig { 
            http_address: default_http_address(),
            database: DatabaseConfig::default(),
        }
    }
}

