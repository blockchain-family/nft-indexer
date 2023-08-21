use crate::types::*;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};

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
            updated      = greatest($7, nft_collection.updated)
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

pub async fn get_collections(limit: i64, pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        select address
        from nft_collection
        order by updated desc
        limit $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await
}
