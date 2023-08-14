use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::AuctionActiveDecoded;

pub async fn save_auc_acitve(pool: &PgPool, data: Vec<AuctionActiveDecoded>) -> Result<()> {
    let addresses = data.iter().map(|n| n.address).collect::<Vec<_>>();
    let nft = data.iter().map(|n| n.nft).collect::<Vec<_>>();
    let wallets = data.iter().map(|n| n.wallet_for_bids).collect::<Vec<_>>();
    let price_tokens = data.iter().map(|n| n.price_token).collect::<Vec<_>>();
    let start_prices = data.iter().map(|n| n.start_price).collect::<Vec<_>>();
    let min_bids = data.iter().map(|n| n.min_bid).collect::<Vec<_>>();
    let created = data.iter().map(|n| n.created_at).collect::<Vec<_>>();
    let finished = data.iter().map(|n| n.finished_at).collect::<Vec<_>>();
    let tx = data.iter().map(|n| n.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        insert into nft_auction (
            address, 
            nft, 
            wallet_for_bids, 
            price_token,
            start_price,
            closing_price_usd, 
            min_bid,
            max_bid, 
            status, 
            created_at,
            finished_at, 
            tx_lt
        )

            select * from unnest(
                $1,
                $2,
                $3,
                $4,
                $5::timestamp[],
                $6::bigint[], 
                $7::bigint[]) 
            on conflict(address) do nothing
        "#,
        &addresses[..],
        &nft[..],
        &wallets[..],
        &price_tokens[..],
        start_prices as _,
        min_bids as _,
        created as _,
        finished as _,
        tx as _
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
