use anyhow::Result;
use indexer_repo::batch::save_collections;
use indexer_repo::types::NftCollection;
use sqlx::PgPool;
use std::collections::HashMap;

pub struct CollectionsQueue {
    pg_pool: PgPool,
    collections: HashMap<String, i64>,
}

const COLLECTIONS_CACHE_SIZE: i64 = 1_000;

impl CollectionsQueue {
    pub async fn new(pg_pool: PgPool) -> Self {
        let now = chrono::Utc::now().timestamp();

        let collections = HashMap::<String, i64>::from_iter(
            indexer_repo::collection::get_collections(&pg_pool, COLLECTIONS_CACHE_SIZE)
                .await
                .expect("Failed fetching collections from DB")
                .into_iter()
                .map(|c| (c, now)),
        );

        Self {
            pg_pool,
            collections,
        }
    }

    pub async fn add_collections(&mut self, new_collections: Vec<NftCollection>) -> Result<()> {
        let mut to_insert = Vec::with_capacity(new_collections.len());
        for collection in new_collections {
            if let Some(last_used) = self.collections.get_mut(&collection.address) {
                *last_used = chrono::Utc::now().timestamp();
            } else {
                if self.collections.len() == COLLECTIONS_CACHE_SIZE as usize {
                    let (key, _) = self.collections.iter().min().unwrap();
                    self.collections.remove_entry(&key.clone());
                }

                self.collections
                    .insert(collection.address.clone(), chrono::Utc::now().timestamp());

                to_insert.push(collection);
            }
        }

        save_collections(&self.pg_pool, &to_insert).await?;

        Ok(())
    }
}
