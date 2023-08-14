use anyhow::{anyhow, Result};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CollectionsQueue {
    pg_pool: PgPool,
    tx: Sender<(String, i64)>,
}

const COLLECTIONS_CACHE_SIZE: i64 = 1_000;
const CHANNEL_BUF_SIZE: usize = 1_000_000;

impl CollectionsQueue {
    pub(in crate::persistence::collections_queue) async fn new(
        pg_pool: PgPool,
    ) -> (Arc<Self>, Receiver<(String, i64)>) {
        let (tx, rx) = mpsc::channel(CHANNEL_BUF_SIZE);

        (Arc::new(Self { pg_pool, tx }), rx)
    }

    pub async fn add(&self, collection: String, nft_mint_ts: i64) -> Result<()> {
        self.tx
            .send((collection, nft_mint_ts))
            .await
            .map_err(|e| anyhow!(e))
    }

    async fn run(&self, mut rx: Receiver<(String, i64)>) {
        let now = chrono::Utc::now().timestamp();

        let mut collections = HashMap::<String, i64>::from_iter(
            indexer_repo::actions::get_collections(COLLECTIONS_CACHE_SIZE, &self.pg_pool)
                .await
                .expect("Can't pull collections from DB")
                .into_iter()
                .map(|c| (c, now)),
        );

        while let Some((collection, nft_mint_ts)) = rx.recv().await {
            if let Some(last_used) = collections.get_mut(&collection) {
                *last_used = chrono::Utc::now().timestamp();
            } else if let Err(e) = indexer_repo::actions::insert_collection(
                collection.clone().into(),
                chrono::NaiveDateTime::from_timestamp_opt(nft_mint_ts, 0).unwrap_or_default(),
                &self.pg_pool,
            )
            .await
            {
                log::error!("Error adding collection to DB: {:#?}", e);
            } else {
                if collections.len() == COLLECTIONS_CACHE_SIZE as usize {
                    let (key, _) = collections.iter().min().unwrap();
                    collections.remove_entry(&key.clone());
                }

                collections.insert(collection, chrono::Utc::now().timestamp());
            }
        }
    }
}

pub async fn create_and_run_queue(pg_pool: PgPool) -> Arc<CollectionsQueue> {
    let (q, rx) = CollectionsQueue::new(pg_pool).await;

    {
        let q = q.clone();
        tokio::spawn(async move { q.run(rx).await });
    }

    q
}
