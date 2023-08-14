use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{Address, EventCategory, EventRecord, EventType};
use sqlx::PgPool;

use crate::persistence::entities::{Decode, Decoded};
use crate::{
    models::events::{DirectBuyDeclined, DirectBuyDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{DecodeContext, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for DirectBuyDeployed {
    async fn save_to_db(&self, pg_pool: &PgPool, ctx: &DecodeContext) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeployed,

            address: ctx.tx_data.get_account().into(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.nft.to_string().into(),
                &mut pg_pool_tx,
            )
            .await,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectBuy]
            .contains(&event_record.address.0)
        {
            indexer_repo::actions::add_whitelist_address(
                &self.direct_buy.to_string().into(),
                &mut pg_pool_tx,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save DirectBuyDeployed event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for DirectBuyDeclined {
    async fn save_to_db(&self, pg_pool: &PgPool, ctx: &DecodeContext) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

            address: ctx.tx_data.get_account().into(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save DirectBuyDeclined event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

impl Decode for DirectBuyDeployed {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let emitter_address: Address = ctx.tx_data.get_account().into();

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectBuy]
            .contains(&emitter_address.0)
        {
            Ok(Decoded::DirectBuyDeployed(emitter_address))
        } else {
            Ok(Decoded::ShouldSkip)
        }
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for DirectBuyDeclined {
    fn decode(&self, _msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
