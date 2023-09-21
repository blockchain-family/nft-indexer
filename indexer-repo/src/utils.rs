use anyhow::Result;
use log::LevelFilter;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Executor, PgPool};
use std::str::FromStr;
use std::time::Duration;

pub async fn init_pg_pool(
    db_string: &str,
    pool_size: u32,
    terminate_open_connections: Option<bool>,
) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(pool_size)
        .after_connect(|conn, _| {
            Box::pin(async move { conn.execute("select 1").await.map(|_| ()) })
        })
        .connect_with(
            PgConnectOptions::from_str(db_string)?
                .log_statements(LevelFilter::Debug)
                .log_slow_statements(LevelFilter::Debug, Duration::from_secs(10))
                .clone(),
        )
        .await?;

    if terminate_open_connections.unwrap_or_default() {
        sqlx::query(
            r#"
            select pg_terminate_backend(pid)
            from pg_stat_activity
            where datname = current_database()
              and query not like 'select pg_terminate_backend%'
            "#,
        )
        .execute(&pool)
        .await?;
    }

    log::info!("connected to database");

    Ok(pool)
}
