use crate::types::*;
use chrono::NaiveDateTime;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};

pub async fn get_owners_count(collection: &str, tx: &mut Transaction<'_, Postgres>) -> Option<i64> {
    sqlx::query_scalar!(
        r#"
        select 
            count(distinct owner) as owners
        from 
            nft
        where 
            collection = $1
        "#,
        collection as _,
    )
    .fetch_one(tx)
    .await
    .unwrap_or_default()
}

pub async fn insert_collection(
    address: &str,
    nft_mint_ts: NaiveDateTime,
    pg_pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();

    sqlx::query!(
        r#"
        insert into nft_collection (
            address, first_mint,
            created, updated
        ) values (
            $1,       $2,
            $3,       $4
        )
        on conflict do nothing
        "#,
        address as _,
        nft_mint_ts,
        now,
        now,
    )
    .execute(pg_pool)
    .await
}

pub async fn update_collection_meta(
    meta: &NftCollectionMeta,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        update nft_collection
        set 
            owner        = $2, 
            name         = coalesce($3, nft_collection.name),
            description  = coalesce($4, nft_collection.description),
            logo         = coalesce($5, nft_collection.logo),
            wallpaper    = coalesce($6, nft_collection.wallpaper),
            updated      = $7
        where address = $1
        "#,
        meta.address as _,
        meta.owner as _,
        meta.name,
        meta.description,
        meta.logo as _,
        meta.wallpaper as _,
        meta.updated,
    )
    .execute(tx)
    .await
}

pub async fn refresh_collection_owners_count(
    address: &str,
    pg_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let mut tx = pg_pool.begin().await?;

    let owners_count = get_owners_count(address, &mut tx).await;

    sqlx::query!(
        r#"
        update nft_collection
        set owners_count = $2
        where address = $1
        "#,
        address as _,
        owners_count.unwrap_or_default() as _,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await
}

pub async fn get_collections(limit: i64, pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        select address
        from nft_collection
        limit $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await
}
