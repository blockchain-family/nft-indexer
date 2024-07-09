use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};

use crate::types::NftCollectionMeta;

#[derive(Clone)]
pub struct MetadataModelService {
    pool: PgPool,
}

const FAILED_META_COOLDOWN_SECS: i64 = 120 * 60;

pub struct NftAddressData {
    pub nft: String,
    pub collection: String,
}

pub struct NftMeta<'a> {
    pub address: &'a str,
    pub meta: &'a serde_json::Value,
    pub updated: NaiveDateTime,
}

pub struct NftMetaAttribute<'a> {
    pub raw: &'a serde_json::Value,
    pub trait_type: &'a str,
    pub value: Option<&'a serde_json::Value>,
}

impl<'a> NftMetaAttribute<'a> {
    pub fn new(raw: &'a serde_json::Value) -> NftMetaAttribute<'a> {
        let trait_type = raw
            .get("trait_type")
            .and_then(|e| e.as_str())
            .unwrap_or_default();

        let value = raw.get("display_value").or_else(|| raw.get("value"));

        Self {
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
        Ok(sqlx::query_scalar!(
            r#"
            select address
            from nft
            where collection = $1
            "#,
            collection,
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_nfts_for_meta_update(
        &self,
        items_per_page: i64,
    ) -> Result<Vec<NftAddressData>> {
        Ok(sqlx::query_as!(
                    NftAddressData,
                    r#"
                    select n.address as nft, n.collection
                    from nft n
                             join nft_collection nc on nc.address = n.collection
                             left join meta_handled_addresses mha on mha.address = n.address
                    where (n.metadata_updated_at is null and mha.address is null)
                       or (mha.updated_at < (extract(epoch from now()) - $2)::bigint and failed and n.metadata_updated_at is null)
                    order by nc.verified desc
                    limit $1
                    "#,
                    items_per_page,
                    FAILED_META_COOLDOWN_SECS as _,
                )
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn get_collections_for_meta_update(
        &self,
        items_per_page: i64,
    ) -> Result<Vec<String>> {
        Ok(sqlx::query_scalar!(
                    r#"
                        select c.address
                        from nft_collection c
                        left join meta_handled_addresses mha on mha.address = c.address
                        where
                            /*c.verified and*/
                            (mha.address is null)
                            or (mha.updated_at < (extract(epoch from now()) - $2)::bigint and failed)
                        order by updated desc
                        limit $1
                        "#,
                    items_per_page,
                    FAILED_META_COOLDOWN_SECS as _,
                )
            .fetch_all(&self.pool)
            .await?)
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
    pub async fn update_nft_basic_meta(
        &mut self,
        addr: &str,
        name: Option<&str>,
        description: Option<&str>,
        updated: NaiveDateTime,
    ) -> Result<PgQueryResult> {
        Ok(sqlx::query!(
            r#"
                update nft
                set name = $2,
                    description = $3,
                    metadata_updated_at = $4
                where address = $1
            "#,
            addr,
            name,
            description,
            updated.and_utc().timestamp()
        )
        .execute(&mut self.tx)
        .await?)
    }

    pub async fn update_nft_attributes(
        &mut self,
        address_data: &NftAddressData,
        attr: &[NftMetaAttribute<'a>],
        updated: NaiveDateTime,
    ) -> Result<()> {
        for nft_attribute in attr {
            sqlx::query!(
                r#"
                    insert into nft_attributes (nft, collection, raw, trait_type, value, updated)
                    values ($1, $2, $3, $4, $5, $6)
                    on conflict (nft, trait_type) where updated < $6 do update
                    set raw = excluded.raw, value = excluded.value, updated = excluded.updated;
                "#,
                &address_data.nft as _,
                &address_data.collection as _,
                nft_attribute.raw,
                nft_attribute.trait_type,
                nft_attribute.value,
                updated,
            )
            .execute(&mut self.tx)
            .await?;
        }

        let nft_trait_types = attr
            .iter()
            .map(|a| a.trait_type.to_string())
            .collect::<Vec<_>>();
        sqlx::query!(
            r#"delete from nft_attributes where nft = $1 and trait_type != all($2);"#,
            &address_data.nft,
            &nft_trait_types[..]
        )
        .execute(&mut self.tx)
        .await?;

        Ok(())
    }

    pub async fn update_nft_meta(&mut self, meta: &NftMeta<'a>) -> Result<PgQueryResult> {
        Ok(sqlx::query!(
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
        .await?)
    }

    pub async fn update_collection(&mut self, meta: &NftCollectionMeta) -> Result<PgQueryResult> {
        Ok(sqlx::query!(
            r#"
            update nft_collection
            set
                name         = coalesce($2, nft_collection.name),
                description  = coalesce($3, nft_collection.description),
                logo         = coalesce($4, nft_collection.logo),
                wallpaper    = coalesce($5, nft_collection.wallpaper),
                updated      = greatest($6, nft_collection.updated),
                owner        = coalesce($7, nft_collection.owner),
                royalty      = coalesce($8, nft_collection.royalty)
            where address = $1
            "#,
            meta.address as _,
            meta.name,
            meta.description,
            meta.logo as _,
            meta.wallpaper as _,
            meta.updated,
            meta.owner as _,
            meta.royalty as _,
        )
        .execute(&mut self.tx)
        .await?)
    }

    pub async fn add_to_proceeded(
        &mut self,
        addr: &str,
        failed: Option<bool>,
    ) -> Result<PgQueryResult> {
        let failed = failed.unwrap_or(false);

        let now = chrono::Utc::now().timestamp();

        Ok(sqlx::query!(
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
        .await?)
    }

    pub async fn commit(self) -> Result<()> {
        Ok(self.tx.commit().await?)
    }
}
