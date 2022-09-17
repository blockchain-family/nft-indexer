use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(type_name = "t_address")]
pub struct Address(pub String);

#[derive(Clone, Debug, Serialize, sqlx::Type)]
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

    NftOwnerChanged,
    NftManagerChanged,

    CollectionOwnershipTransferred,

    NftCreated,
    NftBurned,
}

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "event_category", rename_all = "snake_case")]
pub enum EventCategory {
    Auction,
    DirectBuy,
    DirectSell,
    Nft,
    Collection,
}

impl From<String> for Address {
    fn from(str_address: String) -> Self {
        Address(str_address)
    }
}

impl From<&str> for Address {
    fn from(str_address: &str) -> Self {
        Address(str_address.to_string())
    }
}

pub struct Nft {
    pub address: Address,
    pub collection: Option<Address>,
    pub owner: Option<Address>,
    pub manager: Option<Address>,
    pub name: String,
    pub description: String,
    pub burned: bool,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}

pub struct NftMeta {
    pub nft: Address,
    pub meta: serde_json::Value,
    pub updated: NaiveDateTime,
}
