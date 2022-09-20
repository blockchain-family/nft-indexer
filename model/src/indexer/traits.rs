use anyhow::Result;
use nekoton_abi::transaction_parser::ExtractedOwned;
use sqlx::PgPool;
use std::sync::Arc;
use transaction_consumer::TransactionConsumer;

pub trait ContractEvent {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized;
}
