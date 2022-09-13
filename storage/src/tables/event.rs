use chrono::{DateTime, Utc};
use crate::types::Address;

#[derive(Debug)]
pub struct Event {
    pub id: i64,
    pub address: Address,
    pub nft: Option<Address>,
    pub collection: Option<Address>,
    pub event_cat: EventCategory,
    pub event_type: EventType,
    pub created_at: DateTime<Utc>,
    pub created_lt: DateTime<Utc>,
    pub checked: bool,
    pub args: serde_json::Value,
}

#[derive(sqlx::Type, Debug)]
pub enum EventCategory {
    #[sqlx(rename = "auction")]
    Auction,
    #[sqlx(rename = "direct_buy")]
    DirectBuy,
    #[sqlx(rename = "direct_sell")]
    DirectSell,
}

#[derive(sqlx::Type, Debug)]
pub enum EventType {
    auction_deployed,
    auction_deployed,
    auction_created,
    auction_ownership_transferred,
    auction_active,
    auction_declined,
    auction_bid_placed,
    auction_bid_declined,
    auction_cancelled,
    auction_complete,

    direct_buy_deployed,
    direct_buy_declined,
    direct_buy_ownership_transferred,
    direct_buy_state_changed,

    direct_sell_deployed,
    direct_sell_declined,
    direct_sell_ownership_transferred,
    direct_sell_state_changed,
}