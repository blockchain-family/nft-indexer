use crate::types::decoded::DirectSell;
use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;

pub async fn save_direct_sell(
    tx: &mut Transaction<'_, Postgres>,
    dss: &[DirectSell],
) -> Result<()> {
    let addresses = dss.iter().map(|ds| ds.address.as_str()).collect::<Vec<_>>();
    let roots = dss.iter().map(|ds| ds.root.as_str()).collect::<Vec<_>>();
    let nfts = dss.iter().map(|ds| ds.nft.as_str()).collect::<Vec<_>>();
    let collections = dss
        .iter()
        .map(|ds| ds.collection.as_deref())
        .collect::<Vec<_>>();
    let price_tokens = dss
        .iter()
        .map(|ds| ds.price_token.as_str())
        .collect::<Vec<_>>();
    let prices = dss.iter().map(|ds| ds.price.clone()).collect::<Vec<_>>();
    let sellers = dss.iter().map(|ds| ds.seller.as_str()).collect::<Vec<_>>();
    let finished_at = dss.iter().map(|ds| ds.finished_at).collect::<Vec<_>>();
    let expired_at = dss.iter().map(|ds| ds.expired_at).collect::<Vec<_>>();
    let states = dss.iter().map(|ds| ds.state.clone()).collect::<Vec<_>>();
    let updated = dss.iter().map(|ds| ds.updated).collect::<Vec<_>>();
    let created = dss.iter().map(|ds| ds.created).collect::<Vec<_>>();
    let tx_lt = dss.iter().map(|ds| ds.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_direct_sell(
                address,
                root,
                nft, 
                collection,
                price_token, 
                price, 
                seller,
                finished_at,
                expired_at,
                state,
                created,
                updated,
                tx_lt
            )
            select
                unnest($1::varchar[]), 
                unnest($2::varchar[]),
                unnest($3::varchar[]), 
                unnest($4::varchar[]),
                unnest($5::varchar[]), 
                unnest($6::numeric[]),
                unnest($7::varchar[]),
                unnest($8::timestamp[]),
                unnest($9::timestamp[]),
                unnest($10::direct_sell_state[]),
                unnest($11::timestamp[]),
                unnest($12::timestamp[]),
                unnest($13::bigint[])
            on conflict(address) do nothing
        "#,
        addresses as _,
        roots as _,
        nfts as _,
        collections as _,
        price_tokens as _,
        prices as _,
        sellers as _,
        finished_at as _,
        expired_at as _,
        states as _,
        created as _,
        updated as _,
        tx_lt as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

pub async fn update_direct_sell_state(
    tx: &mut Transaction<'_, Postgres>,
    dss: &mut [DirectSell],
) -> Result<()> {
    dss.sort_by(|a, b| b.tx_lt.cmp(&a.tx_lt));
    let mut last_state_change = HashMap::with_capacity(dss.len());

    for ds in dss.iter() {
        if !last_state_change.contains_key(&ds.address) {
            last_state_change.insert(&ds.address, ds);
        }
    }

    let mut addresses = Vec::with_capacity(dss.len());
    let mut nfts = Vec::with_capacity(dss.len());
    let mut collections = Vec::with_capacity(dss.len());
    let mut price_tokens = Vec::with_capacity(dss.len());
    let mut prices = Vec::with_capacity(dss.len());
    let mut sellers = Vec::with_capacity(dss.len());
    let mut expired_at = Vec::with_capacity(dss.len());
    let mut finished_at = Vec::with_capacity(dss.len());
    let mut states = Vec::with_capacity(dss.len());
    let mut created = Vec::with_capacity(dss.len());
    let mut updated = Vec::with_capacity(dss.len());
    let mut tx_lts = Vec::with_capacity(dss.len());

    for ds in last_state_change.values() {
        addresses.push(ds.address.as_str());
        nfts.push(ds.nft.as_str());
        collections.push(ds.collection.as_deref());
        price_tokens.push(ds.price_token.as_str());
        prices.push(ds.price.clone());
        sellers.push(ds.seller.as_str());
        expired_at.push(ds.expired_at);
        finished_at.push(ds.finished_at);
        states.push(ds.state.clone());
        updated.push(ds.updated);
        created.push(ds.created);
        tx_lts.push(ds.tx_lt);
    }

    sqlx::query!(
        r#"
        update nft_direct_sell set
            state = data.state,
            nft = data.nft,
            collection = data.collection,
            price_token = data.price_token,
            price = data.price,
            seller = data.seller,
            expired_at = data.expired_at,
            finished_at = data.finished_at,
            updated = data.updated,
            created = data.created,
            tx_lt = data.tx_lt
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::direct_sell_state[]) as state,
                unnest($3::timestamp[]) as finished_at,
                unnest($4::timestamp[]) as updated,
                unnest($5::bigint[]) as tx_lt,
                unnest($6::varchar[]) as nft,
                unnest($7::varchar[]) as collection,
                unnest($8::varchar[]) as price_token,
                unnest($9::numeric[]) as price,
                unnest($10::varchar[]) as seller,
                unnest($11::timestamp[]) as expired_at,
                unnest($12::timestamp[]) as created
        ) as data
        where nft_direct_sell.address = data.address
        "#,
        addresses as _,
        states as _,
        finished_at as _,
        updated as _,
        tx_lts as _,
        nfts as _,
        collections as _,
        price_tokens as _,
        prices as _,
        sellers as _,
        expired_at as _,
        created as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
