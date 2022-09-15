use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use storage::types::Address;


#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub tokens: Vec<Token>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Token {
    #[serde(rename = "chainId")]
    pub chain_id: usize,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub vendor: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub decimals: usize,
    pub verified: bool,
}


pub async fn load_tokens() -> anyhow::Result<Arc<HashMap<Address, Token>>> {
    let resp = reqwest::get("https://raw.githubusercontent.com/broxus/ton-assets/master/manifest.json")
        .await?
        .json::<Manifest>()
        .await?;
    let mut map = HashMap::new();
    for token in resp.tokens {
        map.insert(token.address.clone(), token);
    }
    Ok(Arc::new(map))
}
