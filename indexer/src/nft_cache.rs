use anyhow::Result;
use indexer_repo::types::AuctionCachedInfo;
use moka::sync::{Cache, CacheBuilder};
use sqlx::PgPool;

pub struct NftCacheService {
    pg_pool: PgPool,
    nft_cache: Cache<String, String>,
    auc_cache: Cache<String, AuctionCachedInfo>,
}

impl NftCacheService {
    pub fn new(pg_pool: PgPool) -> Self {
        let nft_cache = CacheBuilder::new(1_000_000).build();
        let auc_cache = CacheBuilder::new(1_000_000).build();

        Self {
            pg_pool,
            nft_cache,
            auc_cache,
        }
    }

    pub fn add_collection_of_nft<SN: AsRef<str>, SC: AsRef<str>>(&self, nft: SN, collection: SC) {
        self.nft_cache
            .insert(nft.as_ref().to_string(), collection.as_ref().to_string());
    }

    pub async fn get_collection_of_nft<S: AsRef<str>>(&self, nft: S) -> Result<Option<String>> {
        if let Some(collection) = self.nft_cache.get(nft.as_ref()) {
            return Ok(Some(collection));
        }

        let collection =
            indexer_repo::cached::get_collection_of_nft(&self.pg_pool, nft.as_ref()).await?;

        if let Some(collection) = collection.as_ref() {
            self.nft_cache
                .insert(nft.as_ref().to_string(), collection.clone());
        }

        Ok(collection)
    }

    pub fn add_auction_cached_info<S: AsRef<str>>(&self, auction: S, info: AuctionCachedInfo) {
        self.auc_cache.insert(auction.as_ref().to_string(), info);
    }

    pub async fn get_auction_cached_info<S: AsRef<str>>(
        &self,
        auction: S,
    ) -> Result<Option<AuctionCachedInfo>> {
        if let Some(auction) = self.auc_cache.get(auction.as_ref()) {
            return Ok(Some(auction));
        }

        let info =
            indexer_repo::cached::get_auction_cached_info(&self.pg_pool, auction.as_ref()).await?;

        if let Some(info) = info.as_ref() {
            self.auc_cache
                .insert(auction.as_ref().to_string(), info.clone());
        }

        Ok(info)
    }
}
