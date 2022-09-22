use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::str::FromStr;


#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
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

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "auction_status", rename_all = "snake_case")]
pub enum AuctionStatus {
    Active,
    Cancelled,
    Completed,
}

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "direct_sell_state", rename_all = "snake_case")]
pub enum DirectSellState {
    Create = 0,
    AwaitNft,
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "direct_buy_state", rename_all = "snake_case")]
pub enum DirectBuyState {
    Create = 0,
    AwaitTokens,
    Active,
    Filled,
    Cancelled,
    Expired,
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

impl FromStr for Address {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address(s.to_string()))
    }
}

impl From<i16> for DirectSellState {
    fn from(state: i16) -> Self {
        match state {
            0 => Self::Create,
            1 => Self::AwaitNft,
            2 => Self::Active,
            3 => Self::Filled,
            4 => Self::Cancelled,
            5 => Self::Expired,
            _ => panic!("Unknown state of DirectSell"),
        }
    }
}

impl From<i16> for DirectBuyState {
    fn from(state: i16) -> Self {
        match state {
            0 => Self::Create,
            1 => Self::AwaitTokens,
            2 => Self::Active,
            3 => Self::Filled,
            4 => Self::Cancelled,
            5 => Self::Expired,
            _ => panic!("Unknown state of DirectBuy"),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct NftMeta {
    pub nft: Address,
    pub meta: serde_json::Value,
    pub updated: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct NftCollection {
    pub address: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub updated: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct NftAuction {
    pub address: Address,
    pub nft: Option<Address>,
    pub price_token: Option<Address>,
    pub start_price: Option<BigDecimal>,
    pub max_bid: Option<BigDecimal>,
    pub status: Option<AuctionStatus>,
    pub created_at: Option<NaiveDateTime>,
    pub finished_at: Option<NaiveDateTime>,
    pub tx_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftAuctionBid {
    pub auction: Address,
    pub buyer: Address,
    pub price: BigDecimal,
    pub declined: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct NftDirectSell {
    pub address: Address,
    pub nft: Address,
    pub price_token: Address,
    pub price: BigDecimal,
    pub state: DirectSellState,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftDirectBuy {
    pub address: Address,
    pub nft: Address,
    pub price_token: Address,
    pub price: BigDecimal,
    pub state: DirectBuyState,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}
