use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use nekoton_abi::{transaction_parser::ExtractedOwned, BuildTokenValue};
use std::str::FromStr;
use storage::types::Address;
use ton_abi::{
    Token,
    TokenValue::{self, Uint as UintEnum},
};
use ton_block::CommonMsgInfo;

pub fn get_address(event: &ExtractedOwned) -> Address {
    ("0:".to_string() + &event.tx.account_id().as_hex_string()).into()
}

pub fn get_created_at(event: &ExtractedOwned) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_at.0 as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get created_at of event"))
}

pub fn get_created_lt(event: &ExtractedOwned) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_lt as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get created_lt of event"))
}

pub fn get_token_processor<'a, T>(
    tokens: &'a [Token],
    mapper_fn: impl Fn(&'a TokenValue) -> Option<T> + Clone + 'static,
) -> (impl Fn(&'a str) -> Result<T> + Clone + '_) {
    move |token_name| get_token_value(tokens, token_name, &mapper_fn)
}

pub fn token_to_big_decimal(token: &TokenValue) -> Option<BigDecimal> {
    match token {
        UintEnum(v) => Some(BigDecimal::from_str(&v.number.to_string()).unwrap_or_default()),
        _ => None,
    }
}

pub fn token_to_addr(token: &TokenValue) -> Option<Address> {
    match token.token_value() {
        ton_abi::TokenValue::Address(addr) => {
            Some(("0:".to_string() + &addr.get_address().as_hex_string()).into())
        }
        _ => None,
    }
}

pub fn token_to_i16(token: &TokenValue) -> Option<i16> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i16(),
        _ => None,
    }
}

pub fn token_to_i64(token: &TokenValue) -> Option<i64> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i64(),
        _ => None,
    }
}

pub fn get_token_value<'a, T>(
    tokens: &'a [Token],
    token_name: &'a str,
    mapper_fn: impl Fn(&'a TokenValue) -> Option<T>,
) -> Result<T> {
    let mut iter = tokens.iter();
    let token = iter
        .find(|t| t.name == token_name)
        .map(|t| &t.value)
        .ok_or_else(|| anyhow!("Token with name {} not found", token_name))?;
    mapper_fn(token).ok_or_else(|| anyhow!("Couldn't map token value: {:?}", token))
}
