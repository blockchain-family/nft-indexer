use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{EventCategory, EventRecord, EventType};
use sqlx::PgPool;

use crate::{
    models::events::{DirectBuyDeclined, DirectBuyDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for DirectBuyDeployed {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeployed,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.nft.to_string().into(),
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
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

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
