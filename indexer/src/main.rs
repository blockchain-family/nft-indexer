use crate::settings::config::Config;
use anyhow::Result;
use data_reader::{MetaReaderContext, PriceReaderContext};
use indexer_api::run_api;
use std::net::SocketAddr;
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
    dotenv::dotenv().ok();
    stackdriver_logger::init_with_cargo!();
    log::info!("Indexer is preparing to start");

    let config = Config::new();
    log::info!("DB URL {:?}", &config.database_url);

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

    let meta_reader_context = MetaReaderContext {
        jrpc_client: jrpc_client.clone(),
        pool: pg_pool.clone(),
        jrpc_req_latency_millis: config.jrpc_req_latency_millis,
        idle_after_loop: config.idle_after_meta_loop_sec,
    };

    tokio::spawn(data_reader::run_meta_reader(meta_reader_context.clone()));

    let ctx = PriceReaderContext {
        pool: pg_pool.clone(),
        bc: config.bc_name,
        idle_after_loop: config.idle_after_price_loop_sec,
    };

    tokio::spawn(data_reader::run_price_reader(ctx));

    tokio::spawn(parser::start_parsing(config.clone(), pg_pool.clone()));

    let socket_addr: SocketAddr =
        SocketAddr::from_str(&config.server_api_url).expect("Invalid socket addr");

    run_api(&socket_addr, meta_reader_context)
        .await
        .expect("Failed to run server");

    Ok(())
}
