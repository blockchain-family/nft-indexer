use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{
    AuctionStatus, EventCategory, EventRecord, EventType, NftAuction, NftAuctionBid, NftCollection,
    NftPriceHistory, NftPriceSource,
};
use sqlx::PgPool;

use crate::{
    models::events::{
        AuctionActive, AuctionCancelled, AuctionComplete, AuctionCreated, BidDeclined, BidPlaced,
    },
    utils::{u128_to_bigdecimal, EventMessageInfo, KeyInfo},
};

use super::Entity;

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

        let nfts_whitelist: std::collections::HashSet<
            &str,
            std::collections::hash_map::RandomState,
        > = std::collections::HashSet::from_iter(
            [
                "0:69887fccaff1a7ecc337f08f0951050c5f48fe1a0247d2b5f27b50a087ae5200",
                "0:f686736f8bd02f33c7db19fa06cda8aae4a515b1e310ffd59939be25d4f553be",
                "0:e18b796d280e2979c612d63a6b3d6ed414cef2e94c1fdec2693af3eb6a376f74",
                "0:6760bec41ee7795f9b2f18934cae9ed4d3bad8a8124021efeb9641c28abd3d28",
                "0:c49d70bfa5a54021307395fd1d63a0a95b2c7bff0df98849fb26d307c626ed19",
                "0:7ae2b15e16a73bb89666d50ecd0eb8e3caa11fb9ef9a774966815fd3a402f8aa",
            ]
            .into_iter(),
        );

        if let Some(nft) = &event_record.nft {
            if !nfts_whitelist.contains(nft.0.as_str()) {
                return Ok(());
            }
        } else {
            return Ok(());
        }

        let collections_whitelist = vec![
            "0:9eaf3e084cbe25e67cb8730123f65b75429906abc2b01211cccfd3c97047762c",
            "0:e2611558851f4547c6a13b833189136103dcad4350eba36bbb7bf35b6be98ce1",
            "0:48400246d51dd380ad49261f5e6f026347d3a5be5614d82bd655dcb57819e4bf",
        ];

        if let Some(event_collection) = &event_record.collection {
            let mut is_in_whitelist = false;
            for collection in &collections_whitelist {
                if event_collection.0.as_str() == *collection {
                    is_in_whitelist = true;
                    break;
                }
            }
            if !is_in_whitelist {
                // log::debug!(
                //     "Skip nft {:?} for collection {}",
                //     self.nft,
                //     event_collection.0
                // );
                return Ok(());
            }
        } else {
            return Ok(());
        }

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            wallet_for_bids: Some(self.value0.wallet_for_bids.to_string().into()),
            price_token: Some(self.value0._payment_token.to_string().into()),
            start_price: Some(u128_to_bigdecimal(self.value0._price)),
            closing_price_usd: None,
            min_bid: Some(u128_to_bigdecimal(self.value0._price)),
            max_bid: None,
            status: Some(AuctionStatus::Active),
            created_at: NaiveDateTime::from_timestamp_opt(self.value0.start_time as i64, 0),
            finished_at: NaiveDateTime::from_timestamp_opt(self.value0.finish_time as i64, 0),
            tx_lt: event_record.created_lt,
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
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                let now = chrono::Utc::now().naive_utc();

                let collection = NftCollection {
                    address: collection.clone(),
                    owner: "".into(),
                    name: None,
                    description: None,
                    created: now,
                    updated: now,
                    logo: None,
                    wallpaper: None,
                };

                indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None)
                    .await?;
            }
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
            wallet_for_bids: None,
            price_token: None,
            start_price: None,
            closing_price_usd: None,
            min_bid,
            max_bid: Some(u128_to_bigdecimal(self.value)),
            status: None,
            created_at: None,
            finished_at: None,
            tx_lt: event_record.created_lt,
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
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                let now = chrono::Utc::now().naive_utc();

                let collection = NftCollection {
                    address: collection.clone(),
                    owner: "".into(),
                    name: None,
                    description: None,
                    created: now,
                    updated: now,
                    logo: None,
                    wallpaper: None,
                };

                indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None)
                    .await?;
            }
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
            next_bid_value: None,
            declined: true,
            created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            tx_lt: event_record.created_lt,
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
        // HACK: turn off the usd price request
        let closing_price_usd = None;
        // if price_token.is_some() && false {
        //     let usd_price = rpc::token_to_usd(&price_token.as_ref().unwrap().0)
        //         .await
        //         .unwrap_or_default();
        //     Some(usd_price * u128_to_bigdecimal(self.value))
        // } else {
        //     None
        // };

        let auction = NftAuction {
            address: event_record.address.clone(),
            nft: event_record.nft.clone(),
            wallet_for_bids: None,
            price_token,
            start_price: None,
            closing_price_usd,
            min_bid: None,
            max_bid: Some(u128_to_bigdecimal(self.value)),
            status: Some(AuctionStatus::Completed),
            created_at: None,
            finished_at: None,
            tx_lt: event_record.created_lt,
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                let now = chrono::Utc::now().naive_utc();

                let collection = NftCollection {
                    address: collection.clone(),
                    owner: "".into(),
                    name: None,
                    description: None,
                    created: now,
                    updated: now,
                    logo: None,
                    wallpaper: None,
                };

                indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None)
                    .await?;
            }
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
            wallet_for_bids: None,
            price_token: None,
            start_price: None,
            closing_price_usd: None,
            min_bid: None,
            max_bid: None,
            status: Some(AuctionStatus::Cancelled),
            created_at: None,
            finished_at: None,
            tx_lt: event_record.created_lt,
        };

        indexer_repo::actions::upsert_auction(&auction, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                let now = chrono::Utc::now().naive_utc();

                let collection = NftCollection {
                    address: collection.clone(),
                    owner: "".into(),
                    name: None,
                    description: None,
                    created: now,
                    updated: now,
                    logo: None,
                    wallpaper: None,
                };

                indexer_repo::actions::upsert_collection(&collection, &mut pg_pool_tx, None)
                    .await?;
            }
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
