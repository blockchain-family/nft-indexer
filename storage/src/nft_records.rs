use crate::types::Address;
use sqlx::types::chrono;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NftCollection {
    pub address: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nft {
    pub address: Address,
    pub collection: Address,
    pub owner: Address,
    pub manager: Address,
    pub name: String,
    pub description: String,
    pub data: serde_json::Value,
    pub updated: chrono::NaiveDateTime,
}
