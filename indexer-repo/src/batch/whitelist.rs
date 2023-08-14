use anyhow::{anyhow, Result};
use sqlx::PgPool;

pub async fn save_whitelist_address(pool: &PgPool, data: Vec<String>) -> Result<()> {
    sqlx::query!(
        r#"
            insert into events_whitelist(address)
            select * from unnest($1::varchar[])
            on conflict(address) do nothing
        "#,
        &data[..],
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
