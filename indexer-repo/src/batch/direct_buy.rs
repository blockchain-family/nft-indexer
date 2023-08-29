use crate::types::decoded::DirectBuy;
use anyhow::{anyhow, Result};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn save_direct_buy(pool: &PgPool, dbs: &[DirectBuy]) -> Result<()> {
    let addresses = dbs.iter().map(|db| db.address.as_str()).collect::<Vec<_>>();
    let roots = dbs.iter().map(|db| db.root.as_str()).collect::<Vec<_>>();
    let nfts = dbs.iter().map(|db| db.nft.as_str()).collect::<Vec<_>>();
    let price_tokens = dbs
        .iter()
        .map(|db| db.price_token.as_str())
        .collect::<Vec<_>>();
    let prices = dbs.iter().map(|db| db.price.clone()).collect::<Vec<_>>();
    let buyers = dbs.iter().map(|db| db.buyer.as_str()).collect::<Vec<_>>();
    let finished_at = dbs.iter().map(|db| db.finished_at).collect::<Vec<_>>();
    let expired_at = dbs.iter().map(|db| db.expired_at).collect::<Vec<_>>();
    let states = dbs.iter().map(|db| db.state.clone()).collect::<Vec<_>>();
    let updated = dbs.iter().map(|db| db.updated).collect::<Vec<_>>();
    let created = dbs.iter().map(|db| db.created).collect::<Vec<_>>();
    let tx_lt = dbs.iter().map(|db| db.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_direct_buy(
                address,
                root,
                nft, 
                price_token, 
                price, 
                buyer,
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
                unnest($5::numeric[]),
                unnest($6::varchar[]),
                unnest($7::timestamp[]),
                unnest($8::timestamp[]),
                unnest($9::direct_buy_state[]),
                unnest($10::timestamp[]),
                unnest($11::timestamp[]),
                unnest($12::bigint[])
            on conflict(address) do nothing
        "#,
        addresses as Vec<_>,
        roots as Vec<_>,
        nfts as Vec<_>,
        price_tokens as Vec<_>,
        prices as Vec<_>,
        buyers as Vec<_>,
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

pub async fn update_direct_buy_state(pool: &PgPool, dbs: &mut [DirectBuy]) -> Result<()> {
    dbs.sort_by(|a, b| b.tx_lt.cmp(&a.tx_lt));
    let mut last_state_change = HashMap::with_capacity(dbs.len());

    for db in dbs.iter() {
        if !last_state_change.contains_key(&db.address) {
            last_state_change.insert(&db.address, db);
        }
    }

    let mut addresses = Vec::with_capacity(dbs.len());
    let mut nfts = Vec::with_capacity(dbs.len());
    let mut price_tokens = Vec::with_capacity(dbs.len());
    let mut prices = Vec::with_capacity(dbs.len());
    let mut buyers = Vec::with_capacity(dbs.len());
    let mut expired_at = Vec::with_capacity(dbs.len());
    let mut finished_at = Vec::with_capacity(dbs.len());
    let mut states = Vec::with_capacity(dbs.len());
    let mut created = Vec::with_capacity(dbs.len());
    let mut updated = Vec::with_capacity(dbs.len());
    let mut tx_lts = Vec::with_capacity(dbs.len());

    for db in last_state_change.values() {
        addresses.push(db.address.as_str());
        nfts.push(db.nft.as_str());
        price_tokens.push(db.price_token.as_str());
        prices.push(db.price.clone());
        buyers.push(db.buyer.as_str());
        expired_at.push(db.expired_at);
        finished_at.push(db.finished_at);
        states.push(db.state.clone());
        created.push(db.created);
        updated.push(db.updated);
        tx_lts.push(db.tx_lt);
    }

    sqlx::query!(
        r#"
        update nft_direct_buy set
            state = data.state,
            nft = data.nft,
            price_token = data.price_token,
            price = data.price,
            buyer = data.buyer,
            expired_at = data.expired_at,
            finished_at = data.finished_at,
            created = data.created,
            updated = data.updated,
            tx_lt = data.tx_lt
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::direct_buy_state[]) as state,
                unnest($3::timestamp[]) as finished_at,
                unnest($4::timestamp[]) as updated,
                unnest($5::bigint[]) as tx_lt,
                unnest($6::varchar[]) as nft,
                unnest($7::varchar[]) as price_token,
                unnest($8::numeric[]) as price,
                unnest($9::varchar[]) as buyer,
                unnest($10::timestamp[]) as expired_at,
                unnest($11::timestamp[]) as created
        ) as data
        where nft_direct_buy.address = data.address
        "#,
        addresses as _,
        states as _,
        finished_at as _,
        updated as _,
        tx_lts as _,
        nfts as _,
        price_tokens as _,
        prices as _,
        buyers as _,
        expired_at as _,
        created as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
