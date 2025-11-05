use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sqlx::types::BigDecimal;
use sqlx::PgPool;

use crate::types::BcName;

#[derive(Clone)]
pub struct NftPriceModel {
    pool: PgPool,
}

pub struct RowWithoutUsdPrice {
    pub id: String,
    pub token_addr: String,
    pub token_amount: BigDecimal,
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

    pub async fn get_offers_without_price_usd(
        &self,
        limit: i64,
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
                    created_at: value.created_at.and_utc().timestamp(),
                }
            }
        }

        let now = Local::now().naive_local();
        let zero_time = NaiveDateTime::default();

        sqlx::query_as!(
            Row,
            r#"
                select
                    source as id,
                    price_token as "token_addr!",
                    price as "token_amount!",
                    ts as "created_at!"
                from nft_price_history
                where usd_price is null
                and ts <= $1
                and ts != $2
                limit $3
            "#,
            now,
            zero_time,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map(|v| v.into_iter().map(|r| r.into()).collect())
        .map_err(|e| anyhow!(e))
    }

    pub async fn update_usd_price(&self, id: &str, price: &BigDecimal) -> Result<()> {
        sqlx::query!(
            r#"
                update nft_price_history
                set usd_price = $1
                where source = $2
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
            BcName::Tycho => self.get_pair_address(token_addr, BcName::Tycho).await,
        }
    }

    pub async fn get_tokens_with_dex_pair(&self, source: BcName) -> Result<Vec<String>> {
        sqlx::query_scalar!(
            r#"
                select token
                from token_to_dex
                where source = $1
            "#,
            source as _
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!(e))
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
