use nekoton_abi::{KnownParamTypePlain, PackAbi, PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::utils::serialize_msg_address_int;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct OwnerChanged {
    #[abi(name = "oldOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub old_owner: MsgAddressInt,
    #[abi(name = "newOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub new_owner: MsgAddressInt,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct ManagerChanged {
    #[abi(name = "oldManager")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub old_manager: MsgAddressInt,
    #[abi(name = "newManager")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub new_manager: MsgAddressInt,
}

#[derive(Debug, Clone, PackAbi, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain, Serialize)]
pub struct NftMetadataUpdated;

#[derive(Debug, Clone, PackAbi, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain, Serialize)]
pub struct CollectionMetadataUpdated;

#[derive(Debug, Clone, PackAbi, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain, Serialize)]
pub struct MetadataUpdated;
