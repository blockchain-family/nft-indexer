use crate::server::run_api;
use crate::{settings::config::Config, state_updater::run_updater};
use anyhow::Result;
use meta_reader::MetaReaderContext;
use std::net::SocketAddr;
use std::str::FromStr;

mod abi;
mod api;
mod metadata;
mod models;
mod parser;
mod persistence;
mod server;
mod settings;
mod state_updater;
mod utils;

extern crate num;
extern crate num_derive;

// TODO: вынести все получения меты в отдельный сервис (крейт)
// TODO: убрать JrpcClient из параметров save_to_db
// TODO: вынести Api в отдельный крейт
// TODO: отрефакторить indexer-repo

#[tokio::main(worker_threads = 16)]
async fn main() -> Result<()> {
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

    // TODO: make script for re-indexing
    /*
       0. Delete, if exists, database <db_name>_dump
       1. Make dump of the existing database (that is, rename <db_name> to <db_name>_dump)
       2. Create new database <db_name>
       3. Migrate

       Или убрать параметр RESET из конфига (потому что сейчас бесполезен)
    */
    sqlx::migrate!("../indexer-repo/migrations")
        .run(&pg_pool)
        .await?;

    settings::whitelist::init_trusted_addresses(config.clone())?;
    settings::whitelist::init_whitelist_addresses(&pg_pool).await?;

    tokio::spawn(run_updater(pg_pool.clone()));

    let jrpc_client = settings::get_jrpc_client(&config).await?;
    log::info!("Connected to jrpc endpoint");

    let meta_reader_context = MetaReaderContext {
        jrpc_client: jrpc_client.clone(),
        pool: pg_pool.clone(),
    };

    tokio::spawn(meta_reader::run_meta_reader(meta_reader_context));
    tokio::spawn(parser::start_parsing(
        config.clone(),
        pg_pool.clone(),
        jrpc_client.clone(),
    ));

    let socket_addr: SocketAddr =
        SocketAddr::from_str(&config.server_api_url).expect("Invalid socket addr");

    run_api(&socket_addr, pg_pool, jrpc_client)
        .await
        .expect("Failed to run server");

    Ok(())
}
