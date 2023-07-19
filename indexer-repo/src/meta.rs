use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Transaction};

pub struct NftMetadataModelService {
    pool: PgPool,
}

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
    pub fn new(raw: &'a serde_json::Value, addresses: &'a NftAddressData) -> NftMetaAttribute<'a> {
        let trait_type = raw
            .get("trait_type")
            .and_then(|e| e.as_str())
            .unwrap_or_default();

        let value = raw.get("display_value").or_else(|| raw.get("value"));

        Self {
            nft: &addresses.nft,
            collection: addresses.collection.as_deref(),
            raw,
            trait_type,
            value,
        }
    }
}

impl NftMetadataModelService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_nft_addresses_without_meta(
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
            left join handled_nft hn on hn.address = n.address
            where hn.address is null
            limit $1
        "#,
            items_per_page,
        )
        .fetch_all(&self.pool)
        .await
        .map(|d| d.into_iter().map(|r| r.into()).collect::<Vec<_>>())
        .map_err(|e| anyhow!(e))
    }

    pub async fn start_transaction(&self) -> Result<NftMetadataModelTransaction> {
        let tx = self.pool.begin().await?;

        Ok(NftMetadataModelTransaction { tx })
    }
}

pub struct NftMetadataModelTransaction<'a> {
    tx: Transaction<'a, Postgres>,
}

impl<'a> NftMetadataModelTransaction<'a> {
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

    pub async fn add_to_proceeded(&mut self, addr: &str) -> Result<()> {
        sqlx::query!(
            r#"
                insert into handled_nft (address)
                values ($1)
                on conflict do nothing
            "#,
            addr as _
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
