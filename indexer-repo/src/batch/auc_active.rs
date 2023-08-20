use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::types::{decoded::AuctionActive, AuctionStatus};

pub async fn save_auc_acitve(pool: &PgPool, data: Vec<AuctionActive>) -> Result<()> {
    let addresses = data.iter().map(|n| n.address.as_str()).collect::<Vec<_>>();
    let nft = data.iter().map(|n| n.nft.as_str()).collect::<Vec<_>>();
    let wallets = data
        .iter()
        .map(|n| n.wallet_for_bids.as_str())
        .collect::<Vec<_>>();
    let price_tokens = data
        .iter()
        .map(|n| n.price_token.as_str())
        .collect::<Vec<_>>();
    let start_prices = data
        .iter()
        .map(|n| n.start_price.clone())
        .collect::<Vec<_>>();
    let min_bids = data.iter().map(|n| n.min_bid.clone()).collect::<Vec<_>>();
    let created = data
        .iter()
        .map(|n| NaiveDateTime::from_timestamp_opt(n.created_at, 0).unwrap_or_default())
        .collect::<Vec<_>>();
    let finished = data
        .iter()
        .map(|n| NaiveDateTime::from_timestamp_opt(n.finished_at, 0))
        .collect::<Vec<_>>();
    let tx = data.iter().map(|n| n.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        insert into nft_auction (
            address, 
            nft, 
            wallet_for_bids, 
            price_token,
            start_price,
            min_bid,
            created_at,
            finished_at, 
            tx_lt,
            closing_price_usd, 
            max_bid, 
            status
        )

            select *, 0, 0, $10
            from unnest(
                $1::varchar[],
                $2::varchar[],
                $3::varchar[],
                $4::varchar[],
                $5::numeric[],
                $6::numeric[], 
                $7::timestamp[],
                $8::timestamp[],
                $9::bigint[]
            ) 
            on conflict(address) do nothing
        "#,
        addresses as _,
        nft as _,
        wallets as _,
        price_tokens as _,
        start_prices as _,
        min_bids as _,
        created as _,
        finished as _,
        tx as _,
        AuctionStatus::Active as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
