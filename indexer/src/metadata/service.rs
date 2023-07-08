use chrono::Utc;
use indexer_repo::actions::{
    get_nfts_by_collection, upsert_collection, upsert_nft_attributes, upsert_nft_info,
    upsert_nft_meta, upsert_nft_meta_columns,
};
use indexer_repo::types::{Address, NftAttribute, NftCollection, NftMeta};
use rpc::get_info;
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

pub struct MetadataService {
    pub jrpc_client: JrpcClient,
    pub pool: PgPool,
}

impl MetadataService {
    pub async fn refresh_nft_metadata(
        &self,
        nft_address: &str,
        collection: &str,
    ) -> anyhow::Result<()> {
        let nft = Address::from_str(nft_address)?;
        let nft_address_int = MsgAddressInt::from_str(nft_address)?;
        let meta = fetch_metadata(MsgAddressInt::from_str(nft_address)?, &self.jrpc_client).await;

        let nft_info = get_info(nft_address_int, self.jrpc_client.clone()).await?;

        let collection = Address::from_str(collection)?;

        let mut tx = self.pool.begin().await?;
        let owner = Address::from_str(&format!("0:{}", nft_info.owner.address().as_hex_string()))?;
        let manager =
            Address::from_str(&format!("0:{}", nft_info.manager.address().as_hex_string()))?;
        upsert_nft_info(&owner, &manager, &nft, &mut tx).await?;

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

        self.refresh_collection_metadata(collection, &mut tx)
            .await?;

        let nfts = get_nfts_by_collection(collection, &mut tx).await?;
        for nft in nfts {
            self.refresh_nft_metadata(&nft, collection).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn refresh_collection_metadata(
        &self,
        collection: &str,
        pg_pool_tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        let collection =
            get_collection_data(MsgAddressInt::from_str(collection)?, &self.jrpc_client).await;

        upsert_collection(&collection, pg_pool_tx, None).await?;

        Ok(())
    }
}

pub async fn get_collection_data(
    collection: MsgAddressInt,
    jrpc_client: &JrpcClient,
) -> NftCollection {
    let collection_owner = get_collection_owner(collection.clone(), jrpc_client).await;

    let collection_meta = fetch_metadata(collection.clone(), jrpc_client).await;
    let now = chrono::Utc::now().naive_utc();

    NftCollection {
        address: ("0:".to_owned() + &collection.address().as_hex_string()).into(),
        owner: collection_owner,
        name: collection_meta
            .get("name")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(str::to_string),
        description: collection_meta
            .get("description")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(str::to_string),
        created: now,
        updated: now,
        logo: collection_meta
            .get("preview")
            .cloned()
            .unwrap_or_default()
            .get("source")
            .cloned()
            .unwrap_or_default()
            .as_str()
            .map(|s| s.into()),
        wallpaper: collection_meta
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
    }
}

pub async fn fetch_metadata(address: MsgAddressInt, jrpc_client: &JrpcClient) -> serde_json::Value {
    match rpc::retrier::Retrier::new(|| {
        Box::pin(rpc::get_json(address.clone(), jrpc_client.clone()))
    })
    .attempts(1)
    .trace_id(format!(
        "fetch metadata {}",
        address.address().as_hex_string()
    ))
    .run()
    .await
    {
        Ok(meta) => meta,

        Err(e) => {
            log::error!("Error fetching metadata for {address}: {e:#?}");
            serde_json::Value::default()
        }
    }
}

async fn get_collection_owner(
    collection: MsgAddressInt,
    jrpc_client: &JrpcClient,
) -> indexer_repo::types::Address {
    match rpc::retrier::Retrier::new(|| {
        Box::pin(rpc::owner(collection.clone(), jrpc_client.clone()))
    })
    .attempts(1)
    .trace_id(format!(
        "collection owner {}",
        collection.address().as_hex_string()
    ))
    .run()
    .await
    {
        Ok(owner) => owner.into(),
        Err(e) => {
            log::error!("Can't get {} collection owner: {:#?}", collection, e);
            String::default().into()
        }
    }
}
