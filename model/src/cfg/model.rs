use serde::Deserialize;
use super::{DatabaseConfig, AuctionsConfig};


#[derive(Debug, Deserialize, Clone, Default)]
pub struct ModelConfig {
    #[serde(default)]
    pub database: DatabaseConfig,


    #[serde(default)]
    pub auctions: AuctionsConfig,
}
