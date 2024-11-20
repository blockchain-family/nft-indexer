use anyhow::{anyhow, Result};
use sqlx::PgConnection;

use crate::types::decoded::AuctionBid;

pub async fn update_auc_maxmin(tx: &mut PgConnection, data: &[AuctionBid]) -> Result<()> {
    let addresses = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    let min_bids = data
        .iter()
        .map(|e| e.next_value.clone())
        .collect::<Vec<_>>();
    let max_bids = data.iter().map(|e| e.bid_value.clone()).collect::<Vec<_>>();
    let tx_lts = data.iter().map(|e| e.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_auction set
            min_bid = data.min_bid,
            max_bid = data.max_bid,
            tx_lt = data.tx_lt
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::numeric[]) as min_bid,
                unnest($3::numeric[]) as max_bid,
                unnest($4::bigint[]) as tx_lt
        ) as data
        where nft_auction.address = data.address
    "#,
        addresses as _,
        min_bids as _,
        max_bids as _,
        tx_lts as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
