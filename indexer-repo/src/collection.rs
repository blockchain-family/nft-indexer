use anyhow::{anyhow, Result};
use sqlx::PgPool;

pub async fn get_collections(pg_pool: &PgPool, limit: i64) -> Result<Vec<String>> {
    sqlx::query_scalar!(
        r#"
        select address
        from nft_collection
        order by updated desc
        limit $1
        "#,
        limit
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|e| anyhow!(e))
}
