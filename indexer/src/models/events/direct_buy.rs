use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::{models::types::*, utils::serialize_msg_address_int};

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectBuyStateChanged {
    #[abi]
    pub from: u8,
    #[abi]
    pub to: u8,
    #[abi]
    pub value2: DirectBuyInfo,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub old_owner: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub new_owner: MsgAddressInt,
}
