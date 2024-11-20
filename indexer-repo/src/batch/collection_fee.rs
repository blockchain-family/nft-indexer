use anyhow::{anyhow, Result};
use sqlx::PgConnection;

use crate::types::decoded::CollectionFee;

pub async fn update_collection_fee(tx: &mut PgConnection, data: &[CollectionFee]) -> Result<()> {
    let addresses = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    let nums = data.iter().map(|e| e.numerator).collect::<Vec<_>>();
    let denoms = data.iter().map(|e| e.denominator).collect::<Vec<_>>();
    let ts = data.iter().map(|e| e.timestamp).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_collection set
            fee_numerator   = data.num, 
            fee_denominator = data.den,
            updated         = greatest(data.ts, nft_collection.updated)
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::integer[]) as num,
                unnest($3::integer[]) as den,
                unnest($4::timestamp[]) as ts
        ) as data
        where nft_collection.address = data.address
    "#,
        addresses as _,
        nums as _,
        denoms as _,
        ts as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
