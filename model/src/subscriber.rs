use ton_indexer::utils::*;
use storage::PgPool;
use crate::cfg::*;
use std::sync::Arc;
use anyhow::Result;
use ton_indexer::ProcessBlockContext;
use ton_block::Transaction;

pub struct IndexerSubscriber {
    //cfg: ModelConfig,
    //states: StatesClient,
    _db: PgPool,
    //parser: TransactionParser,
}

impl IndexerSubscriber {
    pub async fn new(cfg: ModelConfig) -> Result<Arc<dyn ton_indexer::Subscriber>> {
        //let parser = TransactionParser::builder()
            //.function_input(function, false)
            //.function_output(function, allow_partial_match)
            //.build()?;
        let _db = cfg.database.init().await?;
        //let states = RpcConfig::from(states_address).init().await?;
        Ok(Arc::new(Self { _db, /*cfg, parser*/ }))
    }

    pub async fn new2(cfg: ModelConfig) -> Result<Arc<Self>> {
        //let parser = TransactionParser::builder()
            //.function_input(function, false)
            //.function_output(function, allow_partial_match)
            //.build()?;
        let _db = cfg.database.init().await?;
        //let states = RpcConfig::from(states_address).init().await?;
        Ok(Arc::new(Self { _db, /*cfg, parser*/ }))
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

    async fn process_full_state(&self, state: &ShardStateStuff) -> Result<()> {
        log::error!("IndexSubscriber process_full_state");
        Ok(())
    }
}
