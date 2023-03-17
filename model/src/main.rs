use crate::server::run_api;
use crate::{settings::config::Config, state_updater::run_updater};
use anyhow::Result;
use indexer::consumer;
use std::net::SocketAddr;
use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};
use transaction_consumer::{ConsumerOptions, TransactionConsumer};

mod api;
mod indexer;
mod server;
mod settings;
mod state_updater;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    stackdriver_logger::init_with_cargo!();
    log::info!("Indexer is preparing to start");

    let config = Config::new();

    let pg_pool = storage::init_pg_pool(&config.database_url, config.database_max_connections)
        .await
        .expect("Postgres connection failed");

    let consumer = init_transactions_consumer(config.clone())
        .await
        .expect("Kafka connection failed");

    {
        let pool = pg_pool.clone();
        tokio::spawn(async move {
            run_updater(pool).await;
        });
    }

    let socket_addr: SocketAddr =
        SocketAddr::from_str(&config.server_api_url).expect("Invalid socket addr");

    {
        let pool = pg_pool.clone();
        let consumer = consumer.clone();
        tokio::spawn(async move { consumer::serve(pool, consumer, config).await });
    }

    run_api(&socket_addr, pg_pool, consumer)
        .await
        .expect("Failed to run server");
    Ok(())
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

    TransactionConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_topic,
        config.states_rpc_endpoints,
        None,
        con_opt,
    )
    .await
}
