use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

    MetadataUpdated,
    NftMetadataUpdated,
    CollectionMetadataUpdated,
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

#[derive(Debug, sqlx::Type, Deserialize, Clone, Copy)]
#[sqlx(type_name = "bc_name", rename_all = "snake_case")]
pub enum BcName {
    Everscale,
    Tycho,
    Venom,
}

pub struct NftCollection {
    pub address: String,
    pub nft_first_mint: NaiveDateTime,
}

#[derive(Default, Clone, Debug)]
pub struct NftCollectionMeta {
    pub address: String,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub updated: NaiveDateTime,
    pub logo: Option<String>,
    pub wallpaper: Option<String>,
    pub royalty: Option<serde_json::Value>,
}

pub mod decoded {
    use chrono::NaiveDateTime;
    use sqlx::types::BigDecimal;

    use crate::types::{DirectBuyState, DirectSellState, EventCategory, EventType, NftPriceSource};

    #[derive(Clone, Debug)]
    pub struct NftPriceHistory {
        pub source: String,
        pub source_type: NftPriceSource,
        pub created_at: NaiveDateTime,
        pub price: BigDecimal,
        pub price_token: String,
        pub usd_price: Option<BigDecimal>,
        pub nft: String,
        pub collection: String,
    }

    #[derive(Clone, Debug)]
    pub struct EventRecord {
        pub event_category: EventCategory,
        pub event_type: EventType,

        pub address: String,
        pub created_lt: i64,
        pub created_at: i64,
        pub message_hash: String,
        pub nft: Option<String>,
        pub collection: Option<String>,

        pub raw_data: serde_json::Value,
    }

    #[derive(Debug, Clone)]
    pub struct NftCreated {
        pub id: BigDecimal,
        pub address: String,
        pub collection: String,
        pub owner: String,
        pub manager: String,
        pub updated: NaiveDateTime,
        pub owner_update_lt: u64,
        pub manager_update_lt: u64,
    }

    #[derive(Debug, Clone)]
    pub struct NftBurned {
        pub address: String,
        pub owner: String,
        pub manager: String,
    }

    #[derive(Debug, Clone)]
    pub struct AddressChanged {
        pub id_address: String,
        pub new_address: String,
        pub logical_time: u64,
        pub timestamp: NaiveDateTime,
    }

    #[derive(Debug, Clone)]
    pub struct AuctionDeployed {
        pub address: String,
        pub root: String,
        pub nft: String,
        pub collection: String,
        pub nft_owner: String,
        pub tx_lt: i64,
    }

    #[derive(Debug, Clone)]
    pub struct AuctionActive {
        pub address: String,
        pub nft: String,
        pub wallet_for_bids: String,
        pub price_token: String,
        pub start_price: BigDecimal,
        pub min_bid: BigDecimal,
        pub created_at: NaiveDateTime,
        pub finished_at: NaiveDateTime,
        pub tx_lt: i64,
    }

    #[derive(Debug, Clone)]
    pub struct AuctionBid {
        pub address: String,
        pub collection: String,
        pub nft: String,
        pub nft_owner: String,
        pub price_token: String,
        pub bid_value: BigDecimal,
        pub next_value: BigDecimal,
        pub buyer: String,
        pub created_at: NaiveDateTime,
        pub tx_lt: i64,
        pub declined: bool,
    }

    #[derive(Debug, Clone)]
    pub struct AuctionComplete {
        pub address: String,
        pub max_bid: BigDecimal,
    }

    #[derive(Debug, Clone)]
    pub struct AuctionCancelled {
        pub address: String,
    }

    #[derive(Debug, Clone)]
    pub struct CollectionFee {
        pub address: String,
        pub timestamp: NaiveDateTime,
        pub numerator: Option<i32>,
        pub denominator: Option<i32>,
    }

    #[derive(Debug, Clone)]
    pub struct DirectBuy {
        pub address: String,
        pub root: String,
        pub nft: String,
        pub collection: Option<String>,
        pub price_token: String,
        pub price: BigDecimal,
        pub buyer: String,
        pub finished_at: Option<NaiveDateTime>,
        pub expired_at: NaiveDateTime,
        pub state: DirectBuyState,
        pub created: NaiveDateTime,
        pub updated: NaiveDateTime,
        pub tx_lt: i64,
    }

    #[derive(Debug, Clone)]
    pub struct DirectSell {
        pub address: String,
        pub root: String,
        pub nft: String,
        pub collection: Option<String>,
        pub price_token: String,
        pub price: BigDecimal,
        pub seller: String,
        pub finished_at: Option<NaiveDateTime>,
        pub expired_at: NaiveDateTime,
        pub state: DirectSellState,
        pub created: NaiveDateTime,
        pub updated: NaiveDateTime,
        pub tx_lt: i64,
    }

    #[derive(Debug, Clone)]
    pub struct OfferDeployed {
        pub address: String,
        pub root: String,
        pub created: NaiveDateTime,
    }

    #[derive(Debug, Clone)]
    pub struct SetRoyalty {
        pub address: String,
        pub denominator: i32,
        pub numerator: i32,
    }

    #[derive(Debug, Clone)]
    pub struct MetadataUpdated {
        pub address: String,
        pub tx_lt: u64,
        pub timestamp: NaiveDateTime,
    }

    #[derive(Debug, Clone)]
    pub struct NftMetadataUpdated {
        pub collection: String,
        pub tx_lt: u64,
        pub timestamp: NaiveDateTime,
    }

    #[derive(Debug, Clone)]
    pub struct CollectionMetadataUpdated {
        pub address: String,
        pub tx_lt: u64,
        pub timestamp: NaiveDateTime,
    }
}
