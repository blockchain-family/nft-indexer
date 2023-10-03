use std::{str::FromStr, time::Duration};

use crate::service::MetadataJrpcService;
use anyhow::{bail, Result};
use indexer_repo::{
    meta::{MetadataModelService, NftAddressData, NftMeta, NftMetaAttribute},
    types::NftCollectionMeta,
};
use serde_json::Value;
use sqlx::{types::chrono, PgPool};
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

const NFT_PER_ITERATION: i64 = 30_000;
const COLLECTION_PER_ITERATION: i64 = 100;

#[derive(Clone)]
pub struct MetaReaderContext {
    pub jrpc_client: JrpcClient,
    pub pool: PgPool,
    pub jrpc_req_latency_millis: u64,
    pub idle_after_loop: u64,
}

pub async fn run_meta_reader(context: MetaReaderContext) -> Result<()> {
    log::info!("Run metadata reader");
    let meta_jrpc_service = MetadataJrpcService::new(context.jrpc_client.clone());
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
                    tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis)).await;
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
                    tokio::time::sleep(Duration::from_millis(context.jrpc_req_latency_millis)).await;
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
    meta_jrpc_service: &MetadataJrpcService,
) -> Result<()> {
    let Ok(collection_address) = MsgAddressInt::from_str(address) else {
        bail!("Error while converting collection address {} to MsgAddressInt", address);
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
    meta_jrpc_service: &MetadataJrpcService,
) -> Result<()> {
    let Ok(nft_address) = MsgAddressInt::from_str(&address_data.nft) else {
                bail!("Error while converting nft address {} to MsgAddressInt", address_data.nft);
            };


    let mut failed = false;

    let meta = match meta_jrpc_service
        .get_nft_meta(&nft_address)
        .await
    {
        Ok(meta) => meta,
        Err(e) => {
            log::error!("Error while reading ${} nft meta: {:#?}", address_data.nft, e);
            failed = true;
            Value::default()
        }
    };

    let Ok(mut tx) = meta_model_service.start_transaction().await else {
                bail!("Cant start transaction for saving metadata");
            };

    if let Err(e) = match (
        extract_name_from_meta(&meta),
        extract_description_from_meta(&meta),
    ) {
        (Some(name), Some(desc)) => tx.update_name_desc(name, desc, &address_data.nft).await,
        (None, Some(desc)) => tx.update_desc(desc, &address_data.nft).await,
        (Some(name), None) => tx.update_name(name, &address_data.nft).await,
        (None, None) => Ok(()),
    } {
        bail!(
            "Nft address: {}, error while updating name and/or description: {:#?}",
            address_data.nft,
            e
        );
    };

    let updated = chrono::Utc::now().naive_utc();

    let attr = meta
        .get("attributes")
        .and_then(|d| d.as_array())
        .and_then(|d| (!d.is_empty()).then_some(d))
        .map(|d| {
            d.iter()
                .map(|e| NftMetaAttribute::new(e, address_data, updated))
                .collect::<Vec<_>>()
        });
    if !failed {
        if let Some(attr) = attr {
            if let Err(e) = tx.update_nft_attributes(&attr).await {
                bail!(
                    "Nft address: {}, error while updating attributes: {:#?}",
                    &address_data.nft,
                    e
                );
            }
        }
    }

    let nft_meta = NftMeta {
        address: &address_data.nft,
        meta: &meta,
        updated,
    };

    if !failed {
        if let Err(e) = tx.update_nft_meta(&nft_meta).await {
            bail!(
                "Nft address: {}, error while updating nft meta: {:#?}",
                &address_data.nft,
                e
            );
        };
    }

    if let Err(e) = tx.add_to_proceeded(&address_data.nft, Some(failed)).await {
        bail!(
            "Nft address: {}, error while adding to meta_handled_addresses table: {:#?}",
            &address_data.nft,
            e
        );
    };

    if let Err(e) = tx.commit().await {
        bail!(
            "Nft address: {}, error while commiting transaction: {:#?}",
            &address_data.nft,
            e
        );
    };

    Ok(())
}

fn extract_name_from_meta(meta: &Value) -> Option<&str> {
    meta.get("name").and_then(|d| d.as_str())
}

fn extract_description_from_meta(meta: &Value) -> Option<&str> {
    meta.get("description").and_then(|d| d.as_str())
}
