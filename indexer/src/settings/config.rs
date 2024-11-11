use indexer_repo::types::BcName;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub kafka_topic: String,
    pub kafka_consumer_group: String,
    pub states_rpc_endpoints: Vec<Url>,
    pub kafka_settings: HashMap<String, String>,
    pub server_api_url: String,
    pub terminate_open_connections: Option<bool>,
    pub jrpc_req_latency_millis: u64,
    pub bc_name: BcName,
    pub dex_host_url: String,
    pub idle_after_price_loop_sec: u64,
    pub idle_after_meta_loop_sec: u64,
    pub price_update_frequency_sec: u64,
    pub is_need_cert_for_kafka: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let mut conf_builder = config::Config::builder().add_source(
            config::Environment::default()
                .separator("__")
                .list_separator(",")
                .with_list_parse_key("states_rpc_endpoints")
                .try_parsing(true),
        );
        if std::path::Path::new("Settings.toml").exists() {
            conf_builder = conf_builder.add_source(config::File::with_name("./Settings.toml"));
        }

        let mut conf = conf_builder
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .unwrap_or_else(|e| panic!("Error parsing config: {}", e));

        conf.kafka_settings = conf
            .kafka_settings
            .into_iter()
            .map(|(k, v)| (k.replace('_', "."), v))
            .collect();

        conf
    }
}
