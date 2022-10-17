use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t_address")]
pub struct Address(pub String);

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t_uri")]
pub struct Uri(pub String);

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "event_type", rename_all = "snake_case")]
pub enum EventType {
    AuctionDeployed,
    AuctionCreated,
    AuctionRootOwnershipTransferred,
    AuctionActive,
    AuctionDeclined,
    AuctionBidPlaced,
    AuctionBidDeclined,
    AuctionCancelled,
    AuctionComplete,

    DirectBuyDeployed,
    DirectBuyDeclined,
    FactoryDirectBuyOwnershipTransferred,
    DirectBuyStateChanged,

    DirectSellDeployed,
    DirectSellDeclined,
    FactoryDirectSellOwnershipTransferred,
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
    Created = 0,
    Active,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "direct_sell_state", rename_all = "snake_case")]
pub enum DirectSellState {
    Create = 0,
    AwaitNft,
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "direct_buy_state", rename_all = "snake_case")]
pub enum DirectBuyState {
    Create = 0,
    AwaitTokens,
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "nft_price_source", rename_all = "camelCase")]
pub enum NftPriceSource {
    AuctionBid = 0,
    DirectBuy,
    DirectSell,
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

impl<'a> From<&'a Address> for &'a String {
    fn from(address: &'a Address) -> Self {
        &address.0
    }
}

impl FromStr for Address {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address(s.to_string()))
    }
}

impl From<String> for Uri {
    fn from(str: String) -> Self {
        Uri(str)
    }
}

impl From<&str> for Uri {
    fn from(str: &str) -> Self {
        Uri(str.to_string())
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

impl From<i16> for AuctionStatus {
    fn from(state: i16) -> Self {
        match state {
            0 => Self::Created,
            1 => Self::Active,
            2 => Self::Completed,
            3 => Self::Cancelled,
            _ => panic!("Unknown state of Auction"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Nft {
    pub address: Address,
    pub collection: Option<Address>,
    pub owner: Option<Address>,
    pub manager: Option<Address>,
    pub name: Option<String>,
    pub description: Option<String>,
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
    pub name: Option<String>,
    pub description: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub logo: Option<Uri>,
    pub wallpaper: Option<Uri>,
}

#[derive(Clone, Debug)]
pub struct NftAuction {
    pub address: Address,
    pub nft: Option<Address>,
    pub wallet_for_bids: Option<Address>,
    pub price_token: Option<Address>,
    pub start_price: Option<BigDecimal>,
    pub min_bid: Option<BigDecimal>,
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
    pub next_bid_value: Option<BigDecimal>,
    pub declined: bool,
    pub created_at: NaiveDateTime,
    pub tx_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftDirectSell {
    pub address: Address,
    pub nft: Address,
    pub collection: Option<Address>,
    pub price_token: Address,
    pub price: BigDecimal,
    pub seller: Address,
    pub finished_at: Option<NaiveDateTime>,
    pub expired_at: NaiveDateTime,
    pub state: DirectSellState,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftDirectBuy {
    pub address: Address,
    pub nft: Address,
    pub collection: Option<Address>,
    pub price_token: Address,
    pub price: BigDecimal,
    pub buyer: Address,
    pub finished_at: Option<NaiveDateTime>,
    pub expired_at: NaiveDateTime,
    pub state: DirectBuyState,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub tx_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftPriceHistory {
    pub source: Address,
    pub source_type: NftPriceSource,
    pub created_at: NaiveDateTime,
    pub price: BigDecimal,
    pub price_token: Option<Address>,
    pub nft: Option<Address>,
    pub collection: Option<Address>,
}
