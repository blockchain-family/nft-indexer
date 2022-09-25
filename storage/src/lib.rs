pub mod actions;
pub mod traits;
pub mod types;
pub mod tables;
pub mod api_service;
pub mod cfg;

use anyhow::Result;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    {ConnectOptions, PgPool},
};
use std::{str::FromStr, time::Duration};

pub async fn init_pg_pool(db_string: &str, pool_size: u32) -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(pool_size)
        .connect_with(std::mem::take(
            PgConnectOptions::from_str(db_string)?
                .log_statements(LevelFilter::Debug)
                .log_slow_statements(LevelFilter::Debug, Duration::from_secs(10)),
        ))
        .await?)
}
