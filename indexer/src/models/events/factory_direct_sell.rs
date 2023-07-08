use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::utils::serialize_msg_address_int;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectSellDeployed {
    #[abi(name = "directSell")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub direct_sell: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub sender: MsgAddressInt,
    #[abi(name = "paymentToken")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub payment_token: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi]
    pub nonce: u64,
    #[abi]
    pub price: u128,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectSellDeclined {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub sender: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
}
