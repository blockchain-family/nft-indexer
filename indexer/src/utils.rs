use anyhow::Result;
use bigdecimal::BigDecimal;
use num::BigInt;
use serde::Serializer;
use ton_block::{GetRepresentationHash, MsgAddressInt};
use ton_types::UInt256;

pub trait KeyInfo {
    fn get_account(&self) -> String;
    fn get_hash(&self) -> Result<Vec<u8>>;
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

    fn get_hash(&self) -> Result<Vec<u8>> {
        Ok(self.hash()?.into_vec())
    }

    fn get_timestamp(&self) -> i64 {
        self.now as i64
    }
}

pub fn _get_default_address() -> String {
    MsgAddressInt::default().to_string()
}

pub struct EventMessageInfo {
    pub tx_data: ton_block::Transaction,
    pub function_inputs: Vec<ton_abi::Token>,
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
    s.serialize_str(&v.to_string())
}

pub fn u128_to_bigdecimal(i: u128) -> BigDecimal {
    BigDecimal::new(BigInt::from(i), 0)
}
