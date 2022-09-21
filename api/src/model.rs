use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct Contract {
    pub address: Address,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<Address>,
}


#[derive(Debug, Clone, Serialize)]
pub struct Price {
    #[serde(rename = "priceToken")]
    pub token: Address,

    pub price: String,

    #[serde(rename = "usdPrice")]
    pub usd_price: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NFTPrice {
    #[serde(flatten)]
    pub price: Price,
    pub ts: usize,
}


#[derive(Debug, Clone, Serialize)]
pub struct NFT {
    #[serde(flatten)]
    pub contract: Contract,
    
    pub image: String,
    pub collection: Contract,
    pub attributes: Option<Vec<serde_json::Value>>,

    #[serde(rename = "currentPrice")]
    pub current_price: Option<Price>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<Price>,

    pub auction: Option<Address>,
    pub forsale: Option<Address>,
    pub manager: Option<Address>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Collection {
    #[serde(flatten)]
    pub contract: Contract,
    
    pub verified: Option<bool>,

    #[serde(rename = "createdAt")]
    pub created_at: usize,

    pub wallpaper: Option<String>,
    pub logo: Option<String>,

    #[serde(rename = "ownersCount")]
    pub owners_count: usize,

    #[serde(rename = "lowestPrice")]
    pub lowest_price: Option<Price>,

    #[serde(rename = "totalPrice")]
    pub total_price: Option<Price>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub id: i64,

    #[serde(rename = "type")]
    pub typ: storage::tables::EventType,
    pub cat: storage::tables::EventCategory,
    pub address: String,
    pub ts: usize,
    pub args: Option<serde_json::Value>,
}

