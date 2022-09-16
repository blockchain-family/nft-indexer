use crate::indexer::{record_build_utils::*, traits::ContractEvent};
use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use nekoton_abi::transaction_parser::ExtractedOwned;
use serde::Serialize;
use storage::{
    traits::EventRecord,
    types::{Address, EventCategory, EventType},
};
use ton_abi::TokenValue::Tuple;

/// AuctionRootTip3 events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub offer_address: Address,

    pub collection: Address,
    pub nft_owner: Address,
    pub nft: Address,
    pub offer: Address,
    pub price: BigDecimal,
    pub auction_duration: i64,
    pub deploy_nonce: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub nft_owner: Address,
    pub data_address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// AuctionTip3 events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionCreated {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub payment_token_root: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionActive {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub auction_subject: Address,
    pub subject_owner: Address,
    pub payment_token_root: Address,
    pub wallet_for_bids: Address,
    pub start_time: i64,
    pub duration: i64,
    pub finish_time: i64,
    pub _price: BigDecimal,
    pub _nonce: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BidPlaced {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BidDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionComplete {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub seller: Address,
    pub buyer: Address,
    pub value: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuctionCancelled {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,
}

/// FactoryDirectBuy events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub direct_buy_address: Address,
    pub sender: Address,
    pub token_root: Address,
    pub nft: Address,
    pub nonce: BigDecimal,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub sender: Address,
    pub token_root: Address,
    pub amount: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// FactoryDirectSell events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellDeployed {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub _direct_sell_address: Address,
    pub sender: Address,
    pub payment_token: Address,
    pub nft: Address,
    pub _nonce: BigDecimal,
    pub price: BigDecimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellDeclined {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub sender: Address,
    pub _nft_address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

/// DirectBuy events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectBuyStateChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: Address,
    pub creator: Address,
    pub spent_token: Address,
    pub nft: Address,
    pub _time_tx: i64,
    pub _price: BigDecimal,
    pub spent_wallet: Address,
    pub status: i16,
    pub sender: Address,
    pub start_time_buy: i64,
    pub duration_time_buy: i64,
    pub end_time_buy: i64,
}

/// DirectSell events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DirectSellStateChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: Address,
    pub creator: Address,
    pub token: Address,
    pub nft: Address,
    pub _time_tx: i64,
    pub start: i64,
    pub end: i64,
    pub _price: BigDecimal,
    pub wallet: Address,
    pub status: i16,
    pub sender: Address,
}

// Nft events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NftOwnerChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NftManagerChanged {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_manager: Address,
    pub new_manager: Address,
}

// Collection events

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CollectionOwnershipTransferred {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub old_owner: Address,
    pub new_owner: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NftCreated {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
    pub creator: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NftBurned {
    #[serde(skip_serializing)]
    pub address: Address,
    #[serde(skip_serializing)]
    pub created_lt: i64,
    #[serde(skip_serializing)]
    pub created_at: i64,

    pub id: BigDecimal,
    pub nft: Address,
    pub owner: Address,
    pub manager: Address,
}

impl ContractEvent for AuctionDeployed {
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
}

impl EventRecord for AuctionDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionDeployed
    }
}

impl ContractEvent for AuctionDeclined {
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

impl EventRecord for AuctionDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionDeclined
    }
}

impl ContractEvent for AuctionOwnershipTransferred {
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

impl EventRecord for AuctionOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionOwnershipTransferred
    }
}

impl ContractEvent for AuctionCreated {
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

impl EventRecord for AuctionCreated {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionCreated
    }
}

impl ContractEvent for AuctionActive {
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

impl EventRecord for AuctionActive {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionActive
    }
}

impl ContractEvent for BidPlaced {
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

impl EventRecord for BidPlaced {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionBidPlaced
    }
}

impl ContractEvent for BidDeclined {
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

impl EventRecord for BidDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionBidDeclined
    }
}

impl ContractEvent for AuctionComplete {
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

impl EventRecord for AuctionComplete {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionComplete
    }
}

impl ContractEvent for AuctionCancelled {
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

impl EventRecord for AuctionCancelled {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Auction
    }

    fn get_event_type(&self) -> EventType {
        EventType::AuctionCancelled
    }
}

impl ContractEvent for DirectBuyDeployed {
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
}

impl EventRecord for DirectBuyDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyDeployed
    }
}

impl ContractEvent for DirectBuyDeclined {
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

impl EventRecord for DirectBuyDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyDeclined
    }
}

impl ContractEvent for DirectBuyOwnershipTransferred {
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

impl EventRecord for DirectBuyOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyOwnershipTransferred
    }
}

impl ContractEvent for DirectSellDeployed {
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
}

impl EventRecord for DirectSellDeployed {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellDeployed
    }
}

impl ContractEvent for DirectSellDeclined {
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
}

impl EventRecord for DirectSellDeclined {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellDeclined
    }
}

impl ContractEvent for DirectSellOwnershipTransferred {
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

impl EventRecord for DirectSellOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellOwnershipTransferred
    }
}

impl ContractEvent for DirectBuyStateChanged {
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
}

impl EventRecord for DirectBuyStateChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectBuy
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectBuyStateChanged
    }
}

impl ContractEvent for DirectSellStateChanged {
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
}

impl EventRecord for DirectSellStateChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::DirectSell
    }

    fn get_event_type(&self) -> EventType {
        EventType::DirectSellStateChanged
    }
}

impl ContractEvent for NftOwnerChanged {
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

        Ok(NftOwnerChanged {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for NftOwnerChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Nft
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftOwnerChanged
    }
}

impl ContractEvent for NftManagerChanged {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let old_manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "oldManager")
            .ok_or_else(|| anyhow!("Couldn't find oldManager token"))?
            .clone();

        let new_manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "newManager")
            .ok_or_else(|| anyhow!("Couldn't find newManager token"))?
            .clone();

        let tokens = vec![old_manager_token, new_manager_token];

        let to_address = get_token_processor(&tokens, token_to_addr);

        Ok(NftManagerChanged {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_manager: to_address("oldManager")?,
            new_manager: to_address("newManager")?,
        })
    }
}

impl EventRecord for NftManagerChanged {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Nft
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftManagerChanged
    }
}

impl ContractEvent for CollectionOwnershipTransferred {
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

        Ok(CollectionOwnershipTransferred {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            old_owner: to_address("oldOwner")?,
            new_owner: to_address("newOwner")?,
        })
    }
}

impl EventRecord for CollectionOwnershipTransferred {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::CollectionOwnershipTransferred
    }
}

impl ContractEvent for NftCreated {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let id_token = event
            .tokens
            .iter()
            .find(|t| t.name == "id")
            .ok_or_else(|| anyhow!("Couldn't find id token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "owner")
            .ok_or_else(|| anyhow!("Couldn't find owner token"))?
            .clone();

        let manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "manager")
            .ok_or_else(|| anyhow!("Couldn't find manager token"))?
            .clone();

        let creator_token = event
            .tokens
            .iter()
            .find(|t| t.name == "creator")
            .ok_or_else(|| anyhow!("Couldn't find creator token"))?
            .clone();

        let tokens = vec![
            id_token,
            nft_token,
            owner_token,
            manager_token,
            creator_token,
        ];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(NftCreated {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
            creator: to_address("creator")?,
        })
    }
}

impl EventRecord for NftCreated {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftCreated
    }
}

impl ContractEvent for NftBurned {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized,
    {
        let id_token = event
            .tokens
            .iter()
            .find(|t| t.name == "id")
            .ok_or_else(|| anyhow!("Couldn't find id token"))?
            .clone();

        let nft_token = event
            .tokens
            .iter()
            .find(|t| t.name == "nft")
            .ok_or_else(|| anyhow!("Couldn't find nft token"))?
            .clone();

        let owner_token = event
            .tokens
            .iter()
            .find(|t| t.name == "owner")
            .ok_or_else(|| anyhow!("Couldn't find owner token"))?
            .clone();

        let manager_token = event
            .tokens
            .iter()
            .find(|t| t.name == "manager")
            .ok_or_else(|| anyhow!("Couldn't find manager token"))?
            .clone();

        let tokens = vec![id_token, nft_token, owner_token, manager_token];

        let to_address = get_token_processor(&tokens, token_to_addr);
        let to_bigdecimal = get_token_processor(&tokens, token_to_big_decimal);

        Ok(NftBurned {
            address: get_address(event),
            created_lt: get_created_lt(event)?,
            created_at: get_created_at(event)?,

            id: to_bigdecimal("id")?,
            nft: to_address("nft")?,
            owner: to_address("owner")?,
            manager: to_address("manager")?,
        })
    }
}

impl EventRecord for NftBurned {
    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn get_created_at(&self) -> i64 {
        self.created_at
    }

    fn get_created_lt(&self) -> i64 {
        self.created_lt
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::Collection
    }

    fn get_event_type(&self) -> EventType {
        EventType::NftBurned
    }
}
