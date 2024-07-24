use std::{str::FromStr, time::Duration};

use crate::service::MetadataRpcService;
use anyhow::{bail, Result};
use everscale_rpc_client::RpcClient;
use indexer_repo::{
    meta::{MetadataModelService, NftAddressData, NftMeta, NftMetaAttribute},
    types::NftCollectionMeta,
};
use serde_json::Value;
use sqlx::{types::chrono, PgPool};
use ton_block::MsgAddressInt;

const NFT_PER_ITERATION: i64 = 30_000;
const COLLECTION_PER_ITERATION: i64 = 100;

#[derive(Clone)]
pub struct MetaReaderContext {
    pub jrpc_client: RpcClient,
    pub http_client: reqwest::Client,
    pub pool: PgPool,
    pub jrpc_req_latency_millis: u64,
    pub idle_after_loop: u64,
}

pub async fn run_meta_reader(context: MetaReaderContext) -> Result<()> {
    log::info!("Run metadata reader");
    let meta_jrpc_service = MetadataRpcService::new(context.jrpc_client, context.http_client);
    let meta_model_service = MetadataModelService::new(context.pool.clone());

    loop {
        let nft_addresses = meta_model_service
            .get_nfts_for_meta_update(NFT_PER_ITERATION)
            .await?;

        for address_data in nft_addresses.iter() {
            if let Err(e) =
                update_nft_meta(address_data, &meta_model_service, &meta_jrpc_service).await
            {
                log::error!("Error of update nft meta. Reason:{:#?}", e);

                let Ok(mut tx) = meta_model_service.start_transaction().await else {
                    log::error!("Cant start transaction for saving metadata");
                    tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis))
                        .await;
                    continue;
                };

                if let Err(e) = tx.add_to_proceeded(&address_data.nft, Some(true)).await {
                    log::error!("Collection address: {}, error while adding to meta_handled_addresses table: {:#?}",
                        address_data.nft,
                        e
                    );
                }
            }

            tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis)).await;
        }

        let collection_addresses = meta_model_service
            .get_collections_for_meta_update(COLLECTION_PER_ITERATION)
            .await?;

        for address in collection_addresses.iter() {
            if let Err(e) =
                update_collections_meta(address, &meta_model_service, &meta_jrpc_service).await
            {
                log::error!("{:#?}", e);

                let Ok(mut tx) = meta_model_service.start_transaction().await else {
                    log::error!("Cant start transaction for saving metadata");
                    tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis))
                        .await;
                    continue;
                };

                if let Err(e) = tx.add_to_proceeded(address, Some(true)).await {
                    log::error!("Collection address: {}, error while adding to meta_handled_addresses table: {:#?}",
                        address,
                        e
                    );
                }
            }

            tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis)).await;
        }

        if nft_addresses.len() < NFT_PER_ITERATION as usize && collection_addresses.is_empty() {
            log::info!("Finished updating metadata work. Idling");
            tokio::time::sleep(Duration::from_secs(context.idle_after_loop)).await;

            continue;
        }
    }
}

pub async fn update_collections_meta(
    address: &str,
    meta_model_service: &MetadataModelService,
    meta_jrpc_service: &MetadataRpcService,
) -> Result<()> {
    let Ok(collection_address) = MsgAddressInt::from_str(address) else {
        bail!(
            "Error while converting collection address {} to MsgAddressInt",
            address
        );
    };

    let mut failed = false;

    let (owner, meta) = match meta_jrpc_service
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

    let Ok(mut tx) = meta_model_service.start_transaction().await else {
        bail!("Cant start transaction for saving metadata");
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
        if let Err(e) = tx.update_collection(&collection).await {
            bail!(
                "Collection address: {}, error while updating collection meta: {:#?}",
                address,
                e
            );
        };
    }

    if let Err(e) = tx.add_to_proceeded(address, Some(failed)).await {
        bail!(
            "Collection address: {}, error while adding to meta_handled_addresses table: {:#?}",
            address,
            e
        );
    };

    if let Err(e) = tx.commit().await {
        bail!(
            "Collection address: {}, error while commiting transaction: {:#?}",
            address,
            e
        );
    };

    Ok(())
}

pub async fn update_nft_meta(
    address_data: &NftAddressData,
    meta_model_service: &MetadataModelService,
    meta_jrpc_service: &MetadataRpcService,
) -> Result<()> {
    let Ok(nft_address) = MsgAddressInt::from_str(&address_data.nft) else {
        bail!(
            "Error while converting nft address {} to MsgAddressInt",
            address_data.nft
        );
    };

    let mut failed = false;

    let meta = meta_jrpc_service
        .get_nft_meta(&nft_address)
        .await
        .map_err(|e| {
            log::error!("Error while reading {} nft meta: {e:#?}", address_data.nft);
            failed = true;
        })
        .unwrap_or_default();

    let Ok(mut tx) = meta_model_service.start_transaction().await else {
        bail!("Cant start transaction for saving metadata");
    };

    if !failed {
        let updated = chrono::Utc::now().naive_utc();

        let name = meta.get("name").and_then(|d| d.as_str());
        let description = meta.get("description").and_then(|d| d.as_str());
        if let Err(e) = tx
            .update_nft_basic_meta(&address_data.nft, name, description, updated)
            .await
        {
            bail!(
                "Nft address: {}, error while updating name and/or description: {e:#?}",
                &address_data.nft,
            );
        }

        let attributes = extract_attributes_from_meta(&meta);
        if let Err(e) = tx
            .update_nft_attributes(address_data, &attributes, updated)
            .await
        {
            bail!(
                "Nft address: {}, error while updating attributes: {e:#?}",
                &address_data.nft,
            );
        }

        let nft_meta = NftMeta {
            address: &address_data.nft,
            meta: &meta,
            updated,
        };
        if let Err(e) = tx.update_nft_meta(&nft_meta).await {
            bail!(
                "Nft address: {}, error while updating nft meta: {e:#?}",
                &address_data.nft,
            );
        };
    }

    if let Err(e) = tx.add_to_proceeded(&address_data.nft, Some(failed)).await {
        bail!(
            "Nft address: {}, error while adding to meta_handled_addresses table: {e:#?}",
            &address_data.nft,
        );
    };

    if let Err(e) = tx.commit().await {
        bail!(
            "Nft address: {}, error while commiting transaction: {e:#?}",
            &address_data.nft,
        );
    };

    Ok(())
}

fn extract_attributes_from_meta(meta: &Value) -> Vec<NftMetaAttribute> {
    meta.get("attributes")
        .and_then(|d| d.as_array())
        .map(|d| {
            d.iter()
                .filter_map(NftMetaAttribute::new)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::meta::extract_attributes_from_meta;
    use serde_json::json;

    #[test]
    pub fn extract_no_attributes_from_meta_empty_test() {
        let meta = json!({});
        let result = extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn extract_no_attributes_from_meta_empty_field_test() {
        let meta = json!({"attributes": []});
        let result = extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn extract_correct_attribute_from_meta_test() {
        let meta = json!({"attributes": [{"trait_type": "eyes", "value": "blue"}]});
        let result = extract_attributes_from_meta(&meta);
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
        let result = extract_attributes_from_meta(&meta);
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
        let result = extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }

    #[test]
    pub fn dont_extract_incomplete_attribute_from_meta_test() {
        let meta = json!({"attributes": [{"trait_type": "eyes"}]});
        let result = extract_attributes_from_meta(&meta);
        assert_eq!(result.len(), 0);
    }
}
