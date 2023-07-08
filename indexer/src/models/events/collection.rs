use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

use crate::utils::{serialize_msg_address_int, serialize_uint256};

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct NftCreated {
    #[abi]
    #[serde(serialize_with = "serialize_uint256")]
    pub id: UInt256,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub owner: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub manager: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub creator: MsgAddressInt,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct NftBurned {
    #[abi]
    #[serde(serialize_with = "serialize_uint256")]
    pub id: UInt256,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub owner: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub manager: MsgAddressInt,
}
