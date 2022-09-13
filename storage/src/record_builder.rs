use crate::{event_records::*, nft_records::*, traits::EventRecord, types::Address};
use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use nekoton_abi::{transaction_parser::ExtractedOwned, BuildTokenValue};
use std::{str::FromStr, sync::Arc};
use ton_abi::{
    Token,
    TokenValue::{self, Tuple, Uint as UintEnum},
};
use ton_block::{CommonMsgInfo, MsgAddressInt};
use transaction_consumer::TransactionConsumer;

impl EventRecord for AuctionDeployed {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
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

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_i64 = get_token_processor(&tokens, token_to_i64);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionDeployed {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            offer_address: to_address("offerAddress")?,

            collection: to_address("collection")?,
            nft_owner: to_address("nftOwner")?,
            nft: to_address("nft")?,
            offer: to_address("offer")?,
            price: to_big_decimal("price")?,
            auction_duration: to_i64("auctionDuration")?,
            deploy_nonce: to_big_decimal("deployNonce")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self.nft.0).unwrap())
    }
}

impl EventRecord for AuctionDeclined {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let nft_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nftOwner")
            .ok_or_else(|| anyhow!("Couldn't find nftOwner token"))?
            .clone();

        let data_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "dataAddress")
            .ok_or_else(|| anyhow!("Couldn't find dataAddress token"))?
            .clone();

        let tokens = vec![nft_owner_token, data_address_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(AuctionDeclined {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            nft_owner: to_address("nftOwner")?,
            data_address: to_address("dataAddress")?,
        })
    }
}

impl EventRecord for AuctionOwnershipTransferred {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(AuctionOwnershipTransferred {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for AuctionCreated {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
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

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionCreated {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            payment_token_root: to_address("paymentTokenRoot")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
        })
    }
}

impl EventRecord for AuctionActive {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
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

        let to_address = get_token_processor(tokens, token_to_addr);
        let to_i64 = get_token_processor(tokens, token_to_i64);
        let to_bigdecimal = get_token_processor(tokens, token_to_big_decimal);

        Ok(AuctionActive {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            auction_subject: to_address("auctionSubject")?,
            subject_owner: to_address("subjectOwner")?,
            payment_token_root: to_address("paymentTokenRoot")?,
            wallet_for_bids: to_address("walletForBids")?,
            start_time: to_i64("startTime")?,
            duration: to_i64("duration")?,
            finish_time: to_i64("finishTime")?,
            _price: to_bigdecimal("_price")?,
            _nonce: to_bigdecimal("_nonce")?,
        })
    }
}

impl EventRecord for BidPlaced {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(BidPlaced {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for BidDeclined {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(BidDeclined {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for AuctionComplete {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let seller_token = event
            .tokens
            .iter()
            .find(|t| t.name == "seller")
            .ok_or_else(|| anyhow!("Couldn't find seller token"))?
            .clone();

        let buyer_token = event
            .tokens
            .iter()
            .find(|t| t.name == "buyer")
            .ok_or_else(|| anyhow!("Couldn't find buyer token"))?
            .clone();

        let value_token = event
            .tokens
            .iter()
            .find(|t| t.name == "value")
            .ok_or_else(|| anyhow!("Couldn't find value token"))?
            .clone();

        let tokens = vec![seller_token, buyer_token, value_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(AuctionComplete {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            seller: to_address("seller")?,
            buyer: to_address("buyer")?,
            value: to_bigdecimal("value")?,
        })
    }
}

impl EventRecord for AuctionCancelled {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(AuctionCancelled {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,
        })
    }
}

impl EventRecord for DirectBuyDeployed {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let direct_buy_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "directBuyAddress")
            .ok_or_else(|| anyhow!("Couldn't find directBuyAddress token"))?
            .clone();

        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let token_root_token = event
            .tokens
            .iter()
            .find(|t| t.name == "tokenRoot")
            .ok_or_else(|| anyhow!("Couldn't find tokenRoot token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let nonce_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nonce")
            .ok_or_else(|| anyhow!("Couldn't find nonce token"))?
            .clone();

        let amount_token = event
            .tokens
            .iter()
            .find(|t| t.name == "amount")
            .ok_or_else(|| anyhow!("Couldn't find amount token"))?
            .clone();

        let tokens = vec![
            direct_buy_address_token,
            sender_token,
            token_root_token,
            nft_token,
            nonce_token,
            amount_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectBuyDeployed {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            direct_buy_address: to_address("directBuyAddress")?,
            sender: to_address("sender")?,
            token_root: to_address("tokenRoot")?,
            nft: to_address("nft")?,
            nonce: to_bigdecimal("nonce")?,
            amount: to_bigdecimal("amount")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self.nft.0).unwrap())
    }
}

impl EventRecord for DirectBuyDeclined {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let token_root_token = event
            .tokens
            .iter()
            .find(|t| t.name == "tokenRoot")
            .ok_or_else(|| anyhow!("Couldn't find tokenRoot token"))?
            .clone();

        let amount_token = event
            .tokens
            .iter()
            .find(|t| t.name == "amount")
            .ok_or_else(|| anyhow!("Couldn't find amount token"))?
            .clone();

        let tokens = vec![sender_token, token_root_token, amount_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectBuyDeclined {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            sender: to_address("sender")?,
            token_root: to_address("tokenRoot")?,
            amount: to_bigdecimal("amount")?,
        })
    }
}

impl EventRecord for DirectBuyOwnershipTransferred {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectBuyOwnershipTransferred {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for DirectSellDeployed {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let _direct_sell_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "_directSellAddress")
            .ok_or_else(|| anyhow!("Couldn't find _directSellAddress token"))?
            .clone();

        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let payment_token_token = event
            .tokens
            .iter()
            .find(|t| t.name == "paymentToken")
            .ok_or_else(|| anyhow!("Couldn't find paymentToken token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let _nonce_token = event
            .tokens
            .iter()
            .find(|t| t.name == "_nonce")
            .ok_or_else(|| anyhow!("Couldn't find _nonce token"))?
            .clone();

        let price_token = event
            .tokens
            .iter()
            .find(|t| t.name == "price")
            .ok_or_else(|| anyhow!("Couldn't find price token"))?
            .clone();

        let tokens = vec![
            _direct_sell_address_token,
            sender_token,
            payment_token_token,
            nft_token,
            _nonce_token,
            price_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(DirectSellDeployed {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            _direct_sell_address: to_address("_directSellAddress")?,
            sender: to_address("sender")?,
            payment_token: to_address("paymentToken")?,
            nft: to_address("nft")?,
            _nonce: to_bigdecimal("_nonce")?,
            price: to_bigdecimal("price")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self.nft.0).unwrap())
    }
}

impl EventRecord for DirectSellDeclined {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let sender_token = event
            .tokens
            .iter()
            .find(|t| t.name == "sender")
            .ok_or_else(|| anyhow!("Couldn't find sender token"))?
            .clone();

        let _nft_address_token = event
            .tokens
            .iter()
            .find(|t| t.name == "_nftAddress")
            .ok_or_else(|| anyhow!("Couldn't find _nftAddress token"))?
            .clone();

        let tokens = vec![sender_token, _nft_address_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectSellDeclined {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            sender: to_address("sender")?,
            _nft_address: to_address("_nftAddress")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self._nft_address.0).unwrap())
    }
}

impl EventRecord for DirectSellOwnershipTransferred {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let old_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldOwner")
            .ok_or_else(|| anyhow!("Couldn't find oldOwner token"))?
            .clone();

        let new_owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newOwner")
            .ok_or_else(|| anyhow!("Couldn't find newOwner token"))?
            .clone();

        let tokens = vec![old_owner_token, new_owner_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(DirectSellOwnershipTransferred {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for DirectBuyStateChanged {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
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

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
        let to_i16 = get_token_processor(&tokens, token_to_i16);
        let to_i64 = get_token_processor(&tokens, token_to_i64);

        Ok(DirectBuyStateChanged {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            from: to_i16("from")?,
            to: to_i16("to")?,

            factory: to_address("factory")?,
            creator: to_address("creator")?,
            spent_token: to_address("spentToken")?,
            nft: to_address("nft")?,
            _time_tx: to_i64("_timeTx")?,
            _price: to_big_decimal("_price")?,
            spent_wallet: to_address("spentWallet")?,
            status: to_i16("status")?,
            sender: to_address("sender")?,
            start_time_buy: to_i64("startTimeBuy")?,
            duration_time_buy: to_i64("durationTimeBuy")?,
            end_time_buy: to_i64("endTimeBuy")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self.nft.0).unwrap())
    }
}

impl EventRecord for DirectSellStateChanged {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
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

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_big_decimal = get_token_processor(&tokens, token_to_big_decimal);
        let to_i16 = get_token_processor(&tokens, token_to_i16);
        let to_i64 = get_token_processor(&tokens, token_to_i64);

        Ok(DirectSellStateChanged {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            from: to_i16("from")?,
            to: to_i16("to")?,

            factory: to_address("factory")?,
            creator: to_address("creator")?,
            token: to_address("token")?,
            nft: to_address("nft")?,
            _time_tx: to_i64("_timeTx")?,
            start: to_i64("start")?,
            end: to_i64("end")?,
            _price: to_big_decimal("_price")?,
            wallet: to_address("wallet")?,
            status: to_i16("status")?,
            sender: to_address("sender")?,
        })
    }

    fn get_nft(&self) -> Option<MsgAddressInt> {
        Some(MsgAddressInt::from_str(&self.nft.0).unwrap())
    }
}

pub async fn build_nft_data(
    nft: MsgAddressInt,
    consumer: Arc<TransactionConsumer>,
) -> Result<(NftCollection, Nft)> {
    let nft_info = rpc::get_info(&nft, consumer.clone()).await?;

    let address = nft.to_string().into();
    let collection = nft_info.collection.to_string().into();
    let owner = nft_info.owner.to_string().into();
    let manager = nft_info.manager.to_string().into();
    let data = rpc::get_json(&nft, consumer.clone()).await?;
    let name = data["name"].to_string();
    let description = data["description"].to_string();
    let updated = chrono::offset::Utc::now().naive_utc();

    let nft = Nft {
        address,
        collection,
        owner,
        manager,
        name,
        description,
        data,
        updated,
    };

    let data = rpc::get_json(&nft_info.collection, consumer.clone()).await?;

    let address = nft_info.collection.to_string().into();
    let owner = rpc::owner(&nft_info.collection, consumer).await?.into();
    let name = data["name"].to_string();
    let description = data["description"].to_string();
    let updated = chrono::offset::Utc::now().naive_utc();

    let collection = NftCollection {
        address,
        owner,
        name,
        description,
        updated,
    };

    Ok((collection, nft))
}

fn get_address(event: &ExtractedOwned) -> Address {
    ("0:".to_string() + &event.tx.account_id().as_hex_string()).into()
}

fn get_created_at(event: &ExtractedOwned) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_at.0 as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get created_at of event"))
}

fn get_created_lt(event: &ExtractedOwned) -> Result<i64> {
    match event.message.header() {
        CommonMsgInfo::ExtOutMsgInfo(info) => Some(info.created_lt as i64),
        _ => None,
    }
    .ok_or_else(|| anyhow!("Couldn't get created_lt of event"))
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

fn token_to_addr(token: &TokenValue) -> Option<Address> {
    match token.token_value() {
        ton_abi::TokenValue::Address(addr) => {
            Some(("0:".to_string() + &addr.get_address().as_hex_string()).into())
        }
        _ => None,
    }
}

fn token_to_i16(token: &TokenValue) -> Option<i16> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i16(),
        _ => None,
    }
}

fn token_to_i64(token: &TokenValue) -> Option<i64> {
    match token.token_value() {
        UintEnum(v) => v.number.to_i64(),
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
