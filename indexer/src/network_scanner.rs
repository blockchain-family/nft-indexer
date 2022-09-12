use std::sync::Arc;
use anyhow::{Context, Result};
use model::subscriber::IndexerSubscriber;
use crate::config::*;


pub struct NetworkScanner {
    indexer: Arc<ton_indexer::Engine>,
}

impl NetworkScanner {
    pub async fn new(
        settings: AppConfig,
        node_settings: NodeConfig,
        global_config: ton_indexer::GlobalConfig,
    ) -> Result<Arc<Self>> {
        let subscriber: Arc<dyn ton_indexer::Subscriber> = IndexerSubscriber::new(settings.model).await?;

        let indexer = ton_indexer::Engine::new(
            node_settings
                .build_indexer_config()
                .await
                .context("Failed to build node config")?,
            global_config,
            vec![subscriber],
        )
        .await
        .context("Failed to start node")?;

        Ok(Arc::new(Self {
            indexer,
        }))
    }

    pub async fn start(self: &Arc<Self>) -> Result<()> {
        log::info!("staring indexer");
        self.indexer.start().await?;
        Ok(())
    }

    pub fn indexer(&self) -> &ton_indexer::Engine {
        self.indexer.as_ref()
    }

    /*pub async fn send_message(&self, message: ton_block::Message) -> Result<(), QueryError> {
        let to = match message.header() {
            ton_block::CommonMsgInfo::ExtInMsgInfo(header) => header.dst.workchain_id(),
            _ => return Err(QueryError::ExternalTonMessageExpected),
        };

        let cells = message
            .write_to_new_cell()
            .map_err(|_| QueryError::FailedToSerialize)?
            .into();

        let serialized =
            ton_types::serialize_toc(&cells).map_err(|_| QueryError::FailedToSerialize)?;

        self.indexer
            .broadcast_external_message(to, &serialized)
            .map_err(|_| QueryError::ConnectionError)
    }*/
}
