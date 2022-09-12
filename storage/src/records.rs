use crate::types::Address;
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use nekoton_abi::transaction_parser::ExtractedOwned;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, PgPool};
use ton_block::MsgAddressInt;

pub trait EventRecord {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized;

    fn get_nft(&self) -> Option<MsgAddressInt> {
        None
    }
}

#[async_trait]
pub trait DatabaseRecord {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NftMetadata {
    pub nft: Address,
    pub data: serde_json::Value,
}

/// AuctionRootTip3 events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub offer_address: Address,

    pub collection: Address,
    pub nft_owner: Address,
    pub nft: Address,
    pub offer: Address,
    pub price: BigDecimal,
    pub auction_duration: i64,
    pub deploy_nonce: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub nft_owner: Address,
    pub data_address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// AuctionTip3 events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionCreated {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub payment_token_root: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub now_time: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionActive {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub payment_token_root: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub now_time: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BidPlaced {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer_address: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BidDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer_address: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionComplete {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer_address: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionCancelled {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,
}

/// FactoryDirectBuy events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub direct_buy_address: Address,
    pub sender: Address,
    pub token_root: Address,
    pub nft: Address,
    pub nonce: BigDecimal,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub sender: Address,
    pub token_root: Address,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// FactoryDirectSell events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub _direct_sell_address: Address,
    pub sender: Address,
    pub payment_token: Address,
    pub nft: Address,
    pub _nonce: BigDecimal,
    pub price: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub sender: Address,
    pub _nft_address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// DirectBuy events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyStateChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: Address,
    pub creator: Address,
    pub spent_token: Address,
    pub nft: Address,
    pub _time_tx: i64,
    pub _price: BigDecimal,
    pub spent_wallet: Address,
    pub status: i16,
    pub sender: Address,
    pub start_time_buy: i64,
    pub duration_time_buy: i64,
    pub end_time_buy: i64,
}

/// DirectSell events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellStateChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: Address,
    pub creator: Address,
    pub token: Address,
    pub nft: Address,
    pub _time_tx: i64,
    pub start: i64,
    pub end: i64,
    pub _price: BigDecimal,
    pub wallet: Address,
    pub status: i16,
    pub sender: Address,
}
