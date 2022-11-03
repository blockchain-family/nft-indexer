use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub kafka_topic: String,
    pub kafka_consumer_group: String,
    pub kafka_reset: Option<bool>,
    pub states_rpc_endpoints: Vec<String>,
    pub kafka_settings: HashMap<String, String>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let mut conf = config::Config::new();
        if std::path::Path::new(path).exists() {
            conf.merge(config::File::with_name(path)).unwrap();
        }
        conf.try_into::<Config>()
            .unwrap_or_else(|e| panic!("Error parsing config: {}", e))
    }
}

pub fn trusted_auction_roots() -> [&'static str; 1] {
    ["0:a9adf011a072ae8efac041aa4cdc046f973e275208eba607543ac08690ebab3c"]
}

pub fn trusted_direct_buy_factories() -> [&'static str; 1] {
    ["0:41ffcebbdc210ed279edaeae705be303a63610eae03bba18014e9945a0f34039"]
}

pub fn trusted_direct_sell_factories() -> [&'static str; 1] {
    ["0:0cd020840266c5ee7ad575787379c4894f9d494c00d0fc00e22889d721df3f8c"]
}
