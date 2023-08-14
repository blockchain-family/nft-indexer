use std::collections::HashMap;

use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::AddressChangedDecoded;

pub async fn save_nft_manager_changed(
    pool: &PgPool,
    mut data: Vec<AddressChangedDecoded>,
) -> Result<()> {
    data.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let mut last_addresses = HashMap::with_capacity(data.len());

    for nft in data.iter() {
        if !last_addresses.contains_key(&nft.id_address) {
            last_addresses.insert(&nft.id_address, nft);
        }
    }

    let mut addresses = Vec::with_capacity(last_addresses.keys().len());
    let mut new_managers = Vec::with_capacity(last_addresses.keys().len());

    for val in last_addresses.values() {
        addresses.push(val.id_address.as_str());
        new_managers.push(val.new_address.as_str());
    }

    sqlx::query!(
        r#"
        update nft set
            manager = data.manager
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::varchar[]) as manager
        ) as data
        where nft.address = data.address
    "#,
        addresses as _,
        new_managers as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
