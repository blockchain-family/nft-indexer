use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{
    DirectBuyState, EventCategory, EventRecord, EventType, NftDirectBuy, NftPriceHistory,
    NftPriceSource,
};
use sqlx::PgPool;

use crate::{
    models::events::DirectBuyStateChanged,
    utils::{u128_to_bigdecimal, EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for DirectBuyStateChanged {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyStateChanged,

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

        let state = self.to.into();
        let created_ts =
            NaiveDateTime::from_timestamp_opt(event_record.created_at, 0).unwrap_or_default();

        if state != DirectBuyState::Create {
            let price_history = NftPriceHistory {
                source: event_record.address.clone(),
                source_type: NftPriceSource::DirectBuy,
                created_at: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                    .unwrap_or_default(),
                price: u128_to_bigdecimal(self.value2._price),
                price_token: Some(self.value2.spent_token.to_string().into()),
                nft: event_record.nft.clone(),
                collection: event_record.collection.clone(),
            };
            indexer_repo::actions::upsert_nft_price_history(&price_history, &mut pg_pool_tx)
                .await?;
        }

        let direct_buy = NftDirectBuy {
            address: event_record.address.clone(),
            nft: event_record.nft.clone().unwrap(),
            collection: event_record.collection.clone(),
            price_token: self.value2.spent_token.to_string().into(),
            price: u128_to_bigdecimal(self.value2._price),
            buy_price_usd: None,
            buyer: self.value2.creator.to_string().into(),
            finished_at: None,
            expired_at: NaiveDateTime::from_timestamp_opt(self.value2.end_time_buy as i64, 0)
                .unwrap_or_default(),
            state,
            created: NaiveDateTime::from_timestamp_opt(self.value2.start_time_buy as i64, 0)
                .unwrap_or_default(),
            updated: created_ts,
            tx_lt: event_record.created_lt,
        };
        indexer_repo::actions::upsert_direct_buy(&direct_buy, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                // let collection = get_collection_data(
                //     MsgAddressInt::from_str(collection.0.as_str())?,
                //     &self.consumer,
                // )
                // .await;
                //     actions::upsert_collection(&collection, &mut tx, None).await?;
            }
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save DirectBuyStateChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
