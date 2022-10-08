use anyhow::Result;
use async_trait::async_trait;
use nekoton_abi::transaction_parser::ExtractedOwned;
use sqlx::PgPool;
use std::sync::Arc;
use transaction_consumer::TransactionConsumer;

#[async_trait]
pub trait ContractEvent {
    fn build_from(
        event: &ExtractedOwned,
        pool: &PgPool,
        consumer: &Arc<TransactionConsumer>,
    ) -> Result<Self>
    where
        Self: Sized;

    async fn update_dependent_tables(&mut self) -> Result<()>;
}
