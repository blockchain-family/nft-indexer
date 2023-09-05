use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::{models::types::*, utils::serialize_msg_address_int};

/*
   FactoryAuction,
   Collection,
   FactoryDirectBuy,
   FactoryDirectSell,
   MintAndSell,
*/
#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct OwnershipTransferred {
    #[abi(name = "oldOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub old_owner: MsgAddressInt,
    #[abi(name = "newOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub new_owner: MsgAddressInt,
}

/*
   FactoryAuction,
   FactoryDirectBuy,
   FactoryDirectSell,
*/
#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct MarketFeeDefaultChanged {
    #[abi]
    pub fee: MarketFee,
}

/*
   FactoryAuction,
   Auction,
   DirectBuy,
   DirectSell,
   FactoryDirectBuy,
   FactoryDirectSell,
*/
#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct MarketFeeChanged {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub auction: MsgAddressInt,
    #[abi]
    pub fee: MarketFee,
}

/*
   FactoryAuction,
   FactoryDirectBuy,
   FactoryDirectSell,
*/
#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AddCollectionRules {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
    #[abi(name = "collectionFeeInfo")]
    pub collection_fee_info: CollectionFeeInfo,
}

/*
   FactoryAuction,
   FactoryDirectBuy,
   FactoryDirectSell,
*/
#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct RemoveCollectionRules {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
}
