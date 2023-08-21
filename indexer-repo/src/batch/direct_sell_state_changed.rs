use crate::types::{decoded::DirectSell, DirectSellState};
use anyhow::{anyhow, Result};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn save_direct_sell_state_changed(pool: &PgPool, dss: &[DirectSell]) -> Result<()> {
    let mut to_insert = Vec::with_capacity(dss.len());
    let mut for_update = Vec::with_capacity(dss.len());

    for db in dss {
        if db.state == DirectSellState::Create {
            to_insert.push(db);
        } else {
            for_update.push(db);
        }
    }

    if !to_insert.is_empty() {
        insert_direct_sell(pool, &to_insert).await?;
    }

    if !for_update.is_empty() {
        update_direct_sell_state(pool, &mut for_update).await?;
    }

    Ok(())
}

async fn insert_direct_sell(pool: &PgPool, dss: &[&DirectSell]) -> Result<()> {
    let addresses = dss.iter().map(|ds| ds.address.as_str()).collect::<Vec<_>>();
    let roots = dss.iter().map(|ds| ds.root.as_str()).collect::<Vec<_>>();
    let nfts = dss.iter().map(|ds| ds.nft.as_str()).collect::<Vec<_>>();
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
            select * from unnest(
                $1::varchar[], 
                $2::varchar[],
                $3::varchar[], 
                $4::varchar[], 
                $5::numeric[],
                $6::varchar[],
                $7::timestamp[],
                $8::timestamp[],
                $9::direct_sell_state[],
                $10::timestamp[],
                $11::timestamp[],
                $12::bigint[]
            ) 
            on conflict(address) do nothing
        "#,
        addresses as Vec<_>,
        roots as Vec<_>,
        nfts as Vec<_>,
        price_tokens as Vec<_>,
        prices as Vec<_>,
        sellers as Vec<_>,
        finished_at as Vec<_>,
        expired_at as Vec<_>,
        states as Vec<_>,
        created as Vec<_>,
        updated as Vec<_>,
        tx_lt as Vec<_>,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

async fn update_direct_sell_state(pool: &PgPool, dss: &mut [&DirectSell]) -> Result<()> {
    dss.sort_by(|a, b| b.tx_lt.cmp(&a.tx_lt));
    let mut last_state_change = HashMap::with_capacity(dss.len());

    for ds in dss.iter() {
        if !last_state_change.contains_key(&ds.address) {
            last_state_change.insert(&ds.address, ds);
        }
    }

    let mut addresses = Vec::with_capacity(dss.len());
    let mut finished_at = Vec::with_capacity(dss.len());
    let mut states = Vec::with_capacity(dss.len());
    let mut updated = Vec::with_capacity(dss.len());
    let mut tx_lts = Vec::with_capacity(dss.len());

    for ds in last_state_change.values() {
        addresses.push(ds.address.as_str());
        finished_at.push(ds.finished_at);
        states.push(ds.state.clone());
        updated.push(ds.updated);
        tx_lts.push(ds.tx_lt);
    }

    sqlx::query!(
        r#"
        update nft_direct_sell set
            state = data.state,
            finished_at = data.finished_at,
            updated = data.updated,
            tx_lt = data.tx_lt
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::direct_sell_state[]) as state,
                unnest($3::timestamp[]) as finished_at,
                unnest($4::timestamp[]) as updated,
                unnest($5::bigint[]) as tx_lt
        ) as data
        where nft_direct_sell.address = data.address
        "#,
        addresses as _,
        states as _,
        finished_at as _,
        updated as _,
        tx_lts as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
