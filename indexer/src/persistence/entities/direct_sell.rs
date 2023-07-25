use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{
    DirectSellState, EventCategory, EventRecord, EventType, NftCollection, NftDirectSell,
    NftPriceHistory, NftPriceSource,
};
use sqlx::PgPool;

use crate::{
    models::events::DirectSellStateChanged,
    utils::{u128_to_bigdecimal, EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for DirectSellStateChanged {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellStateChanged,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.value2.nft.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.value2.nft.to_string().into(),
                &mut pg_pool_tx,
            )
            .await,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let collections_whitelist = vec![
            "0:9eaf3e084cbe25e67cb8730123f65b75429906abc2b01211cccfd3c97047762c",
            "0:e2611558851f4547c6a13b833189136103dcad4350eba36bbb7bf35b6be98ce1",
            "0:e18b796d280e2979c612d63a6b3d6ed414cef2e94c1fdec2693af3eb6a376f74",
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

        let state = self.to.into();
        let created_ts =
            NaiveDateTime::from_timestamp_opt(event_record.created_at, 0).unwrap_or_default();

        if state != DirectSellState::Create {
            let price_history = NftPriceHistory {
                source: event_record.address.clone(),
                source_type: NftPriceSource::DirectSell,
                created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                    .unwrap_or_default(),
                price: u128_to_bigdecimal(self.value2._price),
                price_token: Some(self.value2.token.to_string().into()),
                nft: event_record.nft.clone(),
                collection: event_record.collection.clone(),
            };

            indexer_repo::actions::upsert_nft_price_history(&price_history, &mut pg_pool_tx)
                .await?;
        }

        // HACK: turn off the usd price request
        let (sell_price_usd, finished_at) = (None, None);
        // if state == DirectSellState::Filled && false {
        //     let usd_price = rpc::token_to_usd(&self.value2.token.to_string())
        //         .await
        //         .unwrap_or_default();
        //     (
        //         Some(usd_price * u128_to_bigdecimal(self.value2._price)),
        //         Some(created_ts),
        //     )
        // } else {
        //     (None, None)
        // };

        let direct_sell = NftDirectSell {
            address: event_record.address.clone(),
            nft: event_record.nft.clone().unwrap(),
            collection: event_record.nft.clone(),
            price_token: self.value2.token.to_string().into(),
            price: u128_to_bigdecimal(self.value2._price),
            sell_price_usd,
            seller: self.value2.creator.to_string().into(),
            finished_at,
            expired_at: NaiveDateTime::from_timestamp_opt(self.value2.end as i64, 0)
                .unwrap_or_default(),
            state,
            created: NaiveDateTime::from_timestamp_opt(self.value2.start as i64, 0)
                .unwrap_or_default(),
            updated: created_ts,
            tx_lt: event_record.created_lt,
        };

        indexer_repo::actions::upsert_direct_sell(&direct_sell, &mut pg_pool_tx).await?;

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
            .expect("Failed to save DirectSellStateChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
