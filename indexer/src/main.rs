use crate::settings::config::Config;
use anyhow::Result;
use data_reader::{MetaReaderContext, PriceReader};
use indexer_api::run_api;
use std::net::SocketAddr;
use std::panic;
use std::str::FromStr;

mod abi;
mod models;
mod parser;
mod persistence;
mod settings;
mod utils;

extern crate num;
extern crate num_derive;

#[tokio::main]
async fn main() -> Result<()> {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
        std::process::exit(1);
    }));

    dotenv::dotenv().ok();
    stackdriver_logger::init_with_cargo!();
    log::info!("Indexer is preparing to start");

    let config = Config::new();
    let pg_pool = indexer_repo::utils::init_pg_pool(
        &config.database_url,
        config.database_max_connections,
        config.terminate_open_connections,
    )
    .await
    .expect("Postgres connection failed");

    sqlx::migrate!("../indexer-repo/migrations")
        .run(&pg_pool)
        .await?;

    let jrpc_client = settings::get_jrpc_client(&config).await?;
    log::info!("Connected to jrpc endpoint");

    let price_reader = PriceReader::new(
        pg_pool.clone(),
        config.bc_name,
        config.idle_after_price_loop_sec,
        config.price_update_frequency_sec,
    )
    .await;

    tokio::spawn(price_reader.clone().run_db_updater());

    let meta_reader_context = MetaReaderContext {
        jrpc_client: jrpc_client.clone(),
        pool: pg_pool.clone(),
        jrpc_req_latency_millis: config.jrpc_req_latency_millis,
        idle_after_loop: config.idle_after_meta_loop_sec,
    };

    tokio::spawn(data_reader::run_meta_reader(meta_reader_context.clone()));

    tokio::spawn(parser::start_parsing(
        config.clone(),
        pg_pool.clone(),
        price_reader,
    ));

    let socket_addr: SocketAddr =
        SocketAddr::from_str(&config.server_api_url).expect("Invalid socket addr");

    run_api(&socket_addr, meta_reader_context)
        .await
        .expect("Failed to run server");

    Ok(())
}
