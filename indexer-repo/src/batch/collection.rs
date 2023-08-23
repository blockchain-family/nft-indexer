use crate::types::NftCollection;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn save_collections(pool: &PgPool, collections: &[NftCollection]) -> Result<()> {
    let addresses = collections
        .iter()
        .map(|c| c.address.as_str())
        .collect::<Vec<_>>();
    let mint_ts = collections
        .iter()
        .map(|c| NaiveDateTime::from_timestamp_opt(c.nft_first_mint, 0).unwrap_or_default())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_collection(
                address, 
                first_mint, 
                created, 
                updated
            )
            select
                unnest($1::varchar[]), 
                unnest($2::timestamp[]), 
                unnest($2::timestamp[]), 
                unnest($2::timestamp[]) 
            on conflict(address) do nothing
        "#,
        addresses as _,
        mint_ts as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
