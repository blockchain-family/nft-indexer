use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use nekoton_abi::{transaction_parser::ExtractedOwned, BuildTokenValue};
use std::str::FromStr;
use ton_abi::{
    Token,
    TokenValue::{self, Tuple, Uint as UintEnum},
};
use ton_block::CommonMsgInfo;

use crate::database::records::{
    AuctionActiveRecord, AuctionDeclinedRecord, AuctionDeployedRecord, DirectSellStateChangedRecord,
};

pub fn build_direct_sell_state_changed_record(
    event: &ExtractedOwned,
) -> Result<DirectSellStateChangedRecord> {
    let from_token = event
        .tokens
        .iter()
        .find(|t| t.name == "from")
        .ok_or_else(|| anyhow!("Couldn't find from token"))?
        .clone();

    let to_token = event
        .tokens
        .iter()
        .find(|t| t.name == "to")
        .ok_or_else(|| anyhow!("Couldn't find to token"))?
        .clone();

    let value2_token = event
        .tokens
        .iter()
        .find(|t| t.name == "value2")
        .ok_or_else(|| anyhow!("Couldn't find value2 token"))?;
    let mut tokens = match &value2_token.value {
        Tuple(v) => Some(v.clone()),
        _ => None,
    }
    .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

    tokens.extend_from_slice(&[from_token, to_token]);

    let to_str = get_token_processor(&tokens, token_to_str);
    let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
    let to_i16 = get_token_processor(&tokens, token_to_i16);
    let to_i128 = get_token_processor(&tokens, token_to_i128);

    Ok(DirectSellStateChangedRecord {
        account_addr: get_account_addr(event),
        created_lt: get_created_lt(event)?,
        created_at: get_created_at(event)?,

        from: to_i16("from")?,
        to: to_i16("to")?,

        factory: to_str("factory")?,
        creator: to_str("creator")?,
        token: to_str("token")?,
        nft: to_str("nft")?,
        _time_tx: to_i128("_timeTx")?,
        start: to_i128("start")?,
        end: to_i128("end")?,
        _price: to_big_decimal("_price")?,
        wallet: to_str("wallet")?,
        status: to_i16("status")?,
        sender: to_str("sender")?,
    })
}

pub fn build_auction_active_record(event: &ExtractedOwned) -> Result<AuctionActiveRecord> {
    let value0_token = event
        .tokens
        .iter()
        .find(|t| t.name == "value0")
        .ok_or_else(|| anyhow!("Couldn't find value0 token"))?;
    let tokens = match &value0_token.value {
        Tuple(v) => Some(v),
        _ => None,
    }
    .ok_or_else(|| anyhow!("value2 token value is not tuple"))?;

    let to_str = get_token_processor(tokens, token_to_str);
    let to_i128 = get_token_processor(tokens, token_to_i128);

    Ok(AuctionActiveRecord {
        account_addr: get_account_addr(event),
        created_lt: get_created_lt(event)?,
        created_at: get_created_at(event)?,

        auction_subject: to_str("auctionSubject")?,
        subject_owner: to_str("subjectOwner")?,
        payment_token_root: to_str("paymentTokenRoot")?,
        wallet_for_bids: to_str("walletForBids")?,
        start_time: to_i128("startTime")?,
        duration: to_i128("duration")?,
        finish_time: to_i128("finishTime")?,
        now_time: to_i128("nowTime")?,
    })
}

pub fn build_auction_deployed_record(event: &ExtractedOwned) -> Result<AuctionDeployedRecord> {
    let offer_address_token = event
        .tokens
        .iter()
        .find(|t| t.name == "offerAddress")
        .ok_or_else(|| anyhow!("Couldn't find offerAddress token"))?
        .clone();

    let offer_info = event
        .tokens
        .iter()
        .find(|t| t.name == "offerInfo")
        .ok_or_else(|| anyhow!("Couldn't find offerInfo token"))?;
    let mut tokens = match &offer_info.value {
        Tuple(v) => Some(v.clone()),
        _ => None,
    }
    .ok_or_else(|| anyhow!("offerInfo token value is not tuple"))?;

    tokens.push(offer_address_token);

    let to_str = get_token_processor(&tokens, token_to_str);
    let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
    let to_i128 = get_token_processor(&tokens, token_to_i128);

    Ok(AuctionDeployedRecord {
        account_addr: get_account_addr(event),
        created_lt: get_created_lt(event)?,
        created_at: get_created_at(event)?,

        offer_address: to_str("offerAddress")?,

        collection: to_str("collection")?,
        nft_owner: to_str("nftOwner")?,
        nft: to_str("nft")?,
        offer: to_str("offer")?,
        price: to_big_decimal("price")?,
        auction_duration: to_big_decimal("auctionDuration")?,
        deploy_nonce: to_i128("deployNonce")?,
    })
}

pub fn build_auction_declined_record(event: &ExtractedOwned) -> Result<AuctionDeclinedRecord> {
    let nft_owner_token = event
        .tokens
        .iter()
        .find(|t| t.name == "nftOwner")
        .ok_or_else(|| anyhow!("Couldn't nftOwner from token"))?
        .clone();

    let data_address_token = event
        .tokens
        .iter()
        .find(|t| t.name == "dataAddress")
        .ok_or_else(|| anyhow!("Couldn't find dataAddress token"))?
        .clone();

    let tokens = vec![nft_owner_token, data_address_token];

    let to_str = get_token_processor(&tokens, token_to_str);

    Ok(AuctionDeclinedRecord {
        account_addr: get_account_addr(event),
        created_lt: get_created_lt(event)?,
        created_at: get_created_at(event)?,

        nft_owner: to_str("nftOwner")?,
        data_address: to_str("dataAddress")?,
    })
}

fn get_account_addr(event: &ExtractedOwned) -> String {
    event.tx.account_id().to_hex_string()
}

fn get_created_at(event: &ExtractedOwned) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_at.0 as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get crated_at of event"))
}

fn get_created_lt(event: &ExtractedOwned) -> Result<i128> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_lt as i128),
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

fn token_to_i16(token: &TokenValue) -> Option<i16> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i16(),
        _ => None,
    }
}

fn token_to_i128(token: &TokenValue) -> Option<i128> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i128(),
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
        .ok_or_else(|| anyhow!("Token with name {} not found", token_name))?;
    mapper_fn(token).ok_or_else(|| anyhow!("Couldn't map token value: {:?}", token))
}
