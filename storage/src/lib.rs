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

pub async fn init_pg_pool(db_string: &str, _pool_size: u32) -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(50)
        .connect_with(std::mem::take(
            PgConnectOptions::from_str(db_string)?
                .log_statements(LevelFilter::Debug)
                .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1)),
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

        let mut tx = pool.begin().await.unwrap();

        let nft = "0:b7c55b3f9f82a68e7f6121b49ee58709737f1e38e96288b2531e9a5dc61822e0";
        let collection = "0:2e486ea613d1e9a0ccda5f4ca3f47c8b46de6c70ab9ddb314a3298f5bc4c6b1d";

        let stored_collection = crate::actions::get_collection_by_nft(&nft.into(), &mut tx)
            .await
            .unwrap()
            .0;
        assert_eq!(stored_collection, collection);
    }

    #[tokio::test]
    async fn ever_to_usd() {
        let token = "0:a49cd4e158a9a15555e624759e2e4e766d22600b7800d891e46f9291f044a93d";
        let usd = rpc::token_to_usd(token).await.expect("Can't get usd price");

        println!("usd = {:#?}", usd);
    }

    #[tokio::test]
    async fn bridge_to_usd() {
        let token = "0:f2679d80b682974e065e03bf42bbee285ce7c587eb153b41d761ebfd954c45e1";
        let usd = rpc::token_to_usd(token).await.expect("Can't get usd price");

        println!("usd = {:#?}", usd);
    }

    #[tokio::test]
    async fn qube_to_usd() {
        let token = "0:9f20666ce123602fd7a995508aeaa0ece4f92133503c0dfbd609b3239f3901e2";
        let usd = rpc::token_to_usd(token).await.expect("Can't get usd price");

        println!("usd = {:#?}", usd);
    }

    #[tokio::test]
    async fn usdt_to_usd() {
        let token = "0:a519f99bb5d6d51ef958ed24d337ad75a1c770885dcd42d51d6663f9fcdacfb2";
        let usd = rpc::token_to_usd(token).await.expect("Can't get usd price");

        println!("usd = {:#?}", usd);
    }

    #[tokio::test]
    async fn usdc_to_usd() {
        let token = "0:c37b3fafca5bf7d3704b081fde7df54f298736ee059bf6d32fac25f5e6085bf6";
        let usd = rpc::token_to_usd(token).await.expect("Can't get usd price");

        println!("usd = {:#?}", usd);
    }

    #[tokio::test]
    async fn prices() {
        let db_url = env::var("DATABASE_URL").unwrap();

        let pool = crate::init_pg_pool(&db_url, 5)
            .await
            .expect("Postgres connection failed");

        let mut tx = pool.begin().await.unwrap();

        let nft = "0:b7c55b3f9f82a68e7f6121b49ee58709737f1e38e96288b2531e9a5dc61822e0";
        let collection = crate::actions::get_collection_by_nft(&nft.into(), &mut tx)
            .await
            .unwrap();
        let prices = crate::actions::get_prices(&collection.into(), &mut tx).await;

        println!("prices: {:#?}", prices.expect("Can't get prices"));
    }
}
