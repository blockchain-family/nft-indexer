use crate::database::{records::AuctionDeployedRecord, requests::AuctionDeployedRequest};
use anyhow::Result;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn put_auction_deployed_record(
    _pool: &PgPool,
    _record: &AuctionDeployedRecord,
) -> Result<PgQueryResult> {
    // TODO: Ok(sqlx::query!().execute(pool).await?)
    Ok(PgQueryResult::default())
}

pub async fn _get_limit_order_state_changed_records(
    _pool: &PgPool,
    _request: AuctionDeployedRequest,
) -> Result<(Vec<AuctionDeployedRecord>, i64)> {
    todo!()
}
