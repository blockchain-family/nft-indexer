use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::decoded::SetRoyalty;

pub async fn update_direct_sell(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let address = royalties
        .iter()
        .map(|e| e.address.as_str())
        .collect::<Vec<_>>();
    let numerator = royalties.iter().map(|e| e.numerator).collect::<Vec<_>>();
    let denominator = royalties.iter().map(|e| e.denominator).collect::<Vec<_>>();
    sqlx::query!(
        r#"
        update nft_direct_sell set
            royalty_numerator = data.numerator,
            royalty_denominator = data.denominator
        from
        (
            select
                unnest($1::varchar[]) as address,
                unnest($2::integer[]) as numerator,
                unnest($3::integer[]) as denominator
        ) as data
        where nft_direct_sell.address = data.address
    "#,
        address as _,
        numerator as _,
        denominator as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

pub async fn update_direct_buy(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let address = royalties
        .iter()
        .map(|e| e.address.as_str())
        .collect::<Vec<_>>();
    let numerator = royalties.iter().map(|e| e.numerator).collect::<Vec<_>>();
    let denominator = royalties.iter().map(|e| e.denominator).collect::<Vec<_>>();
    sqlx::query!(
        r#"
        update nft_direct_buy set
            royalty_numerator = data.numerator,
            royalty_denominator = data.denominator
        from
        (
            select
                unnest($1::varchar[]) as address,
                unnest($2::integer[]) as numerator,
                unnest($3::integer[]) as denominator
        ) as data
        where nft_direct_buy.address = data.address
    "#,
        address as _,
        numerator as _,
        denominator as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

pub async fn update_auction(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let address = royalties
        .iter()
        .map(|e| e.address.as_str())
        .collect::<Vec<_>>();
    let numerator = royalties.iter().map(|e| e.numerator).collect::<Vec<_>>();
    let denominator = royalties.iter().map(|e| e.denominator).collect::<Vec<_>>();
    sqlx::query!(
        r#"
        update nft_auction set
            royalty_numerator = data.numerator,
            royalty_denominator = data.denominator
        from
        (
            select
                unnest($1::varchar[]) as address,
                unnest($2::integer[]) as numerator,
                unnest($3::integer[]) as denominator
        ) as data
        where nft_auction.address = data.address
    "#,
        address as _,
        numerator as _,
        denominator as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
