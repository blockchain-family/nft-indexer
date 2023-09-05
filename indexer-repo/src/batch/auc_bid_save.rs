use anyhow::{anyhow, Result};
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
    let created_at = data.iter().map(|e| e.created_at).collect::<Vec<_>>();
    let tx_lts = data.iter().map(|e| e.tx_lt).collect::<Vec<_>>();
    let declined = data.iter().map(|e| e.declined).collect::<Vec<_>>();
    let nfts = data.iter().map(|e| e.nft.as_str()).collect::<Vec<_>>();
    let nfts_owners = data
        .iter()
        .map(|e| e.nft_owner.as_str())
        .collect::<Vec<_>>();
    let collections = data
        .iter()
        .map(|e| e.collection.as_str())
        .collect::<Vec<_>>();
    let price_tokens = data
        .iter()
        .map(|e| e.price_token.as_str())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_auction_bid (
                auction,
                buyer,
                price,
                next_bid_value, 
                created_at,
                tx_lt,
                declined,
                nft,
                nft_owner,
                collection,
                price_token
            )
            select
                unnest($1::varchar[]),
                unnest($2::varchar[]),
                unnest($3::numeric[]),
                unnest($4::numeric[]),
                unnest($5::timestamp[]),
                unnest($6::bigint[]),
                unnest($7::boolean[]),
                unnest($8::varchar[]),
                unnest($9::varchar[]),
                unnest($10::varchar[]),
                unnest($11::varchar[])
            on conflict (auction, buyer, price, created_at) do nothing
        "#,
        auctions as _,
        buyers as _,
        bid_vals as _,
        next_vals as _,
        created_at as _,
        tx_lts as _,
        declined as _,
        nfts as _,
        nfts_owners as _,
        collections as _,
        price_tokens as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
