use serde::Deserialize;

fn default_contracts() -> Vec<String> {
    vec![String::from("postgresql://localhost/nft_indexer")]
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuctionsConfig {
    #[serde(default = "default_contracts")]
    pub contracts: Vec<String>,

}

impl Default for AuctionsConfig {
    fn default() -> Self {
        AuctionsConfig {
            contracts: default_contracts(),
        }
    }
}
