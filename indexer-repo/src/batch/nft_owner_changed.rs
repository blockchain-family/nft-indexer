use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::types::decoded::AddressChanged;

pub async fn save_nft_owner_changed(pool: &PgPool, mut data: Vec<AddressChanged>) -> Result<()> {
    data.sort_by(|a, b| b.logical_time.cmp(&a.logical_time));
    let mut last_addresses = HashMap::with_capacity(data.len());

    for nft in data.iter() {
        if !last_addresses.contains_key(&nft.id_address) {
            last_addresses.insert(&nft.id_address, nft);
        }
    }

    let mut addresses = Vec::with_capacity(last_addresses.keys().len());
    let mut new_owners = Vec::with_capacity(last_addresses.keys().len());
    let mut timestamps = Vec::with_capacity(last_addresses.keys().len());

    for val in last_addresses.values() {
        addresses.push(val.id_address.as_str());
        new_owners.push(val.new_address.as_str());
        timestamps.push(NaiveDateTime::from_timestamp_opt(val.timestamp, 0).unwrap_or_default());
    }

    sqlx::query!(
        r#"
        update nft set
            owner = data.owner,
            updated = data.time
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::varchar[]) as owner,
                unnest($3::timestamp[]) as time
        ) as data
        where nft.address = data.address
    "#,
        addresses as _,
        new_owners as _,
        timestamps as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
