use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sqlx::{types::BigDecimal, PgPool};

use crate::types::BcName;

pub struct NftPriceModel {
    pool: PgPool,
}

pub enum RowWithoutUsdPriceSource {
    DirSell,
    DirBuy,
    Auc,
}

pub struct RowWithoutUsdPrice {
    pub id: String,
    pub token_addr: String,
    pub token_amount: BigDecimal,
    pub source: RowWithoutUsdPriceSource,
    pub created_at: i64,
}

pub struct DexPoolInfo {
    pub address: String,
    pub is_l2r: bool,
    pub decimals: i32,
}

impl NftPriceModel {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_auction_without_price_usd(
        &self,
        elements_number: i64,
    ) -> Result<Vec<RowWithoutUsdPrice>> {
        struct Row {
            id: String,
            token_addr: String,
            token_amount: BigDecimal,
            created_at: NaiveDateTime,
        }
        impl From<Row> for RowWithoutUsdPrice {
            fn from(value: Row) -> Self {
                Self {
                    token_addr: value.token_addr,
                    id: value.id,
                    token_amount: value.token_amount,
                    source: RowWithoutUsdPriceSource::Auc,
                    created_at: value.created_at.timestamp(),
                }
            }
        }

        let now = Local::now().naive_local();
        let zero_time = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();

        // INFO: It looks like there is a mistake with price parsing. The 'min_bid' column always greater
        // than the 'max_bid'. Until this issue is clarified, I'll be using min_bid.
        sqlx::query_as!(
            Row,
            r#"
                select 
                    address as id,
                    price_token as "token_addr!",
                    min_bid as "token_amount!",
                    created_at as "created_at!"
                from nft_auction
                where closing_price_usd is null
                and created_at is not null
                and min_bid is not null
                and price_token is not null
                and created_at <= $1
                and created_at != $2 
                and status = 'completed'
                limit $3
            "#,
            now,
            zero_time,
            elements_number
        )
        .fetch_all(&self.pool)
        .await
        .map(|v| v.into_iter().map(|r| r.into()).collect())
        .map_err(|e| anyhow!(e))
    }

    pub async fn get_direct_sell_without_price_usd(
        &self,
        elements_number: i64,
    ) -> Result<Vec<RowWithoutUsdPrice>> {
        struct Row {
            id: String,
            token_addr: String,
            token_amount: BigDecimal,
            created_at: NaiveDateTime,
        }
        impl From<Row> for RowWithoutUsdPrice {
            fn from(value: Row) -> Self {
                Self {
                    token_addr: value.token_addr,
                    id: value.id,
                    token_amount: value.token_amount,
                    source: RowWithoutUsdPriceSource::DirSell,
                    created_at: value.created_at.timestamp(),
                }
            }
        }

        let now = Local::now().naive_local();
        let zero_time = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();

        sqlx::query_as!(
            Row,
            r#"
                select 
                    address as id,
                    price_token as token_addr,
                    price as token_amount,
                    created as created_at
                from nft_direct_sell
                where sell_price_usd is null
                and created <= $1
                and created != $2 
                and state = 'filled'
                limit $3
            "#,
            now,
            zero_time,
            elements_number
        )
        .fetch_all(&self.pool)
        .await
        .map(|v| v.into_iter().map(|r| r.into()).collect())
        .map_err(|e| anyhow!(e))
    }

    pub async fn get_direct_buy_without_price_usd(
        &self,
        elements_number: i64,
    ) -> Result<Vec<RowWithoutUsdPrice>> {
        struct Row {
            id: String,
            token_addr: String,
            token_amount: BigDecimal,
            created_at: NaiveDateTime,
        }
        impl From<Row> for RowWithoutUsdPrice {
            fn from(value: Row) -> Self {
                Self {
                    token_addr: value.token_addr,
                    id: value.id,
                    token_amount: value.token_amount,
                    source: RowWithoutUsdPriceSource::DirBuy,
                    created_at: value.created_at.timestamp(),
                }
            }
        }

        let now = Local::now().naive_local();
        let zero_time = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();

        sqlx::query_as!(
            Row,
            r#"
                select 
                    address as id,
                    price_token as token_addr,
                    price as token_amount,
                    created as created_at
                from nft_direct_buy
                where buy_price_usd is null
                and created <= $1
                and created != $2 
                and state = 'filled'
                limit $3
            "#,
            now,
            zero_time,
            elements_number
        )
        .fetch_all(&self.pool)
        .await
        .map(|v| v.into_iter().map(|r| r.into()).collect())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_usd_price(
        &self,
        id: &str,
        price: &BigDecimal,
        source: &RowWithoutUsdPriceSource,
    ) -> Result<()> {
        match source {
            RowWithoutUsdPriceSource::DirSell => self.update_direct_sell_usd_price(id, price).await,
            RowWithoutUsdPriceSource::DirBuy => self.update_direct_buy_usd_price(id, price).await,
            RowWithoutUsdPriceSource::Auc => self.update_auction_usd_price(id, price).await,
        }
    }

    async fn update_auction_usd_price(&self, id: &str, price: &BigDecimal) -> Result<()> {
        sqlx::query!(
            r#"
                update nft_auction
                set closing_price_usd = $1
                where address = $2
            "#,
            price,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
        .map(|_| ())
    }

    async fn update_direct_buy_usd_price(&self, id: &str, price: &BigDecimal) -> Result<()> {
        sqlx::query!(
            r#"
                update nft_direct_buy
                set buy_price_usd = $1
                where address = $2
            "#,
            price,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
        .map(|_| ())
    }

    async fn update_direct_sell_usd_price(&self, id: &str, price: &BigDecimal) -> Result<()> {
        sqlx::query!(
            r#"
                update nft_direct_sell
                set sell_price_usd = $1
                where address = $2
            "#,
            price,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
        .map(|_| ())
    }

    pub async fn get_dex_pair_address(&self, token_addr: &str, bc: BcName) -> Result<DexPoolInfo> {
        match bc {
            BcName::Everscale => self.get_pair_address(token_addr, BcName::Everscale).await,
            BcName::Venom => self.get_pair_address(token_addr, BcName::Venom).await,
        }
    }

    async fn get_pair_address(&self, token_addr: &str, source: BcName) -> Result<DexPoolInfo> {
        sqlx::query_as!(
            DexPoolInfo,
            r#"
                select 
                    pair as address,
                    is_l2r,
                    decimals
                from token_to_dex
                where token = $1 and source = $2
            "#,
            token_addr,
            source as _
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
    }
}
