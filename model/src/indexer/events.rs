use crate::indexer::{record_build_utils::*, traits::ContractEvent};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use futures::Future;
use nekoton_abi::transaction_parser::ExtractedOwned;
use serde::Serialize;
use sqlx::{
    postgres::{PgDatabaseError, PgSeverity},
    types::BigDecimal,
    PgPool,
};
use std::{str::FromStr, sync::Arc};
use storage::{actions, traits::EventRecord, types::*};
use ton_abi::TokenValue::Tuple;
use ton_block::MsgAddressInt;
use traits_derive::EventRecord;
use transaction_consumer::TransactionConsumer;

/// AuctionRootTip3 events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    //pub offer: Address,
    pub collection: Address,
    pub nft_owner: Address,
    pub nft: Address,
    pub offer: Address,
    pub price: BigDecimal,
    pub auction_duration: i64,
    pub deploy_nonce: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub nft_owner: Address,
    pub nft: Address,
}

#[derive(Clone, Serialize, EventRecord)]
pub struct AuctionRootOwnershipTransferred {
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// AuctionTip3 events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub _payment_token: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
    pub status: i16,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub _payment_token: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
    pub status: i16,
}

#[derive(Clone, Serialize, EventRecord)]
pub struct AuctionBidPlaced {
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub buyer: Address,
    pub value: BigDecimal,
    pub next_bid_value: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
pub struct AuctionBidDeclined {
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub seller: Address,
    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,
}

/// FactoryDirectBuy events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub direct_buy: Address,
    pub sender: Address,
    pub token: Address,
    pub nft: Address,
    pub nonce: BigDecimal,
    pub amount: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub sender: Address,
    pub token: Address,
    pub amount: BigDecimal,
    pub nft: Address,
}

#[derive(Clone, Serialize, EventRecord)]
pub struct FactoryDirectBuyOwnershipTransferred {
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// FactoryDirectSell events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub direct_sell: Address,
    pub sender: Address,
    pub payment_token: Address,
    pub nft: Address,
    pub nonce: BigDecimal,
    pub price: BigDecimal,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub sender: Address,
    pub nft: Address,
}

#[derive(Clone, Serialize, EventRecord)]
pub struct FactoryDirectSellOwnershipTransferred {
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// DirectBuy events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

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
    pub start_time_buy: i64,
    pub duration_time_buy: i64,
    pub end_time_buy: i64,
}

/// DirectSell events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

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
}

// Nft events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_manager: Address,
    pub new_manager: Address,
}

// Collection events

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
    pub creator: Address,
}

#[derive(Clone, Serialize, EventRecord)]
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

    #[serde(skip_serializing)]
    pub event_nft: Option<Address>,
    #[serde(skip_serializing)]
    pub event_collection: Option<Address>,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
}

async fn await_handling_error<F, T>(f: F, trace_id: &str)
where
    F: Future<Output = Result<T, sqlx::Error>> + Send,
{
    if let Err(e) = f.await {
        if let Some(e) = e.as_database_error() {
            if let Some(e) = e.try_downcast_ref::<PgDatabaseError>() {
                if e.severity() == PgSeverity::Fatal {
                    // better stop indexer
                    std::process::exit(1);
                }
            }
        }

        log::error!("[{}] Error: {:#?}", trace_id, e);
    }
}

#[async_trait]
impl ContractEvent for AuctionDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        // let offer_token = event
        //     .tokens
        //     .iter()
        //     .find(|t| t.name == "offer")
        //     .ok_or_else(|| anyhow!("Couldn't find offer token"))?
        //     .clone();

        let offer_info = event
            .tokens
            .iter()
            .find(|t| t.name == "offerInfo")
            .ok_or_else(|| anyhow!("Couldn't find offerInfo token"))?;
        let tokens = match &offer_info.value {
            Tuple(v) => Some(v.clone()),
            _ => None,
        }
        .ok_or_else(|| anyhow!("offerInfo token value is not tuple"))?;

        // tokens.push(offer_token);

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_i64 = get_token_processor(&tokens, token_to_i64);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionDeployed {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: Some(to_address("collection")?),
            event_nft: Some(to_address("nft")?),

            // offer: to_address("offer")?,
            collection: to_address("collection")?,
            nft_owner: to_address("nftOwner")?,
            nft: to_address("nft")?,
            offer: to_address("offer")?,
            price: to_big_decimal("price")?,
            auction_duration: to_i64("auctionDuration")?,
            deploy_nonce: to_big_decimal("deployNonce")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionDeployed").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let tokens = vec![nft_owner_token, nft_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(AuctionDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: Some(to_address("nft")?),

            nft_owner: to_address("nftOwner")?,
            nft: to_address("nft")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionDeclined").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for AuctionRootOwnershipTransferred {
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

        Ok(AuctionRootOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: None,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving AuctionRootOwnershipTransferred",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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
        .ok_or_else(|| anyhow!("value0 token value is not tuple"))?;

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_i16 = get_token_processor(tokens, token_to_i16);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionCreated {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: Some(to_address("auctionSubject")?),

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            _payment_token: to_address("_paymentToken")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
            status: to_i16("status")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.event_collection =
            actions::get_collection_by_nft(&self.auction_subject, &mut tx).await;
        await_handling_error(
            actions::update_nft_by_auction(
                "nft_events",
                &self.address,
                &self.auction_subject,
                &mut tx,
            ),
            "Updating nft of auctions",
        )
        .await;

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionCreated").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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
        .ok_or_else(|| anyhow!("value0 token value is not tuple"))?;

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_i16 = get_token_processor(tokens, token_to_i16);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionActive {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: Some(to_address("auctionSubject")?),

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            _payment_token: to_address("_paymentToken")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
            status: to_i16("status")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let auction = NftAuction {
            address: self.address.clone(),
            nft: self.event_nft.clone(),
            wallet_for_bids: Some(self.wallet_for_bids.clone()),
            price_token: Some(self._payment_token.clone()),
            start_price: Some(self._price.clone()),
            closing_price_usd: None,
            min_bid: Some(self._price.clone()),
            max_bid: None,
            status: Some(AuctionStatus::Active),
            created_at: NaiveDateTime::from_timestamp_opt(self.start_time, 0),
            finished_at: NaiveDateTime::from_timestamp_opt(self.finish_time, 0),
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_auction(&auction, &mut tx),
            "Inserting Auction",
        )
        .await;

        await_handling_error(
            actions::update_nft_by_auction(
                "nft_events",
                &self.address,
                &self.auction_subject,
                &mut tx,
            ),
            "Updating nft of auctions",
        )
        .await;

        self.event_collection =
            actions::get_collection_by_nft(&self.auction_subject, &mut tx).await;
        if let Some(collection) = self.event_collection.as_ref() {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;
            await_handling_error(
                actions::upsert_collection(&collection, &mut tx),
                "Inserting collection",
            )
            .await;
        }

        let price_history = NftPriceHistory {
            source: self.address.clone(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            price: self._price.clone(),
            price_token: Some(self._payment_token.clone()),
            nft: self.event_nft.clone(),
            collection: self.event_collection.clone(),
        };
        await_handling_error(
            actions::upsert_nft_price_history(&price_history, &mut tx),
            "Updating NftPriceHistory",
        )
        .await;

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionActive").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for AuctionBidPlaced {
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

        let next_bid_value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nextBidValue")
            .ok_or_else(|| anyhow!("Couldn't find nextBidValue token"))?
            .clone();

        let tokens = vec![buyer_token, value_token, next_bid_value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionBidPlaced {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: None,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
            next_bid_value: to_bigdecimal("nextBidValue")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        (self.event_nft, self.event_collection) =
            actions::get_nft_and_collection_by_auction(&self.address, &mut tx).await;

        let bid = NftAuctionBid {
            auction: self.address.clone(),
            buyer: self.buyer.clone(),
            price: self.value.clone(),
            next_bid_value: Some(self.next_bid_value.clone()),
            declined: false,
            created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::insert_auction_bid(&bid, &mut tx),
            "Updating AuctionBid",
        )
        .await;

        let min_bid = Some(self.next_bid_value.clone());

        let auction = NftAuction {
            address: self.address.clone(),
            nft: self.event_nft.clone(),
            wallet_for_bids: None,
            price_token: None,
            start_price: None,
            closing_price_usd: None,
            min_bid,
            max_bid: Some(self.value.clone()),
            status: None,
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_auction(&auction, &mut tx),
            "Updating Auction",
        )
        .await;

        let price_history = NftPriceHistory {
            source: self.address.clone(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            price: self.value.clone(),
            price_token: None,
            nft: self.event_nft.clone(),
            collection: self.event_collection.clone(),
        };
        await_handling_error(
            actions::upsert_nft_price_history(&price_history, &mut tx),
            "Updating NftPriceHistory",
        )
        .await;

        if let Some(collection) = self.event_collection.as_ref() {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;
            await_handling_error(
                actions::upsert_collection(&collection, &mut tx),
                "Inserting collection",
            )
            .await;
        }

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionBid").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for AuctionBidDeclined {
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

        Ok(AuctionBidDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: None,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let bid = NftAuctionBid {
            auction: self.address.clone(),
            buyer: self.buyer.clone(),
            price: self.value.clone(),
            next_bid_value: None,
            declined: true,
            created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            tx_lt: self.created_lt,
        };

        await_handling_error(
            actions::insert_auction_bid(&bid, &mut tx),
            "Updating AuctionBid",
        )
        .await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving AuctionBidDeclined",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: None,

            seller: to_address("seller")?,
            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        (self.event_nft, self.event_collection) =
            actions::get_nft_and_collection_by_auction(&self.address, &mut tx).await;

        let price_token = actions::get_auction_price_token(&self.address, &mut tx).await;
        let closing_price_usd = if price_token.is_some() {
            let usd_price = rpc::token_to_usd(&price_token.as_ref().unwrap().0)
                .await
                .unwrap_or_default();
            Some(usd_price * self.value.clone())
        } else {
            None
        };

        let auction = NftAuction {
            address: self.address.clone(),
            nft: self.event_nft.clone(),
            wallet_for_bids: None,
            price_token,
            start_price: None,
            closing_price_usd,
            min_bid: None,
            max_bid: Some(self.value.clone()),
            status: Some(AuctionStatus::Completed),
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_auction(&auction, &mut tx),
            "Updating Auction",
        )
        .await;

        if let Some(collection) = self.event_collection.as_ref() {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;
            await_handling_error(
                actions::upsert_collection(&collection, &mut tx),
                "Inserting collection",
            )
            .await;
        }

        await_handling_error(actions::save_event(self, &mut tx), "Saving AuctionComplete").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: None,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        (self.event_nft, self.event_collection) =
            actions::get_nft_and_collection_by_auction(&self.address, &mut tx).await;

        let auction = NftAuction {
            address: self.address.clone(),
            nft: self.event_nft.clone(),
            wallet_for_bids: None,
            price_token: None,
            start_price: None,
            closing_price_usd: None,
            min_bid: None,
            max_bid: None,
            status: Some(AuctionStatus::Cancelled),
            created_at: None,
            finished_at: None,
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_auction(&auction, &mut tx),
            "Updating Auction",
        )
        .await;

        if let Some(collection) = self.event_collection.as_ref() {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;
            await_handling_error(
                actions::upsert_collection(&collection, &mut tx),
                "Inserting collection",
            )
            .await;
        }

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving AuctionCancelled",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for DirectBuyDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let direct_buy_token = event
            .tokens
            .iter()
            .find(|t| t.name == "directBuy")
            .ok_or_else(|| anyhow!("Couldn't find directBuy token"))?
            .clone();

        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let token_token = event
            .tokens
            .iter()
            .find(|t| t.name == "token")
            .ok_or_else(|| anyhow!("Couldn't find token token"))?
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
            direct_buy_token,
            sender_token,
            token_token,
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

            event_collection: None,
            event_nft: Some(to_address("nft")?),

            direct_buy: to_address("directBuy")?,
            sender: to_address("sender")?,
            token: to_address("token")?,
            nft: to_address("nft")?,
            nonce: to_bigdecimal("nonce")?,
            amount: to_bigdecimal("amount")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.nft, &mut tx).await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectBuyDeployed",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

        let token_token = event
            .tokens
            .iter()
            .find(|t| t.name == "token")
            .ok_or_else(|| anyhow!("Couldn't find token token"))?
            .clone();

        let amount_token = event
            .tokens
            .iter()
            .find(|t| t.name == "amount")
            .ok_or_else(|| anyhow!("Couldn't find amount token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let tokens = vec![sender_token, token_token, amount_token, nft_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectBuyDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: Some(to_address("nft")?),

            sender: to_address("sender")?,
            token: to_address("token")?,
            amount: to_bigdecimal("amount")?,
            nft: to_address("nft")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectBuyDeclined",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for FactoryDirectBuyOwnershipTransferred {
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

        Ok(FactoryDirectBuyOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: None,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving FactoryDirectBuyOwnershipTransferred",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for DirectSellDeployed {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let direct_sell_token = event
            .tokens
            .iter()
            .find(|t| t.name == "directSell")
            .ok_or_else(|| anyhow!("Couldn't find directSell token"))?
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
            direct_sell_token,
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

            event_collection: None,
            event_nft: Some(to_address("nft")?),

            direct_sell: to_address("directSell")?,
            sender: to_address("sender")?,
            payment_token: to_address("paymentToken")?,
            nft: to_address("nft")?,
            nonce: to_bigdecimal("nonce")?,
            price: to_bigdecimal("price")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.nft, &mut tx).await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectSellDeployed",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let tokens = vec![sender_token, nft_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectSellDeclined {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: Some(to_address("nft")?),

            sender: to_address("sender")?,
            nft: to_address("nft")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectSellDeclined",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
impl ContractEvent for FactoryDirectSellOwnershipTransferred {
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

        Ok(FactoryDirectSellOwnershipTransferred {
            pool: pool.clone(),
            consumer: consumer.clone(),

            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            event_collection: None,
            event_nft: None,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving FactoryDirectSellOwnershipTransferred",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: Some(to_address("nft")?),

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
            start_time_buy: to_i64("startTimeBuy")?,
            duration_time_buy: to_i64("durationTimeBuy")?,
            end_time_buy: to_i64("endTimeBuy")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.nft, &mut tx).await;

        let state = self.to.into();
        let created_ts = NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default();

        if state != DirectBuyState::Create {
            let price_history = NftPriceHistory {
                source: self.address.clone(),
                source_type: NftPriceSource::DirectBuy,
                created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0)
                    .unwrap_or_default(),
                price: self._price.clone(),
                price_token: Some(self.spent_token.clone()),
                nft: self.event_nft.clone(),
                collection: self.event_collection.clone(),
            };
            await_handling_error(
                actions::upsert_nft_price_history(&price_history, &mut tx),
                "Updating NftPriceHistory",
            )
            .await;
        }

        let (buy_price_usd, finished_at) = if state == DirectBuyState::Filled {
            let usd_price = rpc::token_to_usd(&self.spent_token.0)
                .await
                .unwrap_or_default();
            (Some(usd_price * self._price.clone()), Some(created_ts))
        } else {
            (None, None)
        };

        let direct_buy = NftDirectBuy {
            address: self.address.clone(),
            nft: self.nft.clone(),
            collection: self.event_collection.clone(),
            price_token: self.spent_token.clone(),
            price: self._price.clone(),
            buy_price_usd,
            buyer: self.creator.clone(),
            finished_at,
            expired_at: NaiveDateTime::from_timestamp_opt(self.end_time_buy, 0).unwrap_or_default(),
            state,
            created: NaiveDateTime::from_timestamp_opt(self.start_time_buy, 0).unwrap_or_default(),
            updated: created_ts,
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_direct_buy(&direct_buy, &mut tx),
            "Updating DirectBuy",
        )
        .await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectBuyStateChanged",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: Some(to_address("nft")?),

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
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.nft, &mut tx).await;

        let state = self.to.into();
        let created_ts = NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default();

        if state != DirectSellState::Create {
            let price_history = NftPriceHistory {
                source: self.address.clone(),
                source_type: NftPriceSource::DirectSell,
                created_at: NaiveDateTime::from_timestamp_opt(self.created_at, 0)
                    .unwrap_or_default(),
                price: self._price.clone(),
                price_token: Some(self.token.clone()),
                nft: self.event_nft.clone(),
                collection: self.event_collection.clone(),
            };
            await_handling_error(
                actions::upsert_nft_price_history(&price_history, &mut tx),
                "Updating NftPriceHistory",
            )
            .await;
        }

        let (sell_price_usd, finished_at) = if state == DirectSellState::Filled {
            let usd_price = rpc::token_to_usd(&self.token.0).await.unwrap_or_default();
            (Some(usd_price * self._price.clone()), Some(created_ts))
        } else {
            (None, None)
        };

        let direct_sell = NftDirectSell {
            address: self.address.clone(),
            nft: self.nft.clone(),
            collection: self.event_collection.clone(),
            price_token: self.token.clone(),
            price: self._price.clone(),
            sell_price_usd,
            seller: self.creator.clone(),
            finished_at,
            expired_at: NaiveDateTime::from_timestamp_opt(self.end, 0).unwrap_or_default(),
            state,
            created: NaiveDateTime::from_timestamp_opt(self.start, 0).unwrap_or_default(),
            updated: created_ts,
            tx_lt: self.created_lt,
        };
        await_handling_error(
            actions::upsert_direct_sell(&direct_sell, &mut tx),
            "Updating DirectSell",
        )
        .await;

        if let Some(collection) = self.event_collection.as_ref() {
            let collection = get_collection_data(
                MsgAddressInt::from_str(collection.0.as_str())?,
                &self.consumer,
            )
            .await;
            await_handling_error(
                actions::upsert_collection(&collection, &mut tx),
                "Inserting collection",
            )
            .await;
        }

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving DirectSellStateChanged",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: Some(get_address(event)),

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.address, &mut tx).await;

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| {
                    NftAttribute::new(
                        self.address.clone(),
                        self.event_collection.clone(),
                        item.clone(),
                    )
                })
                .collect();

            await_handling_error(
                actions::upsert_nft_attributes(&nft_attributes, &mut tx),
                "Updating nft attributes",
            )
            .await;
        }

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.address.clone(),
            collection: self.event_collection.clone(),
            owner: Some(self.new_owner.clone()),
            manager: None,
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            owner_update_lt: self.created_lt,
            manager_update_lt: 0,
        };

        await_handling_error(actions::upsert_nft(&nft, &mut tx), "Updating nft").await;
        await_handling_error(
            actions::upsert_nft_meta(&nft_meta, &mut tx),
            "Updating nft meta",
        )
        .await;

        await_handling_error(actions::save_event(self, &mut tx), "Saving NftOwnerChanged").await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: None,
            event_nft: Some(get_address(event)),

            old_manager: to_address("oldManager")?,
            new_manager: to_address("newManager")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        let mut tx = self.pool.begin().await?;

        self.event_collection = actions::get_collection_by_nft(&self.address, &mut tx).await;

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| {
                    NftAttribute::new(
                        self.address.clone(),
                        self.event_collection.clone(),
                        item.clone(),
                    )
                })
                .collect();

            await_handling_error(
                actions::upsert_nft_attributes(&nft_attributes, &mut tx),
                "Updating nft attributes",
            )
            .await;
        }

        let nft_meta = NftMeta {
            nft: self.address.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.address.clone(),
            collection: self.event_collection.clone(),
            owner: None,
            manager: Some(self.new_manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            owner_update_lt: 0,
            manager_update_lt: self.created_lt,
        };

        await_handling_error(actions::upsert_nft(&nft, &mut tx), "Updating nft").await;
        await_handling_error(
            actions::upsert_nft_meta(&nft_meta, &mut tx),
            "Updating nft meta",
        )
        .await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving NftManagerChanged",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: Some(get_address(event)),
            event_nft: None,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        let mut tx = self.pool.begin().await?;

        await_handling_error(
            actions::upsert_collection(&collection, &mut tx),
            "Updating collection",
        )
        .await;

        await_handling_error(
            actions::save_event(self, &mut tx),
            "Saving CollectionOwnershipTransferred",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: Some(get_address(event)),
            event_nft: Some(to_address("nft")?),

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
            creator: to_address("creator")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.nft.0.as_str())?,
            &self.consumer,
        )
        .await;

        let mut tx = self.pool.begin().await?;

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| {
                    NftAttribute::new(
                        self.nft.clone(),
                        self.event_collection.clone(),
                        item.clone(),
                    )
                })
                .collect();

            await_handling_error(
                actions::upsert_nft_attributes(&nft_attributes, &mut tx),
                "Updating nft attributes",
            )
            .await;
        }

        let nft_meta = NftMeta {
            nft: self.nft.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.nft.clone(),
            collection: self.event_collection.clone(),
            owner: Some(self.owner.clone()),
            manager: Some(self.manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            owner_update_lt: self.created_lt,
            manager_update_lt: self.created_lt,
        };

        await_handling_error(actions::upsert_nft(&nft, &mut tx), "Updating nft").await;
        await_handling_error(
            actions::upsert_nft_meta(&nft_meta, &mut tx),
            "Updating nft meta",
        )
        .await;

        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        await_handling_error(
            actions::upsert_collection(&collection, &mut tx),
            "Updating collection",
        )
        .await;

        await_handling_error(actions::save_event(self, &mut tx), "Saving NftCreated").await;

        await_handling_error(
            actions::update_collection_by_nft("nft_events", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft("nft_direct_sell", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft("nft_direct_buy", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft(
                "nft_price_history",
                &self.nft,
                &self.address,
                &mut tx,
            ),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft("nft_attributes", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
    }
}

#[async_trait]
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

            event_collection: Some(get_address(event)),
            event_nft: Some(to_address("nft")?),

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
        })
    }

    async fn update_dependent_tables(&mut self) -> Result<()> {
        let meta = fetch_metadata(
            MsgAddressInt::from_str(self.nft.0.as_str())?,
            &self.consumer,
        )
        .await;

        let mut tx = self.pool.begin().await?;

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| {
                    NftAttribute::new(
                        self.nft.clone(),
                        self.event_collection.clone(),
                        item.clone(),
                    )
                })
                .collect();

            await_handling_error(
                actions::upsert_nft_attributes(&nft_attributes, &mut tx),
                "Updating nft attributes",
            )
            .await;
        }

        let nft_meta = NftMeta {
            nft: self.nft.clone(),
            meta,
            updated: chrono::Utc::now().naive_utc(),
        };

        let nft = Nft {
            address: self.nft.clone(),
            collection: self.event_collection.clone(),
            owner: Some(self.owner.clone()),
            manager: Some(self.manager.clone()),
            name: nft_meta
                .meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            description: nft_meta
                .meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            burned: true,
            updated: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default(),
            owner_update_lt: self.created_lt,
            manager_update_lt: self.created_lt,
        };

        await_handling_error(actions::upsert_nft(&nft, &mut tx), "Updating nft").await;
        await_handling_error(
            actions::upsert_nft_meta(&nft_meta, &mut tx),
            "Updating nft meta",
        )
        .await;

        let collection = get_collection_data(
            MsgAddressInt::from_str(self.address.0.as_str())?,
            &self.consumer,
        )
        .await;

        await_handling_error(
            actions::upsert_collection(&collection, &mut tx),
            "Updating collection",
        )
        .await;

        await_handling_error(actions::save_event(self, &mut tx), "Saving NftBurned").await;

        await_handling_error(
            actions::update_collection_by_nft("nft_events", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft("nft_direct_sell", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft("nft_direct_buy", &self.nft, &self.address, &mut tx),
            "Updating collection by nft",
        )
        .await;
        await_handling_error(
            actions::update_collection_by_nft(
                "nft_price_history",
                &self.nft,
                &self.address,
                &mut tx,
            ),
            "Updating collection by nft",
        )
        .await;

        tx.commit().await.map_err(|e| anyhow!(e))
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
            .as_str()
            .map(str::to_string),
        description: collection_meta
            .get("description")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(str::to_string),
        created: now,
        updated: now,
        logo: collection_meta
            .get("preview")
            .cloned()
            .unwrap_or_default()
            .get("source")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(|s| s.into()),
        wallpaper: collection_meta
            .get("files")
            .cloned()
            .unwrap_or_default()
            .as_array()
            .cloned()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default()
            .get("source")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(|s| s.into()),
    }
}

async fn fetch_metadata(
    address: MsgAddressInt,
    consumer: &Arc<TransactionConsumer>,
) -> serde_json::Value {
    match rpc::retrier::Retrier::new(|| Box::pin(rpc::get_json(address.clone(), consumer.clone())))
        .attempts(1)
        .trace_id(format!(
            "fetch metadata {}",
            address.address().as_hex_string()
        ))
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
        .attempts(1)
        .trace_id(format!(
            "collection owner {}",
            collection.address().as_hex_string()
        ))
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
