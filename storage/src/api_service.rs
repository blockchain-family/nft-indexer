use sqlx::{self, postgres::PgPool};
use std::{sync::Arc};
use crate::tables::*;


#[derive(Debug, Clone)]
pub struct ApiService {
    db: Arc<PgPool>,
}

impl ApiService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn get_nft(&self, address: &String) -> sqlx::Result<Option<NFT>> {
        sqlx::query_as!(NFT, "SELECT * FROM nft WHERE nft.address = $1", address)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn get_nft_details(&self, address: &String) -> sqlx::Result<Option<NFTDetails>> {
        sqlx::query_as!(NFTDetails, "
        SELECT n.*
        FROM nft_details n 
        WHERE n.address = $1
        ", address)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn get_nft_meta(&self, address: &String) -> sqlx::Result<Option<NFTMetadata>> {
        sqlx::query_as!(NFTMetadata, "SELECT * FROM nft_metadata WHERE nft_metadata.nft = $1", address)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn get_collection(&self, address: &String) -> sqlx::Result<Option<NFTCollection>> {
        sqlx::query_as!(NFTCollection, "SELECT * FROM nft_collection WHERE address = $1", address)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn list_collections_by_owner(&self, owner: &String) -> sqlx::Result<Vec<NFTCollection>> {
        sqlx::query_as!(NFTCollection, "SELECT * FROM nft_collection WHERE owner = $1", owner)
            .fetch_all(self.db.as_ref())
            .await
    }

    pub async fn nft_search(&self,
        owners: &[Address],
        collections: &[Address],
        _price_from: Option<u64>,
        _price_to: Option<u64>,
        _price_token: Option<Address>,
        _forsale: Option<bool>,
        _auction: Option<bool>,
    ) -> sqlx::Result<Vec<NFTDetails>> {
        sqlx::query_as!(NFTDetails, "
        SELECT *
        FROM nft_details
        WHERE
        (owner = ANY($1) OR array_length($1::varchar[], 1) = 0)
        and (collection = ANY($2) OR array_length($2::varchar[], 1) = 0)
        ", owners, collections)
            .fetch_all(self.db.as_ref())
            .await
    }

    pub async fn list_events(&self,
        nft: Option<String>,
        collection: Option<String>,
        typ: Option<EventType>,
        offset: usize,
        limit: usize,
    ) -> sqlx::Result<Vec<Event>> {
        sqlx::query_as!(Event, "
        SELECT e.id, e.address, e.created_at, e.created_lt, e.args,
        e.event_cat as \"event_cat: _\",
        e.event_type as \"event_type: _\"
        FROM nft_events e
        WHERE
            ($3::varchar is null OR e.nft = $3)
            AND ($4::varchar is null OR e.collection = $4)
            AND ($5::event_type is null OR e.event_type = $5)
        LIMIT $1 OFFSET $2
        ", limit as i64, offset as i64,
            nft, collection, typ as _)
            .fetch_all(self.db.as_ref())
            .await
    }
}

