use anyhow::{anyhow, Result};
use sqlx::PgConnection;

use crate::types::{decoded::AuctionActive, AuctionStatus};

pub async fn save_auc_active(tx: &mut PgConnection, data: &[AuctionActive]) -> Result<()> {
    let addresses = data.iter().map(|a| a.address.as_str()).collect::<Vec<_>>();
    let wallets = data
        .iter()
        .map(|a| a.wallet_for_bids.as_str())
        .collect::<Vec<_>>();
    let price_tokens = data
        .iter()
        .map(|a| a.price_token.as_str())
        .collect::<Vec<_>>();
    let start_prices = data
        .iter()
        .map(|a| a.start_price.clone())
        .collect::<Vec<_>>();
    let min_bids = data.iter().map(|a| a.min_bid.clone()).collect::<Vec<_>>();
    let created = data.iter().map(|a| a.created_at).collect::<Vec<_>>();
    let finished = data.iter().map(|a| a.finished_at).collect::<Vec<_>>();
    let tx_lts = data.iter().map(|a| a.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_auction set
            wallet_for_bids = data.wallet,
            price_token = data.price_token,
            start_price = data.start_price,
            min_bid = data.min_bid,
            created_at = data.created,
            finished_at = data.finished,
            tx_lt = data.tx_lt,
            status = data.status
        from (
            select
                unnest($1::varchar[]) as address,
                unnest($2::varchar[]) as wallet,
                unnest($3::varchar[]) as price_token,
                unnest($4::numeric[]) as start_price,
                unnest($5::numeric[]) as min_bid,
                unnest($6::timestamp[]) as created,
                unnest($7::timestamp[]) as finished,
                unnest($8::bigint[]) as tx_lt,
                $9::auction_status as status
        ) as data
        where nft_auction.address = data.address
        "#,
        addresses as _,
        wallets as _,
        price_tokens as _,
        start_prices as _,
        min_bids as _,
        created as _,
        finished as _,
        tx_lts as _,
        AuctionStatus::Active as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
