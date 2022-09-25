use chrono::NaiveDateTime;
use sqlx::types::BigDecimal;

pub type Address = String;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NFT {
    pub address: Address,
    pub collection: Option<Address>,
    pub owner: Option<Address>,
    pub manager: Option<Address>,
    pub name: String,
    pub burned: bool,
    pub description: String,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NFTMetadata {
    pub nft: Address,
    pub updated: NaiveDateTime,
    pub meta: serde_json::Value,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NFTCollection {
    pub address: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub verified: bool,
    pub wallpaper: Option<String>,
    pub logo: Option<String>,
    pub owners_count: Option<i32>,
    pub max_price: Option<BigDecimal>,
    pub total_price: Option<BigDecimal>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NFTDetails {
    pub address: Option<Address>,
    pub collection: Option<Address>,
    pub collection_owner: Option<Address>,
    pub collection_name: Option<String>,
    pub collection_description: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub owner: Option<Address>,
    pub manager: Option<Address>,
    pub name: Option<String>,
    pub burned: Option<bool>,
    pub description: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub tx_lt: Option<i64>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Auction {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: NaiveDateTime,
    pub finished_at: NaiveDateTime,
    pub price_token: Address,
    pub start_price: i64,
    pub max_bid: Option<BigDecimal>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuctionBid {
    pub auction: Address,
    pub nft: Address,
    pub owner: Address,
    pub created: NaiveDateTime,
    pub price_token: Address,
    pub price: BigDecimal,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DirectBuy {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
    pub price_token: Address,
    pub price: BigDecimal,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DirectSale {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: NaiveDateTime,
    pub price_token: Address,
    pub price: BigDecimal,
}





