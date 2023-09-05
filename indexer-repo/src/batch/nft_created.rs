use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::decoded::NftCreated;

pub async fn save_nft_created(pool: &PgPool, nft_created: &[NftCreated]) -> Result<()> {
    let ids = nft_created.iter().map(|n| n.id.clone()).collect::<Vec<_>>();
    let addresses = nft_created
        .iter()
        .map(|n| n.address.as_str())
        .collect::<Vec<_>>();
    let collections = nft_created
        .iter()
        .map(|n| n.collection.as_str())
        .collect::<Vec<_>>();
    let owners = nft_created
        .iter()
        .map(|n| n.owner.as_str())
        .collect::<Vec<_>>();
    let managers = nft_created
        .iter()
        .map(|n| n.manager.as_str())
        .collect::<Vec<_>>();
    let updated = nft_created.iter().map(|n| n.updated).collect::<Vec<_>>();
    let owner_update_lt = nft_created
        .iter()
        .map(|n| n.owner_update_lt as i64)
        .collect::<Vec<_>>();
    let manager_update_lt = nft_created
        .iter()
        .map(|n| n.manager_update_lt as i64)
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft (
                id,
                address, 
                collection, 
                owner, 
                manager, 
                updated, 
                owner_update_lt, 
                manager_update_lt
            )
            select
                unnest($1::numeric[]),
                unnest($2::varchar[]),
                unnest($3::varchar[]), 
                unnest($4::varchar[]), 
                unnest($5::varchar[]), 
                unnest($6::timestamp[]),
                unnest($7::bigint[]),
                unnest($8::bigint[]) 
            on conflict(address) do nothing
        "#,
        ids as _,
        addresses as _,
        collections as _,
        owners as _,
        managers as _,
        updated as _,
        owner_update_lt as _,
        manager_update_lt as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
