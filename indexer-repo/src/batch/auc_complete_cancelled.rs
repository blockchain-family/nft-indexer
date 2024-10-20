use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::{
    decoded::{AuctionCancelled, AuctionComplete},
    AuctionStatus,
};

pub async fn save_auc_complete(
    tx: &mut Transaction<'_, Postgres>,
    data: &[AuctionComplete],
) -> Result<()> {
    let addresses = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    let max_bids = data.iter().map(|e| e.max_bid.clone()).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_auction set
            max_bid = data.max_bid,
            status = data.status
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::numeric[]) as max_bid,
                $3::auction_status as status
        ) as data
        where nft_auction.address = data.address
    "#,
        addresses as _,
        max_bids as _,
        AuctionStatus::Completed as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

pub async fn save_auc_cancelled(
    tx: &mut Transaction<'_, Postgres>,
    data: &[AuctionCancelled],
) -> Result<()> {
    let addresses = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_auction set
            status = data.status
        from
        (
            select 
                unnest($1::varchar[]) as address,
                $2::auction_status as status
        ) as data
        where nft_auction.address = data.address
    "#,
        addresses as _,
        AuctionStatus::Cancelled as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
