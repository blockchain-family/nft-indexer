use crate::abi::declare_abi::*;
use crate::abi::scope;
use crate::settings::config::Config;
use anyhow::Result;
use everscale_rpc_client::{ClientOptions, RpcClient};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use transaction_buffer::models::{
    AnyExtractable, BufferedConsumerChannels, BufferedConsumerConfig,
};
use transaction_buffer::start_parsing_and_get_channels;
use transaction_consumer::{ConsumerOptions, TransactionConsumer};

pub mod config;
pub async fn init_consumer(config: &Config) -> Result<Arc<TransactionConsumer>> {
    log::info!("initializing transaction consumer");

    let kafka_options = HashMap::from_iter(
        config
            .kafka_settings
            .iter()
            .map(|(param, val)| (param.as_str(), val.as_str())),
    );

    let con_opt = ConsumerOptions {
        kafka_options,
        skip_0_partition: true,
    };

    TransactionConsumer::without_rpc_client(
        &config.kafka_consumer_group,
        &config.kafka_topic,
        con_opt,
    )
    .await
}

pub async fn init_transaction_buffer(
    config: &Config,
    pg_pool: &PgPool,
) -> Result<BufferedConsumerChannels> {
    let transaction_consumer = init_consumer(config).await?;

    log::info!("starting transaction buffer");
    Ok(start_parsing_and_get_channels(BufferedConsumerConfig {
        transaction_consumer,
        pg_pool: pg_pool.clone(),
        any_extractable: get_any_extractable(),
        buff_size: 100_000,
        commit_time_secs: 100,
        cache_timer: 60,
        save_failed_transactions_for_accounts: vec![],
    }))
}

fn get_any_extractable() -> Vec<AnyExtractable> {
    // NOTE: из-за того, что есть два ивента NftCreated,
    // но с разными полями, будет выскакивать ошибка
    // для ивента из Nft.abi.json

    let extractables = vec![
        auction_root_tip3(),
        auction_tip3(),
        callbacks(),
        direct_buy(),
        direct_sell(),
        factory_direct_buy(),
        factory_direct_sell(),
        mint_and_sell(),
        nft(),
        collection(),
    ]
    .into_iter()
    .flat_map(|c| {
        c.events
            .clone()
            .into_values()
            .filter(|e| scope::events().contains(&e.name.as_str()))
            .map(AnyExtractable::Event)
            .chain(
                c.functions
                    .clone()
                    .into_values()
                    .filter(|f| scope::functions().contains(&f.name.as_str()))
                    .map(AnyExtractable::Function),
            )
    })
    .collect::<Vec<_>>();

    log::info!(
        "List of extractables to parse:\n{:#?}",
        extractables
            .iter()
            .map(get_extractable_name)
            .collect::<Vec<_>>()
    );

    extractables
}

fn get_extractable_name(extractable: &AnyExtractable) -> String {
    match extractable {
        AnyExtractable::Event(event) => format!("{} (event)", event.name),
        AnyExtractable::Function(function) => format!("{} (function)", function.name),
    }
}

pub async fn get_jrpc_client(config: &Config) -> Result<RpcClient> {
    RpcClient::new(
        config.states_rpc_endpoints.clone(),
        ClientOptions::default(),
    )
    .await
}
