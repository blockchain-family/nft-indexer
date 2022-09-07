use crate::database::records::*;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, PgPool};

// pub async fn get_limit_order_state_changed_records(
//     pool: &PgPool,
//     request: AuctionDeployedRequest,
// ) -> Result<(Vec<AuctionDeployedRecord>, i64)> {
//     todo!()
// }

#[async_trait]
impl Put for NftMetadataRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into nft_metadata (nft, meta)
            values ($1, $2)
            "#,
            self.nft,
            self.data,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionCreatedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_created (account_addr, created_lt, created_at, auction_subject, subject_owner,
                payment_token_root, wallet_for_bids, start_time, duration, finish_time, now_time)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.auction_subject,
            self.subject_owner,
            self.payment_token_root,
            self.wallet_for_bids,
            self.start_time,
            self.duration,
            self.finish_time,
            self.now_time,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionActiveRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_active (account_addr, created_lt, created_at, auction_subject, subject_owner,
                payment_token_root, wallet_for_bids, start_time, duration, finish_time, now_time)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.auction_subject,
            self.subject_owner,
            self.payment_token_root,
            self.wallet_for_bids,
            self.start_time,
            self.duration,
            self.finish_time,
            self.now_time,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionDeployedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_deployed (account_addr, created_lt, created_at, offer_address, collection,
                nft_owner, nft, offer, price, auction_duration, deploy_nonce)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.offer_address,
            self.collection,
            self.nft_owner,
            self.nft,
            self.offer,
            self.price,
            self.auction_duration,
            self.deploy_nonce,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionDeclinedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_declined (account_addr, created_lt, created_at, nft_owner, data_address)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.nft_owner,
            self.data_address,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionOwnershipTransferredRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_ownership_transferred (account_addr, created_lt, created_at, old_owner, new_owner)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.old_owner,
            self.new_owner,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for BidPlacedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into bid_placed (account_addr, created_lt, created_at, buyer_address, value)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.buyer_address,
            self.value,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for BidDeclinedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into bid_declined (account_addr, created_lt, created_at, buyer_address, value)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.buyer_address,
            self.value,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionCompleteRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into auction_complete (account_addr, created_lt, created_at, buyer_address, value)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.buyer_address,
            self.value,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for AuctionCancelledRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into bid_declined (account_addr, created_lt, created_at)
            values ($1, $2, $3)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectBuyDeployedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_buy_deployed (account_addr, created_lt, created_at, direct_buy_address, sender,
                token_root, nft, nonce, amount)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.direct_buy_address,
            self.sender,
            self.token_root,
            self.nft,
            self.nonce,
            self.amount,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectBuyDeclinedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_buy_declined (account_addr, created_lt, created_at, sender, token_root, amount)
            values ($1, $2, $3, $4, $5, $6)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.sender,
            self.token_root,
            self.amount,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectBuyOwnershipTransferredRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_buy_ownership_transferred (account_addr, created_lt, created_at, old_owner, new_owner)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.old_owner,
            self.new_owner,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectSellDeployedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_sell_deployed (account_addr, created_lt, created_at, _direct_sell_address, sender,
                payment_token, nft, _nonce, price)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self._direct_sell_address,
            self.sender,
            self.payment_token,
            self.nft,
            self._nonce,
            self.price,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectSellDeclinedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_sell_declined (account_addr, created_lt, created_at, sender, _nft_address)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.sender,
            self._nft_address,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectSellOwnershipTransferredRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_sell_ownership_transferred (account_addr, created_lt, created_at, old_owner, new_owner)
            values ($1, $2, $3, $4, $5)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.old_owner,
            self.new_owner,
        )
        .execute(pool)
        .await?)
    }
}
#[async_trait]
impl Put for DirectBuyStateChangedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_buy_state_changed (account_addr, created_lt, created_at, from_state, to_state, factory,
                creator, spent_token, nft, _time_tx, _price, spent_wallet, status, sender, start_time_buy,
                duration_time_buy, end_time_buy)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.from,
            self.to,
            self.factory,
            self.creator,
            self.spent_token,
            self.nft,
            self._time_tx,
            self._price,
            self.spent_wallet,
            self.status,
            self.sender,
            self.start_time_buy,
            self.duration_time_buy,
            self.end_time_buy,
        )
        .execute(pool)
        .await?)
    }
}

#[async_trait]
impl Put for DirectSellStateChangedRecord {
    async fn put_record(&self, pool: &PgPool) -> Result<PgQueryResult>
    where
        Self: Sync,
    {
        Ok(sqlx::query!(
            r#"
            insert into direct_sell_state_changed (account_addr, created_lt, created_at, from_state, to_state, factory,
                creator, token, nft, _time_tx, start_time, end_time, _price, wallet, status, sender)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            self.account_addr,
            self.created_lt,
            self.created_at,
            self.from,
            self.to,
            self.factory,
            self.creator,
            self.token,
            self.nft,
            self._time_tx,
            self.start,
            self.end,
            self._price,
            self.wallet,
            self.status,
            self.sender,
        )
        .execute(pool)
        .await?)
    }
}
