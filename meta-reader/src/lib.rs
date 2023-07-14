use std::{str::FromStr, time::Duration};

use anyhow::Result;
use indexer_repo::meta::{NftMeta, NftMetaAttribute, NftMetadataModelService};
use service::MetadataJrpcService;
use sqlx::{types::chrono, PgPool};
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

mod service;

const NFT_PER_ITERATION: i64 = 1;
const IDLE_TIME_AFTER_FINISH_SEC: u64 = 60;

pub struct MetaReaderContext {
    pub jrpc_client: JrpcClient,
    pub pool: PgPool,
}

pub async fn run_meta_reader(context: MetaReaderContext) -> Result<()> {
    log::info!("Run metadata reader");
    let meta_jrpc_service = MetadataJrpcService::new(context.jrpc_client);
    let meta_model_service = NftMetadataModelService::new(context.pool);
    let mut page = 0i64;

    loop {
        let addresses = meta_model_service
            .get_nft_addresses_without_meta(page, NFT_PER_ITERATION)
            .await?;
        if addresses.is_empty() {
            log::info!("Finished updating metadata work. Idling");
            page = 0;
            tokio::time::sleep(Duration::from_secs(IDLE_TIME_AFTER_FINISH_SEC)).await;
        }

        for address in addresses {
            let Ok(nft_address) = MsgAddressInt::from_str(&address.nft) else {
                log::error!("Error while converting nft address {} to MsgAddressInt", address.nft);
                continue;
            };

            let meta = meta_jrpc_service.fetch_metadata(nft_address).await;
            let Ok(mut tx) = meta_model_service.start_transaction().await else {
                log::error!("Cant start transaction for saving metadata");
                continue;
            };

            if let Some(name) = extract_name_from_meta(&meta) {
                if let Err(e) = tx.update_name(name, &address.nft).await {
                    log::error!(
                        "Nft address: {}, error while updating name: {:#?}",
                        address.nft,
                        e
                    );
                    continue;
                }
            }

            if let Some(desc) = extract_description_from_meta(&meta) {
                if let Err(e) = tx.update_desc(desc, &address.nft).await {
                    log::error!(
                        "Nft address: {}, error while updating description: {:#?}",
                        &address.nft,
                        e
                    );
                    continue;
                }
            }

            let attr = meta
                .get("attributes")
                .and_then(|d| d.as_array())
                .and_then(|d| {
                    if d.is_empty() {
                        None
                    } else {
                        Some(
                            d.iter()
                                .map(|e| NftMetaAttribute::new(e, &address))
                                .collect::<Vec<_>>(),
                        )
                    }
                });

            if attr.is_some() {
                let attr = attr.unwrap();
                if let Err(e) = tx.update_nft_attributes(&attr[..]).await {
                    log::error!(
                        "Nft address: {}, error while updating attributes: {:#?}",
                        &address.nft,
                        e
                    );
                    continue;
                }
            }

            let nft_meta = NftMeta {
                address: &address.nft,
                meta: &meta,
                updated: chrono::Utc::now().naive_utc(),
            };

            if let Err(e) = tx.update_nft_meta(&nft_meta).await {
                log::error!(
                    "Nft address: {}, error while updating nft meta: {:#?}",
                    &address.nft,
                    e
                );
                continue;
            };

            if let Err(e) = tx.add_to_proceeded(&address.nft).await {
                log::error!(
                    "Nft address: {}, error while adding to handled_nft table: {:#?}",
                    &address.nft,
                    e
                );
                continue;
            };

            if let Err(e) = tx.commit().await {
                log::error!(
                    "Nft address: {}, error while commiting transaction: {:#?}",
                    &address.nft,
                    e
                );
            };
        }

        page += 1;
    }
}

fn extract_name_from_meta(meta: &serde_json::Value) -> Option<&str> {
    meta.get("name").and_then(|d| d.as_str())
}

fn extract_description_from_meta(meta: &serde_json::Value) -> Option<&str> {
    meta.get("description").and_then(|d| d.as_str())
}
