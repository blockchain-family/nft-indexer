use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{EventCategory, EventRecord, EventType, Nft, NftCollection};
use sqlx::PgPool;

use crate::{
    models::events::{NftBurned, NftCreated},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for NftCreated {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        // TODO: remove it!
        let collection: Option<indexer_repo::types::Address> =
            Some(msg_info.tx_data.get_account().into());

        // let collections_whitelist = vec![
        //     "0:9eaf3e084cbe25e67cb8730123f65b75429906abc2b01211cccfd3c97047762c",
        //     "0:e2611558851f4547c6a13b833189136103dcad4350eba36bbb7bf35b6be98ce1",
        //     "0:48400246d51dd380ad49261f5e6f026347d3a5be5614d82bd655dcb57819e4bf",
        // ];

        // if let Some(event_collection) = &collection {
        //     let mut is_in_whitelist = false;
        //     for collection in &collections_whitelist {
        //         if event_collection.0.as_str() == *collection {
        //             is_in_whitelist = true;
        //             break;
        //         }
        //     }
        //     if !is_in_whitelist {
        //         log::debug!(
        //             "Skip nft {:?} for collection {}",
        //             self.nft,
        //             event_collection.0
        //         );
        //         return Ok(());
        //     }
        // } else {
        //     return Ok(());
        // }
        // let meta = fetch_metadata(
        //     MsgAddressInt::from_str(self.nft.0.as_str())?,
        //     &self.consumer,
        // )
        // .await;

        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftCreated,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        // let nfts_whitelist: std::collections::HashSet<
        //     &str,
        //     std::collections::hash_map::RandomState,
        // > = std::collections::HashSet::from_iter(
        //     [
        //         "0:69887fccaff1a7ecc337f08f0951050c5f48fe1a0247d2b5f27b50a087ae5200",
        //         "0:f686736f8bd02f33c7db19fa06cda8aae4a515b1e310ffd59939be25d4f553be",
        //         "0:e18b796d280e2979c612d63a6b3d6ed414cef2e94c1fdec2693af3eb6a376f74",
        //         "0:6760bec41ee7795f9b2f18934cae9ed4d3bad8a8124021efeb9641c28abd3d28",
        //         "0:c49d70bfa5a54021307395fd1d63a0a95b2c7bff0df98849fb26d307c626ed19",
        //         "0:7ae2b15e16a73bb89666d50ecd0eb8e3caa11fb9ef9a774966815fd3a402f8aa",
        //     ]
        //     .into_iter(),
        // );

        // if let Some(nft) = &event_record.nft {
        //     if !nfts_whitelist.contains(nft.0.as_str()) {
        //         return Ok(());
        //     }
        // } else {
        //     return Ok(());
        // }

        let nft = Nft {
            address: event_record.nft.clone().unwrap(),
            collection: event_record.collection.clone(),
            owner: Some(self.owner.to_string().into()),
            manager: Some(self.manager.to_string().into()),
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
            manager_update_lt: event_record.created_lt,
        };

        // let mut nft_meta = None;

        // if let Some(collection) = self.event_collection.as_ref() {
        //     let mut is_in_whitelist = false;
        //     for c in &collections_whitelist {
        //         if collection.0.as_str() == *c {
        //             is_in_whitelist = true;
        //             break;
        //         }
        //     }
        //     if is_in_whitelist {
        //         // let meta = fetch_metadata(
        //         //     MsgAddressInt::from_str(self.nft.0.as_str())?,
        //         //     &self.consumer,
        //         // )
        //         // .await;
        //
        //         nft_meta = Some(NftMeta {
        //             nft: self.nft.clone(),
        //             meta: meta.clone(),
        //             updated: chrono::Utc::now().naive_utc(),
        //         });
        //
        //         nft = Nft {
        //             address: self.nft.clone(),
        //             collection: self.event_collection.clone(),
        //             owner: Some(self.owner.clone()),
        //             manager: Some(self.manager.clone()),
        //             name: nft_meta.clone().unwrap()
        //                 .clone()
        //                 .meta
        //                 .get("name")
        //                 .cloned()
        //                 .unwrap_or_default()
        //                 .as_str()
        //                 .map(str::to_string),
        //             description: nft_meta.clone().unwrap()
        //                 .clone()
        //                 .meta
        //                 .get("description")
        //                 .cloned()
        //                 .unwrap_or_default()
        //                 .as_str()
        //                 .map(str::to_string),
        //             burned: false,
        //             updated: NaiveDateTime::from_timestamp_opt(self.created_at, 0)
        //                 .unwrap_or_default(),
        //             owner_update_lt: self.created_lt,
        //             manager_update_lt: self.created_lt,
        //         };
        //
        //         if let Some(attributes) = meta.clone().get("attributes").and_then(|v| v.as_array())
        //         {
        //             let nft_attributes: Vec<NftAttribute> = attributes
        //                 .iter()
        //                 .map(|item| {
        //                     NftAttribute::new(
        //                         self.nft.clone(),
        //                         self.event_collection.clone(),
        //                         item.clone(),
        //                     )
        //                 })
        //                 .collect();
        //
        //                 actions::upsert_nft_attributes(&nft_attributes, &mut tx).await?;
        //         }
        //     }
        // }

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;

        // if let Some(nft_meta) = nft_meta {
        //         actions::upsert_nft_meta(&nft_meta, &mut tx).await?;
        // }

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

                let nft_created_at_timestamp =
                    NaiveDateTime::from_timestamp_opt(event_record.created_at, 0);

                indexer_repo::actions::upsert_collection(
                    &collection,
                    &mut pg_pool_tx,
                    nft_created_at_timestamp,
                )
                .await?;
            }
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
        let collection: Option<indexer_repo::types::Address> =
            Some(msg_info.tx_data.get_account().into());

        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftBurned,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let nft = Nft {
            address: event_record.nft.clone().unwrap(),
            collection: event_record.collection.clone(),
            owner: Some(self.owner.to_string().into()),
            manager: Some(self.manager.to_string().into()),
            name: None,
            description: None,
            burned: true,
            updated: NaiveDateTime::from_timestamp_opt(event_record.created_at, 0)
                .unwrap_or_default(),
            owner_update_lt: event_record.created_lt,
            manager_update_lt: event_record.created_lt,
        };

        indexer_repo::actions::upsert_nft(&nft, &mut pg_pool_tx).await?;

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
