use crate::{event_records::*, nft_records::*, traits::DatabaseRecord, types::Address};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, PgPool};

#[async_trait]
impl DatabaseRecord for AuctionCreated {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_created', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionActive {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_active', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionDeployed {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_deployed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionDeclined {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_declined', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionOwnershipTransferred {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_ownership_transferred', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for BidPlaced {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_bid_placed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for BidDeclined {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_bid_declined', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionComplete {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_complete', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for AuctionCancelled {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('auction', 'auction_cancelled', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectBuyDeployed {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_buy', 'direct_buy_deployed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectBuyDeclined {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_buy', 'direct_buy_declined', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectBuyOwnershipTransferred {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_buy', 'direct_buy_ownership_transferred', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectSellDeployed {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_sell', 'direct_sell_deployed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectSellDeclined {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_sell', 'direct_sell_declined', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectSellOwnershipTransferred {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_sell', 'direct_sell_ownership_transferred', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}
#[async_trait]
impl DatabaseRecord for DirectBuyStateChanged {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_buy', 'direct_buy_state_changed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for DirectSellStateChanged {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        // TODO: upsert
        Ok(sqlx::query!(
            r#"
            insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
            values ('direct_sell', 'direct_sell_state_changed', $1, $2, $3, $4)
            "#,
            &self.address as &Address,
            self.created_lt,
            self.created_at,
            serde_json::to_value(&self)?,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for Nft {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        // TODO: upsert
        sqlx::query!(
            r#"
            insert into nft (address, collection, owner, manager, name, description, updated)
            values ($1, $2, $3, $4, $5, $6, $7)
            "#,
            &self.address as &Address,
            &self.collection as &Address,
            &self.owner as &Address,
            &self.manager as &Address,
            self.name,
            self.description,
            self.updated,
        )
        .execute(pool)
        .await?;

        Ok(sqlx::query!(
            r#"
            insert into nft_metadata (nft, meta, ts)
            values ($1, $2, $3)
            "#,
            &self.address as &Address,
            self.data,
            self.updated,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl DatabaseRecord for NftCollection {
    async fn put_in(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_collection (address, owner, name, description, updated)
            values ($1, $2, $3, $4, $5)
            "#,
            &self.address as &Address,
            &self.owner as &Address,
            self.name,
            self.description,
            self.updated,
        )
        .execute(pool)
        .await?)
    }
}

pub async fn add_whitelist_address(address: &Address, pool: &PgPool) -> Result<PgQueryResult> {
    Ok(sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        "#,
        address as &Address
    )
    .execute(pool)
    .await?)
}
