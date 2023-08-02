use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Transaction};

use crate::types::NftCollection;

#[derive(Clone)]
pub struct MetadataModelService {
    pool: PgPool,
}

const FAILED_META_COOLDOWN_SECS: i64 = 30 * 60;

pub struct NftAddressData {
    pub nft: String,
    pub collection: Option<String>,
}

pub struct NftMeta<'a> {
    pub address: &'a str,
    pub meta: &'a serde_json::Value,
    pub updated: NaiveDateTime,
}

pub struct NftMetaAttribute<'a> {
    pub nft: &'a str,
    pub collection: Option<&'a str>,
    pub raw: &'a serde_json::Value,
    pub trait_type: &'a str,
    pub value: Option<&'a serde_json::Value>,
}

impl<'a> NftMetaAttribute<'a> {
    pub fn new(
        raw: &'a serde_json::Value,
        address_data: &'a NftAddressData,
    ) -> NftMetaAttribute<'a> {
        let trait_type = raw
            .get("trait_type")
            .and_then(|e| e.as_str())
            .unwrap_or_default();

        let value = raw.get("display_value").or_else(|| raw.get("value"));

        Self {
            nft: &address_data.nft,
            collection: address_data.collection.as_deref(),
            raw,
            trait_type,
            value,
        }
    }
}

impl MetadataModelService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_nfts_by_collection(&self, collection: &str) -> Result<Vec<String>> {
        #[derive(Default)]
        struct NftRecord {
            pub address: String,
        }

        let nfts: Vec<NftRecord> = sqlx::query_as!(
            NftRecord,
            r#"
            select address 
            from nft 
            where collection = $1
            "#,
            collection,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(nfts.into_iter().map(|it| it.address).collect())
    }

    pub async fn get_nfts_for_meta_update(
        &self,
        items_per_page: i64,
    ) -> Result<Vec<NftAddressData>> {
        struct Row {
            address: String,
            collection: Option<String>,
        }

        impl From<Row> for NftAddressData {
            fn from(value: Row) -> Self {
                Self {
                    nft: value.address,
                    collection: value.collection,
                }
            }
        }

        sqlx::query_as!(
            Row,
            r#"
                select 
                    n.address,
                    n.collection
                from nft n
                left join meta_handled_addresses mha on mha.address = n.address
                where 
                    (mha.address is null) or
                    (extract(epoch from now()) - mha.updated_at > $2 and failed is true)
                limit $1
            "#,
            items_per_page,
            FAILED_META_COOLDOWN_SECS as _,
        )
        .fetch_all(&self.pool)
        .await
        .map(|d| d.into_iter().map(|r| r.into()).collect::<Vec<_>>())
        .map_err(|e| anyhow!(e))
    }

    pub async fn get_collections_for_meta_update(
        &self,
        items_per_page: i64,
    ) -> Result<Vec<String>> {
        sqlx::query_scalar!(
            r#"
                select c.address
                from nft_collection c
                left join meta_handled_addresses mha on mha.address = c.address
                where 
                    (mha.address is null) or
                    (extract(epoch from now()) - mha.updated_at > $2 and failed is true)
                limit $1
                "#,
            items_per_page,
            FAILED_META_COOLDOWN_SECS as _,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
    }

    pub async fn start_transaction(&self) -> Result<MetadataModelTransaction> {
        let tx = self.pool.begin().await?;

        Ok(MetadataModelTransaction { tx })
    }
}

pub struct MetadataModelTransaction<'a> {
    tx: Transaction<'a, Postgres>,
}

impl<'a> MetadataModelTransaction<'a> {
    pub async fn update_name_desc(&mut self, name: &str, desc: &str, addr: &str) -> Result<()> {
        sqlx::query!(
            r#"
                update nft
                set name = $1,
                    description = $2
                where address = $3
            "#,
            name,
            desc,
            addr
        )
        .execute(&mut self.tx)
        .await
        .map(|_| ())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_name(&mut self, name: &str, addr: &str) -> Result<()> {
        sqlx::query!(
            r#"
                update nft
                set name = $1
                where address = $2
            "#,
            name,
            addr
        )
        .execute(&mut self.tx)
        .await
        .map(|_| ())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_desc(&mut self, desc: &str, addr: &str) -> Result<()> {
        sqlx::query!(
            r#"
                update nft
                set description = $1
                where address = $2
            "#,
            desc,
            addr
        )
        .execute(&mut self.tx)
        .await
        .map(|_| ())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_nft_attributes(&mut self, attr: &[NftMetaAttribute<'a>]) -> Result<()> {
        for nft_attribute in attr {
            sqlx::query!(
                r#"
                    insert into nft_attributes (nft, collection, raw, trait_type, value)
                    values ($1, $2, $3, $4, $5)
                "#,
                &nft_attribute.nft as _,
                &nft_attribute.collection as _,
                nft_attribute.raw,
                nft_attribute.trait_type,
                nft_attribute.value,
            )
            .execute(&mut self.tx)
            .await
            .map_err(|e| anyhow!(e))?;
        }

        Ok(())
    }
    pub async fn update_nft_meta(&mut self, meta: &NftMeta<'a>) -> Result<()> {
        sqlx::query!(
            r#"
                insert into nft_metadata (nft, meta, updated)
                values ($1, $2, $3)
                on conflict (nft) where updated < $3 do update
                set meta = coalesce($2, nft_metadata.meta), updated = $3
            "#,
            &meta.address as _,
            &meta.meta,
            &meta.updated
        )
        .execute(&mut self.tx)
        .await
        .map(|_| ())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_collection(&mut self, collection: &NftCollection) -> Result<()> {
        crate::actions::upsert_collection(collection, &mut self.tx, None)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn add_to_proceeded(&mut self, addr: &str, failed: Option<bool>) -> Result<()> {
        let failed = failed.unwrap_or(false);

        let now = chrono::Utc::now().naive_utc().timestamp();

        sqlx::query!(
            r#"
                insert into meta_handled_addresses (
                    address, 
                    updated_at,
                    failed
                )
                values (
                    $1, 
                    $2,
                    $3
                )
                on conflict (address) do update 
                set
                    updated_at = $2,
                    failed = $3
            "#,
            addr as _,
            now as _,
            failed,
        )
        .execute(&mut self.tx)
        .await
        .map(|_| ())
        .map_err(|e| anyhow!(e))
    }
    pub async fn commit(self) -> Result<()> {
        self.tx.commit().await.map_err(|e| anyhow!(e))
    }
}
