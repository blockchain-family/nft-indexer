use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use std::str::FromStr;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t_address")]
pub struct Address(pub String);

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t_uri")]
pub struct Uri(pub String);

#[derive(Copy, Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "event_type", rename_all = "snake_case")]
pub enum EventType {
    AuctionDeployed,
    AuctionCreated,
    AuctionActive,
    AuctionDeclined,
    AuctionBidPlaced,
    AuctionBidDeclined,
    AuctionCancelled,
    AuctionComplete,

    DirectBuyDeployed,
    DirectBuyDeclined,
    DirectBuyStateChanged,

    DirectSellDeployed,
    DirectSellDeclined,
    DirectSellStateChanged,

    NftOwnerChanged,
    NftManagerChanged,

    NftCreated,
    NftBurned,

    MarketFeeDefaultChanged,
    MarketFeeChanged,
    AddCollectionRules,
    RemoveCollectionRules,

    OwnershipTransferred,
}

impl sqlx::postgres::PgHasArrayType for EventType {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_event_type")
    }
}

#[derive(Copy, Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "event_category", rename_all = "snake_case")]
pub enum EventCategory {
    Auction,
    DirectBuy,
    DirectSell,
    Nft,
    Collection,
    Common,
}

impl sqlx::postgres::PgHasArrayType for EventCategory {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_event_category")
    }
}

#[derive(Clone, Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "auction_status", rename_all = "snake_case")]
pub enum AuctionStatus {
    Created = 0,
    Active,
    Completed,
    Cancelled,
    Expired,
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

#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "nft_price_source", rename_all = "camelCase")]
pub enum NftPriceSource {
    AuctionBid = 0,
    DirectBuy,
    DirectSell,
}

impl sqlx::postgres::PgHasArrayType for NftPriceSource {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_nft_price_source")
    }
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

impl From<u8> for DirectSellState {
    fn from(state: u8) -> Self {
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

impl From<u8> for DirectBuyState {
    fn from(state: u8) -> Self {
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

impl From<u8> for AuctionStatus {
    fn from(state: u8) -> Self {
        match state {
            0 => Self::Created,
            1 => Self::Active,
            2 => Self::Completed,
            3 => Self::Cancelled,
            _ => panic!("Unknown state of Auction"),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Nft {
    pub address: Address,
    pub collection: Option<Address>,
    pub owner: Option<Address>,
    pub manager: Option<Address>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub burned: bool,
    pub updated: NaiveDateTime,
    pub owner_update_lt: i64,
    pub manager_update_lt: i64,
}

#[derive(Clone, Debug)]
pub struct NftMeta {
    pub nft: Address,
    pub meta: serde_json::Value,
    pub updated: NaiveDateTime,
}

#[derive(Default, Clone, Debug)]
pub struct NftCollection {
    pub address: Address,
    pub owner: Address,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub logo: Option<Uri>,
    pub wallpaper: Option<Uri>,
    pub total_price: BigDecimal,
    pub max_price: BigDecimal,
}

#[derive(Default, Clone, Debug)]
pub struct NftAuction {
    pub address: Address,
    pub nft: Option<Address>,
    pub wallet_for_bids: Option<Address>,
    pub price_token: Option<Address>,
    pub start_price: Option<BigDecimal>,
    pub closing_price_usd: Option<BigDecimal>,
    pub min_bid: Option<BigDecimal>,
    pub max_bid: Option<BigDecimal>,
    pub status: Option<AuctionStatus>,
    pub created_at: Option<NaiveDateTime>,
    pub finished_at: Option<NaiveDateTime>,
    pub tx_lt: i64,
}

#[derive(Default, Clone, Debug)]
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
    pub sell_price_usd: Option<BigDecimal>,
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
    pub buy_price_usd: Option<BigDecimal>,
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

#[derive(Clone, Debug)]
pub struct NftAttribute {
    pub nft: Address,
    pub collection: Option<Address>,
    pub raw: serde_json::Value,
    pub trait_type: String,
    pub value: Option<serde_json::Value>,
}

impl NftAttribute {
    pub fn new(nft: Address, collection: Option<Address>, raw: serde_json::Value) -> Self {
        let trait_type = raw
            .get("trait_type")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(str::to_string)
            .unwrap_or_default();

        let value = if let Some(value) = raw.get("display_value").cloned() {
            Some(value)
        } else {
            raw.get("value").cloned()
        };

        Self {
            nft,
            collection,
            raw,
            trait_type,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventRecord {
    pub event_category: EventCategory,
    pub event_type: EventType,

    pub address: Address,
    pub created_lt: i64,
    pub created_at: i64,
    pub message_hash: String,
    pub nft: Option<Address>,
    pub collection: Option<Address>,

    pub raw_data: serde_json::Value,
}

#[derive(Debug, sqlx::Type, Deserialize, Clone, Copy)]
#[sqlx(type_name = "bc_name", rename_all = "snake_case")]
pub enum BcName {
    Everscale,
    Venom,
}

pub struct NftCreateDecoded {
    pub address: String,
    pub collection: String,
    pub owner: String,
    pub manager: String,
    pub updated: NaiveDateTime,
    pub owner_update_lt: i64,
    pub manager_update_lt: i64,
}

pub struct NftBurnedDecoded {
    pub address: String,
    pub owner: String,
    pub manager: String,
}

pub struct AddressChangedDecoded {
    pub id_address: String,
    pub new_address: String,
    pub timestamp: i64,
}

pub struct AuctionActiveDecoded {
    pub address: String,
    pub nft: String,
    pub wallet_for_bids: String,
    pub price_token: String,
    pub start_price: BigDecimal,
    pub min_bid: BigDecimal,
    pub created_at: i64,
    pub finished_at: i64,
    pub tx_lt: i64,
}

pub struct AuctionBidDecoded {
    pub address: String,
    pub bid_value: BigDecimal,
    pub next_value: BigDecimal,
    pub buyer: String,
    pub created_at: i64,
    pub tx_lt: i64,
    pub declined: bool,
}

pub struct AuctionCompleteDecoded {
    pub address: String,
    pub max_bid: BigDecimal,
}

pub struct AuctionCancelledDecoded {
    pub address: String,
}
