use std::collections::HashMap;

use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::decoded::AddressChanged;

pub async fn save_nft_manager_changed(
    tx: &mut Transaction<'_, Postgres>,
    data: &mut [AddressChanged],
) -> Result<()> {
    data.sort_by(|a, b| b.logical_time.cmp(&a.logical_time));
    let mut last_addresses = HashMap::with_capacity(data.len());

    for nft in data.iter() {
        if !last_addresses.contains_key(&nft.id_address) {
            last_addresses.insert(&nft.id_address, nft);
        }
    }

    let mut addresses = Vec::with_capacity(last_addresses.keys().len());
    let mut new_managers = Vec::with_capacity(last_addresses.keys().len());
    let mut timestamps = Vec::with_capacity(last_addresses.keys().len());

    for val in last_addresses.values() {
        addresses.push(val.id_address.as_str());
        new_managers.push(val.new_address.as_str());
        timestamps.push(val.timestamp);
    }

    sqlx::query!(
        r#"
        update nft set
            manager = data.manager,
            updated = data.time
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::varchar[]) as manager,
                unnest($3::timestamp[]) as time
        ) as data
        where nft.address = data.address
    "#,
        addresses as _,
        new_managers as _,
        timestamps as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
