use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::types::decoded::AuctionBid;

pub async fn save_auc_bid(pool: &PgPool, data: &[AuctionBid]) -> Result<()> {
    let auctions = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    let buyers = data.iter().map(|e| e.buyer.as_str()).collect::<Vec<_>>();
    let bid_vals = data.iter().map(|e| e.bid_value.clone()).collect::<Vec<_>>();
    let next_vals = data
        .iter()
        .map(|e| e.next_value.clone())
        .collect::<Vec<_>>();
    let created_at = data
        .iter()
        .map(|e| NaiveDateTime::from_timestamp_opt(e.created_at, 0))
        .collect::<Vec<_>>();
    let tx_lts = data.iter().map(|e| e.tx_lt).collect::<Vec<_>>();
    let declined = data.iter().map(|e| e.declined).collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_auction_bid (
                auction,
                buyer,
                price,
                next_bid_value, 
                created_at,
                tx_lt,
                declined
            )
            select
                unnest($1::varchar[]),
                unnest($2::varchar[]),
                unnest($3::numeric[]),
                unnest($4::numeric[]),
                unnest($5::timestamp[]),
                unnest($6::bigint[]),
                unnest($7::boolean[])
        "#,
        auctions as _,
        buyers as _,
        bid_vals as _,
        next_vals as _,
        created_at as _,
        tx_lts as _,
        declined as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
