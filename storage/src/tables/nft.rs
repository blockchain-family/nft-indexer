use chrono::{DateTime, Utc};
use crate::types::Address;

#[derive(Debug)]
pub struct NFT {
    pub address: Address,
    pub collection: Address,
    pub owner: Address,
    pub manager: Address,
    pub name: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NFTMetadata {
    pub id: i64,
    pub nft: Address,
    pub ts: DateTime<Utc>,
    pub meta: serde_json::Value,
}

#[derive(Debug)]
pub struct NFTCollection {
    pub address: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub verified: bool,
    pub wallpaper: String,
    pub logo: String,
    pub owners_count: usize,
}

#[derive(Debug)]
pub struct Auction {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub price_token: Address,
    pub start_price: i64,
    pub max_bid: Option<i64>,
}

#[derive(Debug)]
pub struct AuctionBid {
    pub auction: Address,
    pub nft: Address,
    pub owner: Address,
    pub created: DateTime<Utc>,
    pub price_token: Address,
    pub price: i64,
}

#[derive(Debug)]
pub struct Offer {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: DateTime<Utc>,
    pub expired_at: Option<DateTime<Utc>>,
    pub price_token: Address,
    pub price: i64,
}

#[derive(Debug)]
pub struct ForSale {
    pub address: Address,
    pub owner: Address,
    pub nft: Address,
    pub created: DateTime<Utc>,
    pub price_token: Address,
    pub price: i64,
}





