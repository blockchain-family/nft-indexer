use crate::service::MetadataRpcService;
use anyhow::{Result, bail};
use everscale_rpc_client::RpcClient;
use indexer_repo::{
    meta::{MetadataModelService, NftMeta, NftMetaAttribute},
    types::NftCollectionMeta,
};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction, types::chrono};
use std::{str::FromStr, time::Duration};
use ton_block::MsgAddressInt;

const NFT_PER_ITERATION: i64 = 30_000;
const COLLECTION_PER_ITERATION: i64 = 100;

#[derive(Clone)]
pub struct MetaUpdaterContext {
    pub jrpc_client: RpcClient,
    pub http_client: reqwest::Client,
    pub pool: PgPool,
    pub jrpc_req_latency_millis: u64,
    pub idle_after_loop: u64,
}

pub async fn run_meta_reader(meta_updater: MetaUpdater) -> Result<()> {
    log::info!("Run metadata reader");
    let MetaUpdater {
        context,
        meta_model_service,
        ..
    } = &meta_updater;

    loop {
        let nft_addresses = meta_model_service
            .get_nfts_for_meta_update(NFT_PER_ITERATION)
            .await?;

        meta_updater
            .update_nfts_meta(
                &nft_addresses.iter().map(|n| n.as_str()).collect::<Vec<_>>(),
                None,
            )
            .await;

        let collection_addresses = meta_model_service
            .get_collections_for_meta_update(COLLECTION_PER_ITERATION)
            .await?;

        meta_updater
            .update_collections_meta(
                &collection_addresses
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
                true,
                None,
            )
            .await;

        if nft_addresses.len() < NFT_PER_ITERATION as usize && collection_addresses.is_empty() {
            log::info!("Finished updating metadata work. Idling");
            tokio::time::sleep(Duration::from_secs(context.idle_after_loop)).await;

            continue;
        }
    }
}

#[derive(Clone)]
pub struct MetaUpdater {
    pub meta_model_service: MetadataModelService,
    pub meta_jrpc_service: MetadataRpcService,
    pub context: MetaUpdaterContext,
}

impl MetaUpdater {
    pub fn new(context: MetaUpdaterContext) -> Self {
        let meta_jrpc_service =
            MetadataRpcService::new(context.jrpc_client.clone(), reqwest::Client::new());
        let meta_model_service = MetadataModelService::new(context.pool.clone());

        Self {
            meta_jrpc_service,
            meta_model_service,
            context,
        }
    }

    pub async fn update_collections_meta(
        &self,
        addresses: &[&str],
        collection_only: bool,
        mut existing_tx: Option<&mut Transaction<'_, Postgres>>,
    ) {
        for &address in addresses.iter() {
            if let Err(e) = self
                .update_collection_meta(address, collection_only, existing_tx.as_deref_mut())
                .await
            {
                log::error!("{:#?}", e);
            }

            tokio::time::sleep(Duration::from_millis(self.context.jrpc_req_latency_millis)).await;
        }
    }

    pub async fn update_collection_meta(
        &self,
        address: &str,
        collection_only: bool,
        existing_tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<()> {
        let Ok(collection_address) = MsgAddressInt::from_str(address) else {
            bail!(
                "Error while converting collection address {} to MsgAddressInt",
                address
            );
        };

        let mut failed = false;

        let (owner, meta) = match self
            .meta_jrpc_service
            .get_collection_meta(collection_address)
            .await
        {
            Ok(meta) => meta,
            Err(e) => {
                log::error!("Error while reading {address} collection meta: {:#?}", e);
                failed = true;
                (None, Value::default())
            }
        };

        let mut owned_tx = None;
        let mut tx = match existing_tx {
            None => {
                let Ok(tx) = self.context.pool.begin().await else {
                    bail!("Cant start transaction for saving metadata");
                };
                owned_tx = Some(tx);
                owned_tx.as_mut()
            }
            Some(tx) => Some(tx),
        };

        let now = chrono::Utc::now().naive_utc();

        let collection = NftCollectionMeta {
            address: address.into(),
            owner,
            name: meta
                .get("name")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            description: meta
                .get("description")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(str::to_string),
            logo: meta
                .get("preview")
                .cloned()
                .unwrap_or_default()
                .get("source")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(|s| s.into()),
            wallpaper: meta
                .get("files")
                .cloned()
                .unwrap_or_default()
                .as_array()
                .cloned()
                .unwrap_or_default()
                .first()
                .cloned()
                .unwrap_or_default()
                .get("source")
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(|s| s.into()),
            royalty: meta.get("royalty").cloned(),
            updated: now,
        };

        if !failed {
            if let Err(e) = self
                .meta_model_service
                .update_collection(&collection, tx.as_deref_mut())
                .await
            {
                bail!(
                    "Collection address: {}, error while updating collection meta: {:#?}",
                    address,
                    e
                );
            };
        }

        if let Err(e) = self
            .meta_model_service
            .add_to_proceeded(address, Some(failed), tx.as_deref_mut())
            .await
        {
            bail!(
                "Collection address: {}, error while adding to meta_handled_addresses table: {:#?}",
                address,
                e
            );
        };

        if !collection_only {
            let nfts = self
                .meta_model_service
                .get_nfts_by_collection(address)
                .await?;

            self.update_nfts_meta(&nfts.iter().map(|n| n.as_str()).collect::<Vec<_>>(), tx)
                .await;
        }

        if let Some(tx) = owned_tx {
            if let Err(e) = tx.commit().await {
                bail!(
                    "Nft address: {}, error while commiting transaction: {e:#?}",
                    &address,
                );
            };
        }

        Ok(())
    }

    pub async fn update_nfts_meta(
        &self,
        addresses: &[&str],
        mut existing_tx: Option<&mut Transaction<'_, Postgres>>,
    ) {
        for &address in addresses.iter() {
            if let Err(e) = self
                .update_nft_meta(address, existing_tx.as_deref_mut())
                .await
            {
                log::error!("Error of update nft meta. Reason:{:#?}", e);
            }

            tokio::time::sleep(Duration::from_millis(self.context.jrpc_req_latency_millis)).await;
        }
    }

    pub async fn update_nft_meta(
        &self,
        address: &str,
        existing_tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<()> {
        let Ok(nft_address) = MsgAddressInt::from_str(address) else {
            bail!(
                "Error while converting nft address {} to MsgAddressInt",
                address
            );
        };

        let mut failed = false;

        let meta = self
            .meta_jrpc_service
            .get_nft_meta(&nft_address)
            .await
            .map_err(|e| {
                log::error!("Error while reading {} nft meta: {e:#?}", address);
                failed = true;
            })
            .unwrap_or_default();

        let mut owned_tx = None;
        let mut tx = match existing_tx {
            None => {
                let Ok(tx) = self.context.pool.begin().await else {
                    bail!("Cant start transaction for saving metadata");
                };
                owned_tx = Some(tx);
                owned_tx.as_mut()
            }
            Some(tx) => Some(tx),
        };

        if !failed {
            let updated = chrono::Utc::now().naive_utc();

            let name = meta.get("name").and_then(|d| d.as_str());
            let description = meta.get("description").and_then(|d| d.as_str());
            if let Err(e) = self
                .meta_model_service
                .update_nft_basic_meta(address, name, description, updated, tx.as_deref_mut())
                .await
            {
                bail!(
                    "Nft address: {}, error while updating name and/or description: {e:#?}",
                    &address,
                );
            }

            let attributes = Self::extract_attributes_from_meta(&meta);
            if let Err(e) = self
                .meta_model_service
                .update_nft_attributes(address, &attributes, updated, tx.as_deref_mut())
                .await
            {
                bail!(
                    "Nft address: {}, error while updating attributes: {e:#?}",
                    &address,
                );
            }

            let nft_meta = NftMeta {
                address,
                meta: &meta,
                updated,
            };
            if let Err(e) = self
                .meta_model_service
                .update_nft_meta(&nft_meta, tx.as_deref_mut())
                .await
            {
                bail!(
                    "Nft address: {}, error while updating nft meta: {e:#?}",
                    &address,
                );
            };
        }

        if let Err(e) = self
            .meta_model_service
            .add_to_proceeded(address, Some(failed), tx)
            .await
        {
            bail!(
                "Nft address: {}, error while adding to meta_handled_addresses table: {e:#?}",
                &address,
            );
        };

        if let Some(tx) = owned_tx {
            if let Err(e) = tx.commit().await {
                bail!(
                    "Nft address: {}, error while commiting transaction: {e:#?}",
                    &address,
                );
            };
        }

        Ok(())
    }

    pub fn extract_attributes_from_meta(meta: &Value) -> Vec<NftMetaAttribute> {
        meta.get("attributes")
            .and_then(|d| d.as_array())
            .map(|d| {
                d.iter()
                    .filter_map(NftMetaAttribute::new)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaUpdater;
    use serde_json::json;

    #[test]
    pub fn extract_no_attributes_from_meta_empty_test() {
        let meta = json!({});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn extract_no_attributes_from_meta_empty_field_test() {
        let meta = json!({"attributes": []});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn extract_correct_attribute_from_meta_test() {
        let meta = json!({"attributes": [{"trait_type": "eyes", "value": "blue"}]});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].raw,
            &json!({"trait_type": "eyes", "value": "blue"})
        );
        assert_eq!(result[0].trait_type, "eyes");
        assert_eq!(result[0].value, "blue");
    }

    #[test]
    pub fn extract_correct_attribute_from_meta_test2() {
        let meta = json!({"attributes": [{"trait_type": "eyes", "display_value": "green"}]});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].raw,
            &json!({"trait_type": "eyes", "display_value": "green"})
        );
        assert_eq!(result[0].trait_type, "eyes");
        assert_eq!(result[0].value, "green");
    }

    #[test]
    pub fn dont_extract_corrupted_attribute_from_meta_test() {
        let meta = json!({"attributes": [{"trait_typo": "eyes", "volume": "blue"}]});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn dont_extract_incomplete_attribute_from_meta_test() {
        let meta = json!({"attributes": [{"trait_type": "eyes"}]});
        let result = MetaUpdater::extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }
}
