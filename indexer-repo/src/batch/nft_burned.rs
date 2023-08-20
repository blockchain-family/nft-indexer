use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::decoded::NftBurned;

pub async fn save_nft_burned(pool: &PgPool, nft_burned: Vec<NftBurned>) -> Result<()> {
    let addresses = nft_burned
        .iter()
        .map(|n| n.address.as_str())
        .collect::<Vec<_>>();
    let owners = nft_burned
        .iter()
        .map(|n| n.owner.as_str())
        .collect::<Vec<_>>();
    let managers = nft_burned
        .iter()
        .map(|n| n.manager.as_str())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft set
            burned = true,
            owner = data.owner,
            manager = data.manager
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::varchar[]) as owner,
                unnest($3::varchar[]) as manager
        ) as data
        where nft.address = data.address
    "#,
        addresses as _,
        owners as _,
        managers as _
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
