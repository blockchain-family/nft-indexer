use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use nekoton_abi::{transaction_parser::Extracted, BuildTokenValue};
use std::str::FromStr;
use ton_abi::{
    Token,
    TokenValue::{self, Uint as UintEnum},
};
use ton_block::CommonMsgInfo;

// TODO: main contract addr?

// TODO: build_record...

fn get_account_addr(event: &Extracted) -> String {
    event.tx.account_id().to_hex_string()
}

fn get_created_at(event: &Extracted) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_at.0 as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get crated_at of event"))
}

fn get_created_lt(event: &Extracted) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_lt as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get crated_lt of event"))
}

fn get_token_processor<'a, T>(
    tokens: &'a [Token],
    mapper_fn: impl Fn(&'a TokenValue) -> Option<T> + Clone + 'static,
) -> (impl Fn(&'a str) -> Result<T> + Clone + '_) {
    move |token_name| get_token_value(tokens, token_name, &mapper_fn)
}

fn token_to_big_decimal(token: &TokenValue) -> Option<BigDecimal> {
    match token {
        UintEnum(v) => Some(BigDecimal::from_str(&v.number.to_string()).unwrap_or_default()),
        _ => None,
    }
}

fn token_to_str(token: &TokenValue) -> Option<String> {
    match token.token_value() {
        ton_abi::TokenValue::Address(addr) => Some(addr.get_address().as_hex_string()),
        _ => None,
    }
}

fn get_token_value<'a, T>(
    tokens: &'a [Token],
    token_name: &'a str,
    mapper_fn: impl Fn(&'a TokenValue) -> Option<T>,
) -> Result<T> {
    let mut iter = tokens.iter();
    let token = iter
        .find(|t| t.name == token_name)
        .map(|t| &t.value)
        .ok_or_else(|| anyhow!("token with name {} not found", token_name))?;
    mapper_fn(token).ok_or_else(|| anyhow!("couldn't map token value: {:?}", token))
}
