use crate::indexer::events::fetch_metadata;
use chrono::Utc;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use storage::actions::{
    get_nfts_by_collection, upsert_nft_attributes, upsert_nft_meta, upsert_nft_meta_columns,
};
use storage::types::{Address, NftAttribute, NftMeta};
use ton_block::MsgAddressInt;
use transaction_consumer::TransactionConsumer;

pub struct MetadataService {
    pub consumer: Arc<TransactionConsumer>,
    pub pool: PgPool,
}

impl MetadataService {
    pub async fn refresh_metadata(
        &self,
        nft_address: &str,
        collection: &str,
    ) -> anyhow::Result<()> {
        let meta = fetch_metadata(MsgAddressInt::from_str(nft_address)?, &self.consumer).await;

        let nft = Address::from_str(nft_address)?;
        let collection = Address::from_str(collection)?;

        let mut tx = self.pool.begin().await?;

        if let Some(attributes) = meta.get("attributes").and_then(|v| v.as_array()) {
            let nft_attributes: Vec<NftAttribute> = attributes
                .iter()
                .map(|item| NftAttribute::new(nft.clone(), Some(collection.clone()), item.clone()))
                .collect();

            upsert_nft_attributes(&nft_attributes, &mut tx).await?;
        }

        let nft_meta = NftMeta {
            nft: nft.clone(),
            meta: meta.clone(),
            updated: Utc::now().naive_utc(),
        };

        let name = meta
            .get("name")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .unwrap_or_default()
            .to_string();
        let description = meta
            .get("description")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .unwrap_or_default()
            .to_string();

        let dt = Utc::now().naive_utc();
        upsert_nft_meta_columns(nft_address, &name, &description, dt, &mut tx).await?;
        upsert_nft_meta(&nft_meta, &mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn refresh_metadata_by_collection(&self, collection: &str) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let nfts = get_nfts_by_collection(collection, &mut tx).await?;
        for nft in nfts {
            self.refresh_metadata(&nft, collection).await?;
        }
        Ok(())
    }
}
