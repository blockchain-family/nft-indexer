use nekoton_abi::{BuildTokenValue, PackAbi, UnpackAbi, UnpackerError};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::Serialize;
use ton_abi::TokenValue;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

use crate::utils::{serialize_msg_address_int, serialize_uint256};

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct MarketOffer {
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
    #[abi(name = "nftOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft_owner: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub nft: MsgAddressInt,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub offer: MsgAddressInt,
    #[abi]
    pub price: u128,
    #[abi(name = "auctionDuration")]
    pub auction_duration: u128,
    #[abi(name = "deployNonce")]
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
            .map(num_traits::FromPrimitive::from_u8)
            .transpose()
            .ok_or(UnpackerError::InvalidAbi)?
    }
}

impl BuildTokenValue for AuctionStatus {
    fn token_value(self) -> TokenValue {
        TokenValue::Uint(ton_abi::Uint {
            number: bigdecimal::num_bigint::BigUint::from_u8(self.to_u8().unwrap()).unwrap(),
            size: 8,
        })
    }
}

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct AuctionDetails {
    #[abi(name = "auctionSubject")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub auction_subject: MsgAddressInt,
    #[abi(name = "subjectOwner")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub subject_owner: MsgAddressInt,
    #[abi(name = "paymentToken")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub payment_token: MsgAddressInt,
    #[abi(name = "walletForBids")]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub wallet_for_bids: MsgAddressInt,
    #[abi(name = "startTime")]
    pub start_time: u64,
    #[abi]
    pub duration: u64,
    #[abi(name = "endTime")]
    pub end_time: u64,
    #[abi]
    pub price: u128,
    #[abi]
    pub nonce: u64,
    #[abi]
    pub status: AuctionStatus,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
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
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
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
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub collection: MsgAddressInt,
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

#[derive(UnpackAbi, PackAbi, PartialEq, Eq, Clone, Debug, Serialize)]
pub struct Royalty {
    #[abi]
    pub numerator: u32,
    #[abi]
    pub denominator: u32,
    #[abi]
    #[serde(serialize_with = "serialize_msg_address_int")]
    pub receiver: MsgAddressInt,
}
