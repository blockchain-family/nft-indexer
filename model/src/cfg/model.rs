use serde::Deserialize;
use super::{DatabaseConfig, AuctionsConfig};


#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    #[serde(default)]
    pub database: DatabaseConfig,


    #[serde(default)]
    pub auctions: AuctionsConfig,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            database: DatabaseConfig::default(),
            auctions: AuctionsConfig::default(),
        }
    }
}
