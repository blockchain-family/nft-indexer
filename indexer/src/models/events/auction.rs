use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::models::types::*;
use crate::utils::serialize_msg_address_int;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionCreated {
    #[abi]
    pub value0: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionActive {
    #[abi]
    pub value0: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct BidPlaced {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub buyer: MsgAddressInt,
    #[abi]
    pub value: u128,
    #[abi(name = "nextBidValue")]
    pub next_bid_value: u128,
    #[abi]
    pub value3: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct BidDeclined {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub buyer: MsgAddressInt,
    #[abi]
    pub value: u128,
    #[abi]
    pub value2: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionComplete {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub buyer: MsgAddressInt,
    #[abi]
    pub value: u128,
    #[abi]
    pub value2: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionCancelled {
    #[abi]
    pub value0: AuctionDetails,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct RoyaltySet {
    #[abi]
    pub _royalty: Royalty,
}
