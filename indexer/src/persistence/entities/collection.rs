use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use indexer_repo::types::{EventCategory, EventRecord, EventType, Nft, NftAttribute};
use sqlx::PgPool;
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

use crate::{
    metadata::service::{fetch_metadata, get_collection_data},
    models::events::{NftBurned, NftCreated},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for NftCreated {
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        jrpc_client: &JrpcClient,
    ) -> Result<()> {
        // let collections_whitelist = vec![
        //     "0:ec0ab798c85aa7256865221bacd4f3df220cf60277a2b79b3091b76c265d1cd7",
        //     "0:33a630f9c54fc4092f43ab978f3fd65964bb0d775553c16953aa1568eb63ab0f",
        //     "0:d62691c79f447f512d7ad235a291435a8a886debff1b72dfc3ff5e486798d96e",
        //     "0:7eb6488246ba08f88fe8779e9257ca9ebc7d2f82f6111ce6747abda368e3c7a8",
        //     "0:3edef5a608cf6627edf41f0cd019e9a5c2baf955f80952dff0d8b034e7d1f808",
        //     "0:180742c2f9cfeeb2dbf50c01785d01f59224381deb1d04f0f00f0a4413503377"
        // ];

        // if let Some(event_collection) = &self.event_collection {
        //     let mut is_in_whitelist = false;
        //     for collection in &collections_whitelist {
        //         if event_collection.0.as_str() == *collection {
        //             is_in_whitelist = true;
        //             break;
        //         }
        //     }
        //     if !is_in_whitelist {
        //         log::debug!(
        //             "Skip nft {} for collection {}",
        //             self.address.0.as_str(),
        //             event_collection.0.as_str()
        //         );
        //         return Ok(());
        //     }
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
            collection: Some(msg_info.tx_data.get_account().into()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

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
                let collection = get_collection_data(
                    MsgAddressInt::from_str(event_record.address.0.as_str())?,
                    jrpc_client,
                )
                .await;

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
    async fn save_to_db(
        &self,
        pg_pool: &PgPool,
        msg_info: &EventMessageInfo,
        jrpc_client: &JrpcClient,
    ) -> Result<()> {
        let meta = fetch_metadata(self.nft.clone(), jrpc_client).await;

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

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| {
                    NftAttribute::new(
                        self.nft.to_string().into(),
                        event_record.collection.clone(),
                        item.clone(),
                    )
                })
                .collect();

            indexer_repo::actions::upsert_nft_attributes(&nft_attributes, &mut pg_pool_tx).await?;
        }

        // let nft_meta = NftMeta {
        //     nft: self.nft.clone(),
        //     meta,
        //     updated: chrono::Utc::now().naive_utc(),
        // };

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

        //     actions::upsert_nft_meta(&nft_meta, &mut tx).await?;

        if let Some(collection) = event_record.collection.as_ref() {
            let exists = indexer_repo::actions::check_collection_exists(
                collection.0.as_str(),
                &mut pg_pool_tx,
            )
            .await
            .expect("Failed to check collection exists for collection {collection:?}");
            if !exists {
                let collection = get_collection_data(
                    MsgAddressInt::from_str(event_record.address.0.as_str())?,
                    jrpc_client,
                )
                .await;

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
