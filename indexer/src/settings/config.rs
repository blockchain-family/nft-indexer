use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub kafka_topic: String,
    pub kafka_consumer_group: String,
    pub kafka_reset: Option<bool>,
    pub states_rpc_endpoints: Vec<Url>,
    pub kafka_settings: HashMap<String, String>,
    pub reset: bool,
    pub trusted_auction_roots: Vec<String>,
    pub trusted_direct_buy_factories: Vec<String>,
    pub trusted_direct_sell_factories: Vec<String>,
    pub server_api_url: String,
    pub terminate_open_connections: Option<bool>,
    pub jrpc_req_latency_millis: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let conf = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("states_rpc_endpoints")
                    .with_list_parse_key("trusted_auction_roots")
                    .with_list_parse_key("trusted_direct_buy_factories")
                    .with_list_parse_key("trusted_direct_sell_factories")
                    .try_parsing(true),
            )
            .build()
            .unwrap();

        let mut conf = conf
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
