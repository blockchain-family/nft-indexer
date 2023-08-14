use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{
    AddressChangedDecoded, AuctionActiveDecoded, AuctionBidDecoded, AuctionCancelledDecoded,
    AuctionCompleteDecoded, AuctionStatus, EventCategory, EventRecord, EventType, NftAuction,
    NftAuctionBid, NftCollection, NftPriceHistory, NftPriceSource,
};
use sqlx::PgPool;

use crate::utils::u128_to_bigdecimal;
use crate::{
    models::events::{
        AuctionActive, AuctionCancelled, AuctionComplete, AuctionCreated, BidDeclined, BidPlaced,
    },
    utils::{EventMessageInfo, KeyInfo},
};

use super::{Decode, Decoded, Entity};

impl Decode for AuctionCreated {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::AuctionCreated(AddressChangedDecoded {
            id_address: msg_info.tx_data.get_account().into(),
            new_address: self.value0.auction_subject.to_string().into(),
            timestamp: 0,
        }))
    }
}

impl Decode for AuctionActive {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let auction = AuctionActiveDecoded {
            address: msg_info.tx_data.get_account(),
            nft: self.value0.auction_subject.to_string(),
            wallet_for_bids: self.value0.wallet_for_bids.to_string(),
            price_token: self.value0._payment_token.to_string(),
            start_price: u128_to_bigdecimal(self.value0._price),
            min_bid: u128_to_bigdecimal(self.value0._price),
            created_at: self.value0.start_time,
            finished_at: self.value0.finish_time,
            tx_lt: msg_info.tx_data.logical_time().try_into()?,
        };

        let price_hist = NftPriceHistory {
            source: msg_info.tx_data.get_account().into(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(msg_info.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value0._price),
            price_token: Some(self.value0._payment_token.to_string().into()),
            nft: Some(self.value0.auction_subject.to_string().into()),
            collection: None,
        };

        Ok(Decoded::AuctionActive((auction, price_hist)))
    }
}

impl Decode for BidPlaced {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let bid = AuctionBidDecoded {
            address: msg_info.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: u128_to_bigdecimal(self.next_bid_value),
            buyer: self.buyer.to_string(),
            created_at: msg_info.tx_data.get_timestamp().try_into()?,
            tx_lt: msg_info.tx_data.logical_time(),
        };

        let price_hist = NftPriceHistory {
            source: msg_info.tx_data.get_account().into(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(msg_info.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value),
            price_token: None,
            nft: None,
            collection: None,
        };

        Ok(Decoded::AuctionBidPlaced((bid, price_hist)))
    }
}

impl Decode for BidDeclined {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let bid = AuctionBidDecoded {
            address: msg_info.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: Default::default(),
            buyer: self.buyer.to_string(),
            created_at: msg_info.tx_data.get_timestamp().try_into()?,
            tx_lt: msg_info.tx_data.logical_time(),
        };

        Ok(Decoded::AuctionBidDeclined(bid))
    }
}

impl Decode for AuctionComplete {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let auc = AuctionCompleteDecoded {
            address: msg_info.tx_data.get_account(),
            max_bid: u128_to_bigdecimal(self.value),
        };

        Ok(Decoded::AuctionComplete(auc))
    }
}

impl Decode for AuctionCancelled {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let auc = AuctionCancelledDecoded {
            address: msg_info.tx_data.get_account(),
        };

        Ok(Decoded::AuctionCancelled(auc))
    }
}

#[async_trait]
impl Entity for AuctionCreated {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCreated,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.value0.auction_subject.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.value0.auction_subject.to_string().into(),
                &mut pg_pool_tx,
            )
            .await,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        indexer_repo::actions::update_nft_by_auction(
            "nft_events",
            &event_record.address,
            event_record.nft.as_ref().unwrap(),
            &mut pg_pool_tx,
        )
        .await?;

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionCreated event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for AuctionActive {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionActive,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.value0.auction_subject.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.value0.auction_subject.to_string().into(),
                &mut pg_pool_tx,
            )
            .await,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            wallet_for_bids: Some(self.value0.wallet_for_bids.to_string().into()),
            price_token: Some(self.value0._payment_token.to_string().into()),
            start_price: Some(u128_to_bigdecimal(self.value0._price)),
            min_bid: Some(u128_to_bigdecimal(self.value0._price)),
            status: Some(AuctionStatus::Active),
            created_at: NaiveDateTime::from_timestamp_opt(self.value0.start_time as i64, 0),
            finished_at: NaiveDateTime::from_timestamp_opt(self.value0.finish_time as i64, 0),
            tx_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        indexer_repo::actions::update_nft_by_auction(
            "nft_events",
            &event_record.address,
            event_record.nft.as_ref().unwrap(),
            &mut pg_pool_tx,
        )
        .await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let now = chrono::Utc::now().naive_utc();

            let collection = NftCollection {
                address: collection.clone(),
                created: now,
                updated: now,
                ..Default::default()
            };

            indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None).await?;
        }

        let price_history = NftPriceHistory {
            source: event_record.address.clone(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value0._price),
            price_token: Some(self.value0._payment_token.to_string().into()),
            nft: event_record.nft.clone(),
            collection: event_record.collection.clone(),
        };
        indexer_repo::actions::upsert_nft_price_history(&price_history, &mut pg_pool_tx).await?;

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionActive event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for BidPlaced {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let mut event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidPlaced,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let (event_nft, event_collection) =
            indexer_repo::actions::get_nft_and_collection_by_auction(
                &event_record.address,
                &mut pg_pool_tx,
            )
            .await;

        event_record.nft = event_nft;
        event_record.collection = event_collection;

        let bid = NftAuctionBid {
            auction: event_record.address.clone(),
            buyer: self.buyer.to_string().into(),
            price: u128_to_bigdecimal(self.value),
            next_bid_value: Some(u128_to_bigdecimal(self.next_bid_value)),
            declined: false,
            created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            tx_lt: event_record.created_lt,
        };

        indexer_repo::actions::insert_auction_bid(&bid, &mut pg_pool_tx).await?;

        let min_bid = Some(u128_to_bigdecimal(self.next_bid_value));

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            min_bid,
            max_bid: Some(u128_to_bigdecimal(self.value)),
            tx_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        let price_history = NftPriceHistory {
            source: event_record.address.clone(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value),
            price_token: None,
            nft: event_record.nft.clone(),
            collection: event_record.collection.clone(),
        };

        indexer_repo::actions::upsert_nft_price_history(&price_history, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let now = chrono::Utc::now().naive_utc();

            let collection = NftCollection {
                address: collection.clone(),
                created: now,
                updated: now,
                ..Default::default()
            };

            indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None).await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionBid event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for BidDeclined {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let bid = NftAuctionBid {
            auction: event_record.address.clone(),
            buyer: self.buyer.to_string().into(),
            price: u128_to_bigdecimal(self.value),
            declined: true,
            created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            tx_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::insert_auction_bid(&bid, &mut pg_pool_tx).await?;

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionBidDeclined event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for AuctionComplete {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let mut event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionComplete,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let (event_nft, event_collection) =
            indexer_repo::actions::get_nft_and_collection_by_auction(
                &event_record.address,
                &mut pg_pool_tx,
            )
            .await;

        event_record.nft = event_nft;
        event_record.collection = event_collection;

        let price_token =
            indexer_repo::actions::get_auction_price_token(&event_record.address, &mut pg_pool_tx)
                .await;

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            price_token,
            max_bid: Some(u128_to_bigdecimal(self.value)),
            status: Some(AuctionStatus::Completed),
            tx_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let now = chrono::Utc::now().naive_utc();

            let collection = NftCollection {
                address: collection.clone(),
                created: now,
                updated: now,
                ..Default::default()
            };

            indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None).await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionComplete event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for AuctionCancelled {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let mut event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCancelled,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let (event_nft, event_collection) =
            indexer_repo::actions::get_nft_and_collection_by_auction(
                &event_record.address,
                &mut pg_pool_tx,
            )
            .await;

        event_record.nft = event_nft;
        event_record.collection = event_collection;

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            status: Some(AuctionStatus::Cancelled),
            tx_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let now = chrono::Utc::now().naive_utc();

            let collection = NftCollection {
                address: collection.clone(),
                created: now,
                updated: now,
                ..Default::default()
            };

            indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None).await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionCancelled event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
