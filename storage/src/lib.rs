pub mod actions;
pub mod traits;
pub mod types;

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

#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn collection_by_nft() {
        let db_url = env::var("DATABASE_URL").unwrap();

        let pool = crate::init_pg_pool(&db_url, 5)
            .await
            .expect("Postgres connection failed");

        let nft = "0:986f1aedf80d63b814ce9d68a90fa28e952456a8f2421069bce32c27e95155c5";
        let collection = "0:7292647c4471a6b65460b9f3211915427e22126a0c39a5f7a4b582896389f812";

        let stored_collection = crate::actions::get_collection_by_nft(&nft.into(), &pool)
            .await
            .unwrap();

        assert_eq!(stored_collection.0, collection);
    }

    #[tokio::test]
    async fn token_to_usdt() {
        let token = "0:a49cd4e158a9a15555e624759e2e4e766d22600b7800d891e46f9291f044a93d";
        let usdt = crate::actions::token_to_usdt(token)
            .await
            .expect("Can't get usdt price");

        println!("usdt = {:#?}", usdt);
    }

    #[tokio::test]
    async fn prices() {
        let db_url = env::var("DATABASE_URL").unwrap();

        let pool = crate::init_pg_pool(&db_url, 5)
            .await
            .expect("Postgres connection failed");

        let nft = "0:986f1aedf80d63b814ce9d68a90fa28e952456a8f2421069bce32c27e95155c5";
        let collection = crate::actions::get_collection_by_nft(&nft.into(), &pool)
            .await
            .unwrap();

        let mut tx = pool.begin().await.unwrap();

        let prices = crate::actions::get_prices(&collection.into(), &mut tx).await;

        tx.commit().await.unwrap();
        println!("prices: {:#?}", prices.expect("Can't get prices"));
    }
}
