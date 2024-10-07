use anyhow::Result;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

pub async fn safe_refresh_nft_verified_extended(
    pool: &PgPool,
    full_update: bool,
) -> Result<PgQueryResult> {
    Ok(
        sqlx::query!("select safe_refresh_nft_verified_extended($1)", full_update)
            .execute(pool)
            .await?,
    )
}

pub async fn refresh_nft_events_verified_materialized_view(pool: &PgPool) -> Result<PgQueryResult> {
    Ok(
        sqlx::query!("refresh materialized view concurrently nft_events_verified_mv")
            .execute(pool)
            .await?,
    )
}

pub async fn refresh_nft_types_materialized_view(pool: &PgPool) -> Result<PgQueryResult> {
    Ok(
        sqlx::query!("refresh materialized view concurrently nft_type_mv")
            .execute(pool)
            .await?,
    )
}

pub async fn refresh_collection_types_materialized_view(pool: &PgPool) -> Result<PgQueryResult> {
    Ok(
        sqlx::query!("refresh materialized view concurrently collection_type_mv")
            .execute(pool)
            .await?,
    )
}

pub async fn update_latest_collections(pool: &PgPool) -> Result<PgQueryResult> {
    Ok(
        sqlx::query!("call update_latest_collections(interval '30 minutes')")
            .execute(pool)
            .await?,
    )
}

pub async fn update_all_collections(pool: &PgPool) -> Result<PgQueryResult> {
    Ok(sqlx::query!("call update_all_collections()")
        .execute(pool)
        .await?)
}
