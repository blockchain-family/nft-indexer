use anyhow::Result;

use crate::utils::DecodeContext;

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

pub trait Decode {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded>;
    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded>;
}
