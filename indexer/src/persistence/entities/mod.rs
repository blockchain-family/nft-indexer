use anyhow::Result;
use ton_block::MsgAddressInt;

pub use self::types::Decoded;
use crate::utils::DecodeContext;

mod auction;
mod collection;
mod common;
mod direct_buy;
mod direct_sell;
mod factory_auction;
mod factory_direct_buy;
mod factory_direct_sell;
mod nft;
mod royalty;
mod types;

pub trait Decode {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded>;
    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded>;
}

fn to_address(addr: &MsgAddressInt) -> String {
    format!("{}:{}", addr.workchain_id(), addr.address().as_hex_string())
}
