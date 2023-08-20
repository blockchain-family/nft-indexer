use anyhow::{anyhow, Result};
use sqlx::PgPool;

use crate::types::decoded::NftPriceHistory;

pub async fn save_price_history(pool: &PgPool, data: Vec<NftPriceHistory>) -> Result<()> {
    let sources = data.iter().map(|e| e.source.as_str()).collect::<Vec<_>>();
    let source_types = data.iter().map(|e| e.source_type).collect::<Vec<_>>();
    let created_at = data.iter().map(|e| e.created_at).collect::<Vec<_>>();
    let prices = data.iter().map(|e| e.price.clone()).collect::<Vec<_>>();
    let price_tokens = data
        .iter()
        .map(|e| e.price_token.as_deref())
        .collect::<Vec<_>>();
    let nft = data.iter().map(|e| e.nft.as_deref()).collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_price_history (
                source, 
                source_type, 
                ts, 
                price,
                price_token, 
                nft
            )
            select *
            from unnest(
                $1::varchar[],
                $2::nft_price_source[],
                $3::timestamp[],
                $4::numeric[],
                $5::varchar[],
                $6::varchar[]
            ) 
        "#,
        sources as _,
        source_types as _,
        created_at as _,
        prices as _,
        price_tokens as _,
        nft as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
