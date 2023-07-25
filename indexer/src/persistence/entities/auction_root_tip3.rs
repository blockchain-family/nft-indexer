use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{EventCategory, EventRecord, EventType};
use sqlx::PgPool;

use crate::{
    models::events::{AuctionDeclined, AuctionDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for AuctionDeployed {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionDeployed,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.offer_info.nft.to_string().into()),
            collection: Some(self.offer_info.collection.to_string().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

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

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::AuctionRoot]
            .contains(&event_record.address.0)
        {
            indexer_repo::actions::add_whitelist_address(
                &self.offer.to_string().into(),
                &mut pg_pool_tx,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionDeployed event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for AuctionDeclined {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save AuctionDeclined event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}
