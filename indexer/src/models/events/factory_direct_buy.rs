use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::utils::serialize_msg_address_int;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectBuyDeployed {
    #[abi(name = "directBuy")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub direct_buy: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub sender: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub token: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi]
    pub nonce: u64,
    #[abi]
    pub amount: u128,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectBuyDeclined {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub sender: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub token: MsgAddressInt,
    #[abi]
    pub amount: u128,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
}
