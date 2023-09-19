use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::decoded::NftPriceHistory;

pub async fn save_price_history(
    tx: &mut Transaction<'_, Postgres>,
    data: &[NftPriceHistory],
) -> Result<()> {
    let sources = data.iter().map(|e| e.source.as_str()).collect::<Vec<_>>();
    let source_types = data.iter().map(|e| e.source_type).collect::<Vec<_>>();
    let created_at = data.iter().map(|e| e.created_at).collect::<Vec<_>>();
    let prices = data.iter().map(|e| e.price.clone()).collect::<Vec<_>>();
    let price_tokens = data
        .iter()
        .map(|e| e.price_token.as_str())
        .collect::<Vec<_>>();
    let nft = data.iter().map(|e| e.nft.as_str()).collect::<Vec<_>>();
    let usd_prices = data.iter().map(|e| e.usd_price.clone()).collect::<Vec<_>>();
    let collections = data
        .iter()
        .map(|e| e.collection.as_str())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_price_history (
                source, 
                source_type, 
                ts, 
                price,
                price_token, 
                nft,
                usd_price,
                collection
            )
            select
                unnest($1::varchar[]),
                unnest($2::nft_price_source[]),
                unnest($3::timestamp[]),
                unnest($4::numeric[]),
                unnest($5::varchar[]),
                unnest($6::varchar[]),
                unnest($7::numeric[]),
                unnest($8::varchar[])
        "#,
        sources as _,
        source_types as _,
        created_at as _,
        prices as _,
        price_tokens as _,
        nft as _,
        usd_prices as _,
        collections as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
