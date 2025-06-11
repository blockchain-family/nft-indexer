use anyhow::Result;
use bigdecimal::num_bigint::Sign;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime};
use serde::Serializer;
use ton_block::{GetRepresentationHash, MsgAddressInt};
use ton_types::UInt256;

pub trait KeyInfo {
    fn get_account(&self) -> String;
    fn _get_hash(&self) -> Result<Vec<u8>>;
    fn get_timestamp(&self) -> i64;
}

impl KeyInfo for ton_block::Transaction {
    fn get_account(&self) -> String {
        self.in_msg
            .as_ref()
            .and_then(|m| {
                m.read_struct()
                    .ok()
                    .and_then(|s| s.dst_ref().map(|d| d.to_string()))
            })
            .unwrap_or_else(|| format!("0:{}", self.account_addr.to_hex_string()))
    }

    fn _get_hash(&self) -> Result<Vec<u8>> {
        Ok(self.hash()?.into_vec())
    }

    fn get_timestamp(&self) -> i64 {
        self.now.into()
    }
}

pub struct DecodeContext {
    pub tx_data: ton_block::Transaction,
    pub _function_inputs: Vec<ton_abi::Token>,
    pub message_hash: UInt256,
}

pub fn serialize_msg_address_int<S>(addr: &MsgAddressInt, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&addr.to_string())
}

pub fn serialize_uint256<S>(v: &UInt256, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(u256_to_bigdecimal(v).to_string().as_str())
}

pub fn u128_to_bigdecimal(i: u128) -> BigDecimal {
    BigDecimal::new(bigdecimal::num_bigint::BigInt::from(i), 0)
}

pub fn u256_to_bigdecimal(i: &UInt256) -> BigDecimal {
    BigDecimal::new(
        bigdecimal::num_bigint::BigInt::from_bytes_be(Sign::Plus, i.as_slice()),
        0,
    )
}

pub fn timestamp_to_datetime(ts: i64) -> NaiveDateTime {
    DateTime::from_timestamp(ts, 0)
        .unwrap_or_default()
        .naive_utc()
}
