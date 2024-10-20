use anyhow::Result;
use async_trait::async_trait;
use ton_block::MsgAddressInt;

use crate::utils::DecodeContext;

pub use self::types::Decoded;

mod auction;
mod collection;
mod common;
mod direct_buy;
mod direct_sell;
mod factory_auction;
mod factory_direct_buy;
mod factory_direct_sell;
mod nft;
mod types;

#[async_trait]
pub trait Decode {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded>;
    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded>;
}

fn to_address(addr: &MsgAddressInt) -> String {
    format!("{}:{}", addr.workchain_id(), addr.address().as_hex_string())
}
