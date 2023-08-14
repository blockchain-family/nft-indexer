use anyhow::Result;
use sqlx::PgPool;

use crate::utils::EventMessageInfo;

pub use self::types::Decoded;

mod auction_root_tip3;
mod auction_tip3;
mod collection;
mod common;
mod direct_buy;
mod direct_sell;
mod factory_direct_buy;
mod factory_direct_sell;
mod nft;
mod types;

#[async_trait::async_trait]
pub trait Entity: Sync + Send {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()>;
}

pub trait Decode {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded>;
    fn decode_event(&self, msg_info: &EventMessageInfo) -> Result<Decoded>;
}
