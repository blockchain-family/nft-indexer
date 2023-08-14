use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{
    EventCategory, EventRecord, EventType, Nft, NftBurnedDecoded, NftCollection, NftCreateDecoded,
};
use sqlx::PgPool;

use crate::{
    models::events::{NftBurned, NftCreated},
    utils::{EventMessageInfo, KeyInfo},
};

use super::{types::Decoded, Decode, Entity};

impl Decode for NftCreated {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let logical_time = msg_info.tx_data.logical_time() as i64;
        let record = NftCreateDecoded {
            address: self.nft.to_string(),
            collection: msg_info.tx_data.get_account(),
            owner: self.owner.to_string(),
            manager: self.manager.to_string(),
            updated: NaiveDateTime::from_timestamp_opt(msg_info.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            owner_update_lt: logical_time,
            manager_update_lt: logical_time,
        };

        Ok(Decoded::CreateNft(record))
    }

    fn decode_event(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftCreated,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: Some(msg_info.tx_data.get_account().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for NftBurned {
    fn decode(&self, _: &EventMessageInfo) -> Result<Decoded> {
        let record = NftBurnedDecoded {
            address: self.nft.to_string(),
            owner: self.owner.to_string(),
            manager: self.manager.to_string(),
        };

        Ok(Decoded::BurnNft(record))
    }

    fn decode_event(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftBurned,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: Some(msg_info.tx_data.get_account().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

#[async_trait]
impl Entity for NftCreated {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftCreated,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: Some(msg_info.tx_data.get_account().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let nft = Nft {
            address: event_record.nft.clone().unwrap(),
            collection: event_record.collection.clone(),
            owner: Some(self.owner.to_string().into()),
            manager: Some(self.manager.to_string().into()),
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            owner_update_lt: event_record.created_lt,
            manager_update_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let now = chrono::Utc::now().naive_utc();

            let collection = NftCollection {
                address: collection.clone(),
                created: now,
                updated: now,
                ..Default::default()
            };

            let nft_created_at_timestamp =
                NaiveDateTime::from_timestamp_opt(event_record.created_at, 0);

            indexer_repo::actions::upsert_collection(
                &collection,
                &mut pg_pool_tx,
                nft_created_at_timestamp,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save NftCreated event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        indexer_repo::actions::update_collection_by_nft(
            "nft_events",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_direct_sell",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_direct_buy",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_price_history",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_attributes",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for NftBurned {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftBurned,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: Some(msg_info.tx_data.get_account().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let nft = Nft {
            address: event_record.nft.clone().unwrap(),
            collection: event_record.collection.clone(),
            owner: Some(self.owner.to_string().into()),
            manager: Some(self.manager.to_string().into()),
            burned: true,
            updated: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            owner_update_lt: event_record.created_lt,
            manager_update_lt: event_record.created_lt,
            ..Default::default()
        };

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;

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
            .expect("Failed to save NftBurned event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        indexer_repo::actions::update_collection_by_nft(
            "nft_events",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_direct_sell",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_direct_buy",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        indexer_repo::actions::update_collection_by_nft(
            "nft_price_history",
            event_record.nft.as_ref().unwrap(),
            &event_record.address,
            &mut pg_pool_tx,
        )
        .await?;

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
