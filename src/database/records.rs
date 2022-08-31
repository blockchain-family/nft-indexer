use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use nekoton_abi::transaction_parser::ExtractedOwned;
use sqlx::{postgres::PgQueryResult, PgPool};
use ton_block::MsgAddressInt;

pub trait Build {
    fn build_record(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized;

    fn get_nft(&self) -> Option<MsgAddressInt> {
        None
    }
}

#[async_trait]
pub trait Put {
    async fn put_record(&self, _pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(PgQueryResult::default())
    }
}

// TODO: Make one struct OwnerShipTransferred?

/// AuctionRootTip3 events

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionDeployedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub offer_address: String,

    pub collection: String,
    pub nft_owner: String,
    pub nft: String,
    pub offer: String,
    pub price: BigDecimal,
    pub auction_duration: BigDecimal,
    pub deploy_nonce: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionDeclinedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub nft_owner: String,
    pub data_address: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionOwnershipTransferredRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub old_owner: String,
    pub new_owner: String,
}

/// AuctionTip3 events

// TODO: AuctionCreated?

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionActiveRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub auction_subject: String,
    pub subject_owner: String,
    pub payment_token_root: String,
    pub wallet_for_bids: String,
    pub start_time: i128,
    pub duration: i128,
    pub finish_time: i128,
    pub now_time: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BidPlacedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub buyer_address: String,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BidDeclinedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub buyer_address: String,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionCompleteRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub buyer_address: String,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionCancelledRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,
}

/// FactoryDirectBuy events

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectBuyDeployedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub direct_buy_address: String,
    pub sender: String,
    pub token_root: String,
    pub nft: String,
    pub nonce: i128,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectBuyDeclinedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub sender: String,
    pub token_root: String,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectBuyOwnershipTransferredRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub old_owner: String,
    pub new_owner: String,
}

/// FactoryDirectSell events

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectSellDeployedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub _direct_sell_address: String,
    pub sender: String,
    pub payment_token: String,
    pub nft: String,
    pub _nonce: i128,
    pub price: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectSellDeclinedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub sender: String,
    pub _nft_address: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectSellOwnershipTransferredRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub old_owner: String,
    pub new_owner: String,
}

/// DirectBuy events

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectBuyStateChangedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: String,
    pub creator: String,
    pub spent_token: String,
    pub nft: String,
    pub _time_tx: i128,
    pub _price: BigDecimal,
    pub spent_wallet: String,
    pub status: i16,
    pub sender: String,
    pub start_time_buy: i128,
    pub duration_time_buy: i128,
    pub end_time_buy: i128,
}

/// DirectSell events

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectSellStateChangedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: String,
    pub creator: String,
    pub token: String,
    pub nft: String,
    pub _time_tx: i128,
    pub start: i128,
    pub end: i128,
    pub _price: BigDecimal,
    pub wallet: String,
    pub status: i16,
    pub sender: String,
}
