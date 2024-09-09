use crate::types::AuctionCachedInfo;
use anyhow::{anyhow, Result};
use sqlx::PgPool;

pub async fn get_collection_of_nft<S: AsRef<str>>(
    pg_pool: &PgPool,
    nft: S,
) -> Result<Option<String>> {
    sqlx::query_scalar!(
        r#"
        select collection
        from nft
        where address = $1
        "#,
        nft.as_ref(),
    )
    .fetch_optional(pg_pool)
    .await
    .map_err(|e| anyhow!(e))
}

pub async fn get_auction_cached_info<S: AsRef<str>>(
    pg_pool: &PgPool,
    auction: S,
) -> Result<Option<AuctionCachedInfo>> {
    sqlx::query_as!(
        AuctionCachedInfo,
        r#"
        select nft,
               collection,
               nft_owner,
               price_token,
               extract('epoch' from created_at)::bigint start_time
        from nft_auction
        where address = $1
        "#,
        auction.as_ref(),
    )
    .fetch_optional(pg_pool)
    .await
    .map_err(|e| anyhow!(e))
}
