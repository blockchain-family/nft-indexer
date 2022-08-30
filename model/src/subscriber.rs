use crate::cfg::*;
use anyhow::Result;
use std::sync::Arc;
use storage::PgPool;
use ton_block::{HashmapAugType, Transaction};
use ton_indexer::utils::ShardStateStuff;
use ton_indexer::ProcessBlockContext;

pub struct IndexerSubscriber {
    //cfg: ModelConfig,
    //states: StatesClient,
    _db: PgPool,
    //parser: TransactionParser,
}

impl IndexerSubscriber {
    pub async fn new(cfg: ModelConfig) -> Result<Self> {
        let _db = cfg.database.init().await?;
        Ok(Self { _db })
    }

    pub async fn new2(cfg: ModelConfig) -> Result<Arc<Self>> {
        let _db = cfg.database.init().await?;
        Ok(Arc::new(Self { _db }))
    }

    pub async fn handle_transaction(&self, _tran: Transaction) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl ton_indexer::Subscriber for IndexerSubscriber {
    async fn process_block(&self, ctx: ProcessBlockContext<'_>) -> Result<()> {
        log::error!("IndexSubscriber process_block");
        let block_stuff = ctx.block_stuff();
        let _block_id = block_stuff.id();
        let _block = block_stuff.block();
        let block_extra = _block.read_extra()?;

        // Process transactions
        let mut ts = Vec::with_capacity(128);
        block_extra
            .read_account_blocks()?
            .iterate_objects(|account_block| {
                account_block.transactions().iterate_objects(|t| {
                    ts.push(self.handle_transaction(t.as_ref().clone()));
                    Ok(true)
                })?;
                Ok(true)
            })?;
        futures::future::join_all(ts).await;
        Ok(())
    }

    async fn process_full_state(&self, _state: &ShardStateStuff) -> Result<()> {
        log::error!("IndexSubscriber process_full_state");
        Ok(())
    }
}
