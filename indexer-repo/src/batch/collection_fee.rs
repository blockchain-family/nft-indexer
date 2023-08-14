use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::CollectionFeeDecoded;

pub async fn update_collection_fee(pool: &PgPool, data: Vec<CollectionFeeDecoded>) -> Result<()> {
    let addresses = data.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    let nums = data.iter().map(|e| e.numerator).collect::<Vec<_>>();
    let denoms = data.iter().map(|e| e.denominator).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        update nft_collection set
            fee_numerator   = data.num, 
            fee_denominator = data.den
        from
        (
            select 
                unnest($1::varchar[]) as address,
                unnest($2::integer[]) as num,
                unnest($3::integer[]) as den
        ) as data
        where nft_collection.address = data.address
    "#,
        addresses as _,
        nums as _,
        denoms as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
