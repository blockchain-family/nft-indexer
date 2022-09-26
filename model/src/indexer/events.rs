use crate::indexer::{record_build_utils::*, traits::ContractEvent};
use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use nekoton_abi::transaction_parser::ExtractedOwned;
use serde::Serialize;
use sqlx::PgPool;
use std::{str::FromStr, sync::Arc};
use storage::{actions, traits::EventRecord, types::*};
use ton_abi::TokenValue::Tuple;
use ton_block::MsgAddressInt;
use transaction_consumer::TransactionConsumer;

/// AuctionRootTip3 events

#[derive(Clone, Serialize)]
pub struct AuctionDeployed {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct AuctionDeclined {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub nft_owner: Address,
    pub data_address: Address,
}

#[derive(Clone, Serialize)]
pub struct AuctionOwnershipTransferred {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct AuctionCreated {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct AuctionActive {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct BidPlaced {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct BidDeclined {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct AuctionComplete {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub seller: Address,
    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct AuctionCancelled {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,
}

/// FactoryDirectBuy events

#[derive(Clone, Serialize)]
pub struct DirectBuyDeployed {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct DirectBuyDeclined {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct DirectBuyOwnershipTransferred {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct DirectSellDeployed {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub direct_sell_address: Address,
    pub sender: Address,
    pub payment_token: Address,
    pub nft: Address,
    pub nonce: BigDecimal,
    pub price: BigDecimal,
}

#[derive(Clone, Serialize)]
pub struct DirectSellDeclined {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub sender: Address,
    pub _nft_address: Address,
}

#[derive(Clone, Serialize)]
pub struct DirectSellOwnershipTransferred {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct DirectBuyStateChanged {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

#[derive(Clone, Serialize)]
pub struct DirectSellStateChanged {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

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

// Nft events

#[derive(Clone, Serialize)]
pub struct NftOwnerChanged {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Serialize)]
pub struct NftManagerChanged {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_manager: Address,
    pub new_manager: Address,
}

// Collection events

#[derive(Clone, Serialize)]
pub struct CollectionOwnershipTransferred {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Serialize)]
pub struct NftCreated {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
    pub creator: Address,
}

#[derive(Clone, Serialize)]
pub struct NftBurned {
    #[serde(skip_serializing)]
    pub pool: PgPool,
    #[serde(skip_serializing)]
    pub consumer: Arc<TransactionConsumer>,

    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
}

impl ContractEvent for AuctionDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let offer_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "offerAddress")
            .ok_or_else(|| anyhow!("Couldn't find offerAddress token"))?
            .clone();

        let offer_info = event
            .tokens
            .iter()
            .find(|t| t.name == "offerInfo")
            .ok_or_else(|| anyhow!("Couldn't find offerInfo token"))?;
        let mut tokens = match &offer_info.value {
            Tuple(v) => Some(v.clone()),
            _ => None,
        }
        .ok_or_else(|| anyhow!("offerInfo token value is not tuple"))?;

        tokens.push(offer_address_token);

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_i64 = get_token_processor(&tokens, token_to_i64);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionDeployed {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            offer_address: to_address("offerAddress")?,

            collection: to_address("collection")?,
            nft_owner: to_address("nftOwner")?,
            nft: to_address("nft")?,
            offer: to_address("offer")?,
            price: to_big_decimal("price")?,
            auction_duration: to_i64("auctionDuration")?,
            deploy_nonce: to_big_decimal("deployNonce")?,
        })
    }
}

impl EventRecord for AuctionDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionDeployed
    }
}

impl ContractEvent for AuctionDeclined {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let nft_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nftOwner")
            .ok_or_else(|| anyhow!("Couldn't find nftOwner token"))?
            .clone();

        let data_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "dataAddress")
            .ok_or_else(|| anyhow!("Couldn't find dataAddress token"))?
            .clone();

        let tokens = vec![nft_owner_token, data_address_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(AuctionDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            nft_owner: to_address("nftOwner")?,
            data_address: to_address("dataAddress")?,
        })
    }
}

impl EventRecord for AuctionDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionDeclined
    }
}

impl ContractEvent for AuctionOwnershipTransferred {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(AuctionOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for AuctionOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionOwnershipTransferred
    }
}

impl ContractEvent for AuctionCreated {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let value0_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value0")
            .ok_or_else(|| anyhow!("Couldn't find value0 token"))?;
        let tokens = match &value0_token.value {
            Tuple(v) => Some(v),
            _ => None,
        }
        .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionCreated {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            payment_token_root: to_address("paymentTokenRoot")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
        })
    }
}

impl EventRecord for AuctionCreated {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionCreated
    }
}

impl ContractEvent for AuctionActive {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let value0_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value0")
            .ok_or_else(|| anyhow!("Couldn't find value0 token"))?;
        let tokens = match &value0_token.value {
            Tuple(v) => Some(v),
            _ => None,
        }
        .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionActive {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            payment_token_root: to_address("paymentTokenRoot")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
        })
    }
}

impl EventRecord for AuctionActive {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionActive
    }
}

impl AuctionActive {
    pub async fn upsert_auction(&self) -> Result<()> {
        let auction = NftAuction {
            address: self.address.clone(),
            nft: Some(self.auction_subject.clone()),
            price_token: Some(self.payment_token_root.clone()),
            start_price: Some(self._price.clone()),
            max_bid: Some(self._price.clone()),
            status: Some(AuctionStatus::Active),
            created_at: Some(NaiveDateTime::from_timestamp(self.start_time, 0)),
            finished_at: Some(NaiveDateTime::from_timestamp(self.finish_time, 0)),
            tx_lt: self.created_lt,
        };

        actions::upsert_auction(&auction, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) =
            actions::get_collection_by_nft(&self.auction_subject, &self.pool).await
        {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for BidPlaced {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(BidPlaced {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for BidPlaced {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionBidPlaced
    }
}

impl BidPlaced {
    pub async fn upsert_bid(&self) -> Result<()> {
        let bid = NftAuctionBid {
            auction: self.address.clone(),
            buyer: self.buyer.clone(),
            price: self.value.clone(),
            declined: false,
            created_at: NaiveDateTime::from_timestamp(self.created_at, 0),
        };

        actions::upsert_bid(&bid, &self.pool).await
    }

    pub async fn upsert_auction(&self) -> Result<()> {
        let auction = NftAuction {
            address: self.address.clone(),
            nft: None,
            price_token: None,
            start_price: None,
            max_bid: Some(self.value.clone()),
            status: None,
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };

        actions::upsert_auction(&auction, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) =
            actions::get_collection_by_auction(&self.address, &self.pool).await
        {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for BidDeclined {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(BidDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for BidDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionBidDeclined
    }
}

impl BidDeclined {
    pub async fn upsert_bid(&self) -> Result<()> {
        let bid = NftAuctionBid {
            auction: self.address.clone(),
            buyer: self.buyer.clone(),
            price: self.value.clone(),
            declined: true,
            created_at: NaiveDateTime::from_timestamp(self.created_at, 0),
        };

        actions::upsert_bid(&bid, &self.pool).await
    }

    pub async fn upsert_auction(&self) -> Result<()> {
        let auction = NftAuction {
            address: self.address.clone(),
            nft: None,
            price_token: None,
            start_price: None,
            max_bid: None,
            status: None,
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };

        actions::upsert_auction(&auction, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) =
            actions::get_collection_by_auction(&self.address, &self.pool).await
        {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for AuctionComplete {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let seller_token = event
            .tokens
            .iter()
            .find(|t| t.name == "seller")
            .ok_or_else(|| anyhow!("Couldn't find seller token"))?
            .clone();

        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![seller_token, buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionComplete {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            seller: to_address("seller")?,
            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for AuctionComplete {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionComplete
    }
}

impl AuctionComplete {
    pub async fn upsert_auction(&self) -> Result<()> {
        let auction = NftAuction {
            address: self.address.clone(),
            nft: None,
            price_token: None,
            start_price: None,
            max_bid: Some(self.value.clone()),
            status: Some(AuctionStatus::Completed),
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };

        actions::upsert_auction(&auction, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) =
            actions::get_collection_by_auction(&self.address, &self.pool).await
        {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for AuctionCancelled {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(AuctionCancelled {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,
        })
    }
}

impl EventRecord for AuctionCancelled {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionCancelled
    }
}

impl AuctionCancelled {
    pub async fn upsert_auction(&self) -> Result<()> {
        let auction = NftAuction {
            address: self.address.clone(),
            nft: None,
            price_token: None,
            start_price: None,
            max_bid: None,
            status: Some(AuctionStatus::Cancelled),
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };

        actions::upsert_auction(&auction, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) =
            actions::get_collection_by_auction(&self.address, &self.pool).await
        {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for DirectBuyDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let direct_buy_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "directBuyAddress")
            .ok_or_else(|| anyhow!("Couldn't find directBuyAddress token"))?
            .clone();

        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let token_root_token = event
            .tokens
            .iter()
            .find(|t| t.name == "tokenRoot")
            .ok_or_else(|| anyhow!("Couldn't find tokenRoot token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let nonce_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nonce")
            .ok_or_else(|| anyhow!("Couldn't find nonce token"))?
            .clone();

        let amount_token = event
            .tokens
            .iter()
            .find(|t| t.name == "amount")
            .ok_or_else(|| anyhow!("Couldn't find amount token"))?
            .clone();

        let tokens = vec![
            direct_buy_address_token,
            sender_token,
            token_root_token,
            nft_token,
            nonce_token,
            amount_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectBuyDeployed {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            direct_buy_address: to_address("directBuyAddress")?,
            sender: to_address("sender")?,
            token_root: to_address("tokenRoot")?,
            nft: to_address("nft")?,
            nonce: to_bigdecimal("nonce")?,
            amount: to_bigdecimal("amount")?,
        })
    }
}

impl EventRecord for DirectBuyDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyDeployed
    }
}

impl ContractEvent for DirectBuyDeclined {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let token_root_token = event
            .tokens
            .iter()
            .find(|t| t.name == "tokenRoot")
            .ok_or_else(|| anyhow!("Couldn't find tokenRoot token"))?
            .clone();

        let amount_token = event
            .tokens
            .iter()
            .find(|t| t.name == "amount")
            .ok_or_else(|| anyhow!("Couldn't find amount token"))?
            .clone();

        let tokens = vec![sender_token, token_root_token, amount_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectBuyDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            sender: to_address("sender")?,
            token_root: to_address("tokenRoot")?,
            amount: to_bigdecimal("amount")?,
        })
    }
}

impl EventRecord for DirectBuyDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyDeclined
    }
}

impl ContractEvent for DirectBuyOwnershipTransferred {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectBuyOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for DirectBuyOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyOwnershipTransferred
    }
}

impl ContractEvent for DirectSellDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let direct_sell_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "directSellAddress")
            .ok_or_else(|| anyhow!("Couldn't find directSellAddress token"))?
            .clone();

        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let payment_token_token = event
            .tokens
            .iter()
            .find(|t| t.name == "paymentToken")
            .ok_or_else(|| anyhow!("Couldn't find paymentToken token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let nonce_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nonce")
            .ok_or_else(|| anyhow!("Couldn't find nonce token"))?
            .clone();

        let price_token = event
            .tokens
            .iter()
            .find(|t| t.name == "price")
            .ok_or_else(|| anyhow!("Couldn't find price token"))?
            .clone();

        let tokens = vec![
            direct_sell_address_token,
            sender_token,
            payment_token_token,
            nft_token,
            nonce_token,
            price_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectSellDeployed {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            direct_sell_address: to_address("_directSellAddress")?,
            sender: to_address("sender")?,
            payment_token: to_address("paymentToken")?,
            nft: to_address("nft")?,
            nonce: to_bigdecimal("_nonce")?,
            price: to_bigdecimal("price")?,
        })
    }
}

impl EventRecord for DirectSellDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellDeployed
    }
}

impl ContractEvent for DirectSellDeclined {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let _nft_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "_nftAddress")
            .ok_or_else(|| anyhow!("Couldn't find _nftAddress token"))?
            .clone();

        let tokens = vec![sender_token, _nft_address_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectSellDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            sender: to_address("sender")?,
            _nft_address: to_address("_nftAddress")?,
        })
    }
}

impl EventRecord for DirectSellDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellDeclined
    }
}

impl ContractEvent for DirectSellOwnershipTransferred {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectSellOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for DirectSellOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellOwnershipTransferred
    }
}

impl ContractEvent for DirectBuyStateChanged {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let from_token = event
            .tokens
            .iter()
            .find(|t| t.name == "from")
            .ok_or_else(|| anyhow!("Couldn't find from token"))?
            .clone();

        let to_token = event
            .tokens
            .iter()
            .find(|t| t.name == "to")
            .ok_or_else(|| anyhow!("Couldn't find to token"))?
            .clone();

        let value2_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value2")
            .ok_or_else(|| anyhow!("Couldn't find value2 token"))?;
        let mut tokens = match &value2_token.value {
            Tuple(v) => Some(v.clone()),
            _ => None,
        }
        .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

        tokens.extend_from_slice(&[from_token, to_token]);

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
        let to_i16 = get_token_processor(&tokens, token_to_i16);
        let to_i64 = get_token_processor(&tokens, token_to_i64);

        Ok(DirectBuyStateChanged {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            from: to_i16("from")?,
            to: to_i16("to")?,

            factory: to_address("factory")?,
            creator: to_address("creator")?,
            spent_token: to_address("spentToken")?,
            nft: to_address("nft")?,
            _time_tx: to_i64("_timeTx")?,
            _price: to_big_decimal("_price")?,
            spent_wallet: to_address("spentWallet")?,
            status: to_i16("status")?,
            sender: to_address("sender")?,
            start_time_buy: to_i64("startTimeBuy")?,
            duration_time_buy: to_i64("durationTimeBuy")?,
            end_time_buy: to_i64("endTimeBuy")?,
        })
    }
}

impl EventRecord for DirectBuyStateChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyStateChanged
    }
}

impl DirectBuyStateChanged {
    pub async fn upsert_direct_buy(&self) -> Result<()> {
        let direct_buy = NftDirectBuy {
            address: self.address.clone(),
            nft: self.nft.clone(),
            price_token: self.spent_token.clone(),
            price: self._price.clone(),
            state: self.to.into(),
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_direct_buy(&direct_buy, &self.pool).await
    }
}

impl ContractEvent for DirectSellStateChanged {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let from_token = event
            .tokens
            .iter()
            .find(|t| t.name == "from")
            .ok_or_else(|| anyhow!("Couldn't find from token"))?
            .clone();

        let to_token = event
            .tokens
            .iter()
            .find(|t| t.name == "to")
            .ok_or_else(|| anyhow!("Couldn't find to token"))?
            .clone();

        let value2_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value2")
            .ok_or_else(|| anyhow!("Couldn't find value2 token"))?;
        let mut tokens = match &value2_token.value {
            Tuple(v) => Some(v.clone()),
            _ => None,
        }
        .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

        tokens.extend_from_slice(&[from_token, to_token]);

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
        let to_i16 = get_token_processor(&tokens, token_to_i16);
        let to_i64 = get_token_processor(&tokens, token_to_i64);

        Ok(DirectSellStateChanged {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            from: to_i16("from")?,
            to: to_i16("to")?,

            factory: to_address("factory")?,
            creator: to_address("creator")?,
            token: to_address("token")?,
            nft: to_address("nft")?,
            _time_tx: to_i64("_timeTx")?,
            start: to_i64("start")?,
            end: to_i64("end")?,
            _price: to_big_decimal("_price")?,
            wallet: to_address("wallet")?,
            status: to_i16("status")?,
            sender: to_address("sender")?,
        })
    }
}

impl EventRecord for DirectSellStateChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellStateChanged
    }
}

impl DirectSellStateChanged {
    pub async fn upsert_direct_sell(&self) -> Result<()> {
        let direct_sell = NftDirectSell {
            address: self.address.clone(),
            nft: self.nft.clone(),
            price_token: self.token.clone(),
            price: self._price.clone(),
            state: self.to.into(),
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_direct_sell(&direct_sell, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        if let Some(collection) = actions::get_collection_by_nft(&self.nft, &self.pool).await {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;

            actions::upsert_collection(&collection, &self.pool).await
        } else {
            Ok(())
        }
    }
}

impl ContractEvent for NftOwnerChanged {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(NftOwnerChanged {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for NftOwnerChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Nft
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftOwnerChanged
    }
}

impl NftOwnerChanged {
    pub async fn upsert_nft(&self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.address.clone(),
            collection: None,
            owner: Some(self.new_owner.clone()),
            manager: None,
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            burned: false,
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_nft(&nft, &self.pool).await?;
        actions::upsert_nft_meta(&nft_meta, &self.pool).await
    }
}

impl ContractEvent for NftManagerChanged {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldManager")
            .ok_or_else(|| anyhow!("Couldn't find oldManager token"))?
            .clone();

        let new_manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newManager")
            .ok_or_else(|| anyhow!("Couldn't find newManager token"))?
            .clone();

        let tokens = vec![old_manager_token, new_manager_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(NftManagerChanged {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_manager: to_address("oldManager")?,
            new_manager: to_address("newManager")?,
        })
    }
}

impl EventRecord for NftManagerChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Nft
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftManagerChanged
    }
}

impl NftManagerChanged {
    pub async fn upsert_nft(&self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.address.clone(),
            collection: None,
            owner: None,
            manager: Some(self.new_manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            burned: false,
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_nft(&nft, &self.pool).await?;
        actions::upsert_nft_meta(&nft_meta, &self.pool).await
    }
}

impl ContractEvent for CollectionOwnershipTransferred {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(CollectionOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for CollectionOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::CollectionOwnershipTransferred
    }
}

impl CollectionOwnershipTransferred {
    pub async fn upsert_collection(&self) -> Result<()> {
        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        actions::upsert_collection(&collection, &self.pool).await
    }
}

impl ContractEvent for NftCreated {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let id_token = event
            .tokens
            .iter()
            .find(|t| t.name == "id")
            .ok_or_else(|| anyhow!("Couldn't find id token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "owner")
            .ok_or_else(|| anyhow!("Couldn't find owner token"))?
            .clone();

        let manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "manager")
            .ok_or_else(|| anyhow!("Couldn't find manager token"))?
            .clone();

        let creator_token = event
            .tokens
            .iter()
            .find(|t| t.name == "creator")
            .ok_or_else(|| anyhow!("Couldn't find creator token"))?
            .clone();

        let tokens = vec![
            id_token,
            nft_token,
            owner_token,
            manager_token,
            creator_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(NftCreated {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
            creator: to_address("creator")?,
        })
    }
}

impl EventRecord for NftCreated {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftCreated
    }
}

impl NftCreated {
    pub async fn upsert_nft(&self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.nft.0.as_str())?,
            &self.consumer,
        )
        .await;

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.nft.clone(),
            collection: Some(self.address.clone()),
            owner: Some(self.owner.clone()),
            manager: Some(self.manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            burned: false,
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_nft(&nft, &self.pool).await?;
        actions::upsert_nft_meta(&nft_meta, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        actions::upsert_collection(&collection, &self.pool).await
    }
}

impl ContractEvent for NftBurned {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let id_token = event
            .tokens
            .iter()
            .find(|t| t.name == "id")
            .ok_or_else(|| anyhow!("Couldn't find id token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "owner")
            .ok_or_else(|| anyhow!("Couldn't find owner token"))?
            .clone();

        let manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "manager")
            .ok_or_else(|| anyhow!("Couldn't find manager token"))?
            .clone();

        let tokens = vec![id_token, nft_token, owner_token, manager_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(NftBurned {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
        })
    }
}

impl EventRecord for NftBurned {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftBurned
    }
}

impl NftBurned {
    pub async fn upsert_nft(&self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.nft.0.as_str())?,
            &self.consumer,
        )
        .await;

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.nft.clone(),
            collection: Some(self.address.clone()),
            owner: Some(self.owner.clone()),
            manager: Some(self.manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .to_string(),
            burned: true,
            updated: NaiveDateTime::from_timestamp(self.created_at, 0),
            tx_lt: self.created_lt,
        };

        actions::upsert_nft(&nft, &self.pool).await?;
        actions::upsert_nft_meta(&nft_meta, &self.pool).await
    }

    pub async fn upsert_collection(&self) -> Result<()> {
        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        actions::upsert_collection(&collection, &self.pool).await
    }
}

async fn get_collection_data(
    collection: MsgAddressInt,
    consumer: &Arc<TransactionConsumer>,
) -> NftCollection {
    let collection_owner = get_collection_owner(collection.clone(), consumer).await;

    let collection_meta = fetch_metadata(collection.clone(), consumer).await;
    let now = chrono::Utc::now().naive_utc();

    NftCollection {
        address: ("0:".to_owned() + &collection.address().as_hex_string()).into(),
        owner: collection_owner,
        name: collection_meta
            .get("name")
            .cloned()
            .unwrap_or_default()
            .to_string(),
        description: collection_meta
            .get("description")
            .cloned()
            .unwrap_or_default()
            .to_string(),
        created: now,
        updated: now,
    }
}

async fn fetch_metadata(
    address: MsgAddressInt,
    consumer: &Arc<TransactionConsumer>,
) -> serde_json::Value {
    match rpc::retrier::Retrier::new(|| Box::pin(rpc::get_json(address.clone(), consumer.clone())))
        .attempts(3)
        .backoff(10)
        .factor(2)
        .run()
        .await
    {
        Ok(meta) => meta,

        Err(e) => {
            log::error!("Error fetching metadata for {}: {:#?}", address, e);
            serde_json::Value::default()
        }
    }
}

async fn get_collection_owner(
    collection: MsgAddressInt,
    consumer: &Arc<TransactionConsumer>,
) -> storage::types::Address {
    match rpc::retrier::Retrier::new(|| Box::pin(rpc::owner(collection.clone(), consumer.clone())))
        .attempts(3)
        .backoff(10)
        .factor(2)
        .run()
        .await
    {
        Ok(owner) => owner.into(),
        Err(e) => {
            log::error!("Can't get {} collection owner: {:#?}", collection, e);
            String::default().into()
        }
    }
}
