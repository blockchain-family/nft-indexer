pub mod actions;
pub mod meta;
pub mod price;
pub mod types;

pub mod utils;

#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn collection_by_nft() {
        let db_url = env::var("DATABASE_URL").unwrap();

        let pool = crate::utils::init_pg_pool(&db_url, 5, Some(true))
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
}
