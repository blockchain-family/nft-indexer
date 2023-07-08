use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{EventCategory, EventRecord, EventType};
use sqlx::PgPool;
use transaction_consumer::JrpcClient;

use crate::{
    models::events::{
        AddCollectionRules, MarketFeeChanged, MarketFeeDefaultChanged, OwnershipTransferred,
        RemoveCollectionRules,
    },
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for OwnershipTransferred {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        _jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Common,
            event_type: EventType::OwnershipTransferred,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save OwnershipTransferred event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for MarketFeeDefaultChanged {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        _jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::MarketFeeDefaultChanged,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save MarketFeeChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for MarketFeeChanged {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        _jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::MarketFeeChanged,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save MarketFeeChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for AddCollectionRules {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        _jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::AddCollectionRules,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        indexer_repo::actions::update_collection_fee(
            Some(self.collection_fee_info.numerator as i32),
            Some(self.collection_fee_info.denominator as i32),
            &event_record.collection.clone().unwrap(),
            &mut pg_pool_tx,
        )
        .await?;

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save MarketFeeChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for RemoveCollectionRules {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        _jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::RemoveCollectionRules,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        indexer_repo::actions::update_collection_fee(
            None,
            None,
            &event_record.collection.clone().unwrap(),
            &mut pg_pool_tx,
        )
        .await?;

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save MarketFeeChanged event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
