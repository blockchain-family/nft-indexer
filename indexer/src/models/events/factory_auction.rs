use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;
use ton_block::MsgAddressInt;

use crate::models::types::*;
use crate::utils::serialize_msg_address_int;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionDeployed {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub offer: MsgAddressInt,
    #[abi(name = "offerInfo")]
    pub offer_info: MarketOffer,
}

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct AuctionDeclined {
    #[abi(name = "nftOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft_owner: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
}
