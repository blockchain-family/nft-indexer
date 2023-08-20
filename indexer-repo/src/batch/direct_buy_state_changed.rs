use crate::types::{decoded::DirectBuy, DirectBuyState};
use anyhow::{anyhow, Result};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn save_direct_buy_state_changed(pool: &PgPool, dbs: Vec<DirectBuy>) -> Result<()> {
    let mut to_insert = Vec::with_capacity(dbs.len());
    let mut for_update = Vec::with_capacity(dbs.len());

    for db in dbs {
        if db.state == DirectBuyState::Active {
            to_insert.push(db);
        } else if db.state != DirectBuyState::Create && db.state != DirectBuyState::AwaitTokens {
            for_update.push(db);
        }
    }

    if !to_insert.is_empty() {
        insert_direct_buy(pool, to_insert).await?;
    }

    if !for_update.is_empty() {
        update_direct_buy_state(pool, for_update).await?;
    }

    Ok(())
}

async fn insert_direct_buy(pool: &PgPool, dbs: Vec<DirectBuy>) -> Result<()> {
    let addresses = dbs.iter().map(|db| db.address.as_str()).collect::<Vec<_>>();
    let nfts = dbs.iter().map(|db| db.nft.as_str()).collect::<Vec<_>>();
    let collections = dbs
        .iter()
        .map(|db| db.collection.clone())
        .collect::<Vec<_>>();
    let price_tokens = dbs
        .iter()
        .map(|db| db.price_token.as_str())
        .collect::<Vec<_>>();
    let prices = dbs.iter().map(|db| db.price.clone()).collect::<Vec<_>>();
    let buy_prices_usd = dbs
        .iter()
        .map(|db| db.buy_price_usd.clone())
        .collect::<Vec<_>>();
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
                nft, 
                collection, 
                price_token, 
                price, 
                buy_price_usd,
                buyer,
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
                $6::numeric[],
                $7::varchar[],
                $8::timestamp[],
                $9::timestamp[],
                $10::direct_buy_state[],
                $11::timestamp[],
                $12::timestamp[],
                $13::bigint[]
            ) 
            on conflict(address) do nothing
        "#,
        addresses as Vec<_>,
        nfts as Vec<_>,
        collections as Vec<_>,
        price_tokens as Vec<_>,
        prices as Vec<_>,
        buy_prices_usd as Vec<_>,
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

async fn update_direct_buy_state(pool: &PgPool, mut dbs: Vec<DirectBuy>) -> Result<()> {
    dbs.sort_by(|a, b| b.tx_lt.cmp(&a.tx_lt));
    let mut last_state_change = HashMap::with_capacity(dbs.len());

    for db in dbs.iter() {
        if !last_state_change.contains_key(&db.address) {
            last_state_change.insert(&db.address, db);
        }
    }

    let mut addresses = Vec::with_capacity(dbs.len());
    let mut finished_at = Vec::with_capacity(dbs.len());
    let mut states = Vec::with_capacity(dbs.len());
    let mut updated = Vec::with_capacity(dbs.len());

    for db in last_state_change.values() {
        addresses.push(db.address.as_str());
        finished_at.push(db.finished_at);
        states.push(db.state.clone());
        updated.push(db.updated);
    }

    sqlx::query!(
        r#"
        update nft_direct_buy set
            state = data.state,
            finished_at = data.finished_at,
            updated = data.updated
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::direct_buy_state[]) as state,
                unnest($3::timestamp[]) as finished_at,
                unnest($4::timestamp[]) as updated
        ) as data
        where nft_direct_buy.address = data.address
        "#,
        addresses as _,
        states as _,
        finished_at as _,
        updated as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
