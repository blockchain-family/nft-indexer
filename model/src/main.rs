use crate::settings::config::Config;
use anyhow::Result;
use env_logger::Builder;
use indexer::consumer;
use log::LevelFilter;
use std::{collections::HashMap, sync::Arc};
use transaction_consumer::{ConsumerOptions, TransactionConsumer};

mod indexer;
mod settings;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Info).init();

    let config = Config::new("Settings.toml");

    let pg_pool = storage::init_pg_pool(&config.database_url, config.database_max_connections)
        .await
        .expect("Postgres connection failed");

    let consumer = init_transactions_consumer(config.clone())
        .await
        .expect("Kafka connection failed");

    consumer::serve(pg_pool, consumer).await
}

pub async fn init_transactions_consumer(config: Config) -> Result<Arc<TransactionConsumer>> {
    log::info!("Initializing consumer");

    let kafka_options = HashMap::from_iter(
        config
            .kafka_settings
            .iter()
            .map(|(param, val)| (param.as_str(), val.as_str())),
    );

    let con_opt = ConsumerOptions {
        kafka_options,
        skip_0_partition: false,
    };

    transaction_consumer::TransactionConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_topic,
        &config.states_rpc_endpoints,
        None,
        con_opt,
    )
    .await
}
