use super::{AuctionsConfig, DatabaseConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ModelConfig {
    #[serde(default)]
    pub database: DatabaseConfig,

    #[serde(default)]
    pub auctions: AuctionsConfig,
}
