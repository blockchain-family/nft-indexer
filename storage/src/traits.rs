use anyhow::Result;
use async_trait::async_trait;
use nekoton_abi::transaction_parser::ExtractedOwned;
use sqlx::{postgres::PgQueryResult, PgPool};
use ton_block::MsgAddressInt;

pub trait EventRecord {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized;

    fn get_nft(&self) -> Option<MsgAddressInt> {
        None
    }
}

#[async_trait]
pub trait DatabaseRecord {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync;
}
