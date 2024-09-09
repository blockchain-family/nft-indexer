use nekoton_abi::{BuildTokenValue, PackAbi, UnpackAbi, UnpackerError};
use num::BigUint;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::Serialize;
use ton_abi::TokenValue;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

use crate::utils::{serialize_msg_address_int, serialize_uint256};

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct MarketOffer {
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(address)]
    pub collection: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(name = "nftOwner", address)]
    pub nft_owner: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(address)]
    pub nft: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(address)]
    pub offer: MsgAddressInt,
    #[abi(uint128)]
    pub price: u128,
    #[abi(name = "auctionDuration", uint128)]
    pub auction_duration: u128,
    #[abi(name = "deployNonce", uint64)]
    pub deploy_nonce: u64,
}

#[derive(FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Debug, Serialize)]
pub enum AuctionStatus {
    Created = 1,
    Active,
    Complete,
    Cancelled,
}

impl UnpackAbi<AuctionStatus> for TokenValue {
    fn unpack(self) -> nekoton_abi::UnpackerResult<AuctionStatus> {
        UnpackAbi::<u8>::unpack(self)
            .map(FromPrimitive::from_u8)
            .transpose()
            .ok_or(UnpackerError::InvalidAbi)?
    }
}

impl BuildTokenValue for AuctionStatus {
    fn token_value(self) -> TokenValue {
        TokenValue::Uint(ton_abi::Uint {
            number: BigUint::from_u8(num::ToPrimitive::to_u8(&self).unwrap()).unwrap(),
            size: 8,
        })
    }
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct AuctionDetails {
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(name = "auctionSubject", address)]
    pub auction_subject: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(name = "subjectOwner", address)]
    pub subject_owner: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(name = "_paymentToken", address)]
    pub payment_token: MsgAddressInt,
    #[serde(serialize_with = "serialize_msg_address_int")]
    #[abi(name = "walletForBids", address)]
    pub wallet_for_bids: MsgAddressInt,
    #[abi(name = "startTime", uint64)]
    pub start_time: u64,
    #[abi(uint64)]
    pub duration: u64,
    #[abi(name = "finishTime", uint64)]
    pub finish_time: u64,
    #[abi(name = "_price", uint128)]
    pub price: u128,
    #[abi(name = "_nonce", uint64)]
    pub nonce: u64,
    #[abi(uint8)]
    pub status: u8,
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct DirectBuyInfo {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub factory: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub creator: MsgAddressInt,
    #[abi(name = "spentToken")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub spent_token: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi(name = "_timeTx")]
    pub _time_tx: u64,
    #[abi]
    pub _price: u128,
    #[abi(name = "spentWallet")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub spent_wallet: MsgAddressInt,
    #[abi]
    pub status: u8,
    #[abi(name = "startTimeBuy")]
    pub start_time_buy: u64,
    #[abi(name = "durationTimeBuy")]
    pub duration_time_buy: u64,
    #[abi(name = "endTimeBuy")]
    pub end_time_buy: u64,
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct DirectSellInfo {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub factory: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub creator: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub token: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi(name = "_timeTx")]
    pub _time_tx: u64,
    #[abi]
    pub start: u64,
    #[abi]
    pub end: u64,
    #[abi]
    pub _price: u128,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub wallet: MsgAddressInt,
    #[abi]
    pub status: u8,
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct MarketFee {
    #[abi]
    pub numerator: u32,
    #[abi]
    pub denominator: u32,
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct CollectionFeeInfo {
    #[abi(name = "codeHash")]
    #[serde(serialize_with = "serialize_uint256")]
    pub code_hash: UInt256,
    #[abi(name = "codeDepth")]
    pub code_depth: u16,
    #[abi]
    pub numerator: u32,
    #[abi]
    pub denominator: u32,
}
