use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Event {
    pub id: i64,
    pub address: String,
    pub event_cat: EventCategory,
    pub event_type: EventType,
    pub created_at: i64,
    pub created_lt: i64,
    pub args: Option<serde_json::Value>,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "event_category")] 
pub enum EventCategory {
    #[sqlx(rename = "auction")]
    Auction,
    #[sqlx(rename = "direct_buy")]
    DirectBuy,
    #[sqlx(rename = "direct_sell")]
    DirectSell,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "event_type", rename_all = "snake_case")]
pub enum EventType {
    AuctionDeployed,
    AuctionCreated,
    AuctionOwnershipTransferred,
    AuctionActive,
    AuctionDeclined,
    AuctionBidPlaced,
    AuctionBidDeclined,
    AuctionCancelled,
    AuctionComplete,

    DirectBuyDeployed,
    DirectBuyDeclined,
    DirectBuyOwnershipTransferred,
    DirectBuyStateChanged,

    DirectSellDeployed,
    DirectSellDeclined,
    DirectSellOwnershipTransferred,
    DirectSellStateChanged,
}