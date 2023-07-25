use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{EventCategory, EventRecord, EventType, Nft};
use sqlx::PgPool;

use crate::{
    models::events::{ManagerChanged, OwnerChanged},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for OwnerChanged {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        // let meta = fetch_metadata(
        //     MsgAddressInt::from_str(self.address.0.as_str())?,
        //     &self.consumer,
        // )
        // .await;

        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Nft,
            event_type: EventType::NftOwnerChanged,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(msg_info.tx_data.get_account().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &msg_info.tx_data.get_account().into(),
                &mut pg_pool_tx,
            )
            .await,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        // if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
        //     let nft_attributes: Vec<NftAttribute> = attributes
        //         .iter()
        //         .map(|item| {
        //             NftAttribute::new(
        //                 self.address.clone(),
        //                 self.event_collection.clone(),
        //                 item.clone(),
        //             )
        //         })
        //         .collect();

        //     actions::upsert_nft_attributes(&nft_attributes, &mut tx).await;

        // let nft_meta = NftMeta {
        //     nft: self.address.clone(),
        //     meta,
        //     updated: chrono::Utc::now().naive_utc(),
        // };

        let nft = Nft {
            address: event_record.address.clone(),
            collection: event_record.collection.clone(),
            owner: Some(self.new_owner.to_string().into()),
            manager: None,
            // name: nft_meta
            //     .meta
            //     .get("name")
            //     .cloned()
            //     .unwrap_or_default()
            //     .as_str()
            //     .map(str::to_string),
            // description: nft_meta
            //     .meta
            //     .get("description")
            //     .cloned()
            //     .unwrap_or_default()
            //     .as_str()
            //     .map(str::to_string),
            name: None,
            description: None,
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            owner_update_lt: event_record.created_lt,
            manager_update_lt: 0,
        };

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;
        //     actions::upsert_nft_meta(&nft_meta, &mut tx).await;

        if let Some(event_collection) = &event_record.collection {
            indexer_repo::actions::refresh_collection_owners_count(
                event_collection,
                &mut pg_pool_tx,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save NftOwnerChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for ManagerChanged {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        // let meta = fetch_metadata(
        //     MsgAddressInt::from_str(self.address.0.as_str())?,
        //     &self.consumer,
        // )
        // .await;

        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Nft,
            event_type: EventType::NftManagerChanged,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(msg_info.tx_data.get_account().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &msg_info.tx_data.get_account().into(),
                &mut pg_pool_tx,
            )
            .await,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        // if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
        //     let nft_attributes: Vec<NftAttribute> = attributes
        //         .iter()
        //         .map(|item| {
        //             NftAttribute::new(
        //                 self.address.clone(),
        //                 self.event_collection.clone(),
        //                 item.clone(),
        //             )
        //         })
        //         .collect();
        //
        //         actions::upsert_nft_attributes(&nft_attributes, &mut tx).await?;
        // }
        //
        // let nft_meta = NftMeta {
        //     nft: self.address.clone(),
        //     meta,
        //     updated: chrono::Utc::now().naive_utc(),
        // };

        let nft = Nft {
            address: event_record.address.clone(),
            collection: event_record.collection.clone(),
            owner: None,
            manager: Some(self.new_manager.to_string().into()),
            // name: nft_meta
            //     .meta
            //     .get("name")
            //     .cloned()
            //     .unwrap_or_default()
            //     .as_str()
            //     .map(str::to_string),
            // description: nft_meta
            //     .meta
            //     .get("description")
            //     .cloned()
            //     .unwrap_or_default()
            //     .as_str()
            //     .map(str::to_string),
            name: None,
            description: None,
            burned: false,
            updated: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            owner_update_lt: 0,
            manager_update_lt: event_record.created_lt,
        };

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;
        //     actions::upsert_nft_meta(&nft_meta, &mut tx).await?;

        if let Some(event_collection) = &event_record.collection {
            indexer_repo::actions::refresh_collection_owners_count(
                event_collection,
                &mut pg_pool_tx,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save NftManagerChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
