use crate::settings::config::Config;
use anyhow::Result;
use env_logger::Builder;
use indexer::consumer;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    {ConnectOptions, PgPool},
};
use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};
use transaction_consumer::{ConsumerOptions, TransactionConsumer};

mod database;
mod indexer;
mod settings;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Info).init();

    let config = Arc::new(Config::new("Settings.toml"));

    let pg_pool = Arc::<PgPool>::new(
        init_pg_pool(&config.database_url, config.database_max_connections)
            .await
            .expect("Postgres connection failed"),
    );

    let consumer = init_transactions_consumer(config.clone())
        .await
        .expect("Kafka connection failed");

    consumer::serve(pg_pool, consumer).await
}

pub async fn init_transactions_consumer(config: Arc<Config>) -> Result<Arc<TransactionConsumer>> {
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
        std::iter::empty::<&str>(),
        None,
        con_opt,
    )
    .await
}

pub async fn init_pg_pool(db_string: &str, pool_size: u32) -> Result<PgPool> {
    log::info!("Connecting to DB");

    Ok(PgPoolOptions::new()
        .max_connections(pool_size)
        .connect_with(std::mem::take(
            PgConnectOptions::from_str(db_string)?
                .log_statements(LevelFilter::Debug)
                .log_slow_statements(LevelFilter::Debug, Duration::from_secs(10)),
        ))
        .await?)
}
