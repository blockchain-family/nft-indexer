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

        let nft = "0:b7c55b3f9f82a68e7f6121b49ee58709737f1e38e96288b2531e9a5dc61822e0";
        let collection = "0:2e486ea613d1e9a0ccda5f4ca3f47c8b46de6c70ab9ddb314a3298f5bc4c6b1d";

        let stored_collection = crate::actions::get_collection_by_nft(&nft.into(), &pool)
            .await
            .unwrap()
            .0;
        assert_eq!(stored_collection, collection);
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
    async fn usdt_to_usdt() {
        let token = "0:a519f99bb5d6d51ef958ed24d337ad75a1c770885dcd42d51d6663f9fcdacfb2";
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

        let nft = "0:b7c55b3f9f82a68e7f6121b49ee58709737f1e38e96288b2531e9a5dc61822e0";
        let collection = crate::actions::get_collection_by_nft(&nft.into(), &pool)
            .await
            .unwrap();
        let prices = crate::actions::get_prices(&collection.into(), &pool).await;

        println!("prices: {:#?}", prices.expect("Can't get prices"));
    }
}
