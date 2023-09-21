use crate::types::NftCollection;
use anyhow::{anyhow, Result};
use sqlx::PgPool;

pub async fn save_collections(pool: &PgPool, collections: &[NftCollection]) -> Result<()> {
    let addresses = collections
        .iter()
        .map(|c| c.address.as_str())
        .collect::<Vec<_>>();
    let mint_ts = collections
        .iter()
        .map(|c| c.nft_first_mint)
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_collection (
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
