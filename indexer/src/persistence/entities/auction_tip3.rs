use anyhow::Result;
use chrono::NaiveDateTime;
use indexer_repo::types::{decoded, EventCategory, EventType, NftPriceSource};

use crate::utils::u128_to_bigdecimal;
use crate::{
    models::events::{
        AuctionActive, AuctionCancelled, AuctionComplete, AuctionCreated, BidDeclined, BidPlaced,
    },
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

impl Decode for AuctionCreated {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionCreated(decoded::AddressChanged {
            id_address: ctx.tx_data.get_account(),
            new_address: self.value0.auction_subject.to_string(),
            timestamp: 0,
        }))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCreated,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.value0.auction_subject.to_string()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionActive {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let auction = decoded::AuctionActive {
            address: ctx.tx_data.get_account(),
            nft: self.value0.auction_subject.to_string(),
            wallet_for_bids: self.value0.wallet_for_bids.to_string(),
            price_token: self.value0._payment_token.to_string(),
            start_price: u128_to_bigdecimal(self.value0._price),
            min_bid: u128_to_bigdecimal(self.value0._price),
            created_at: self.value0.start_time.try_into()?,
            finished_at: self.value0.finish_time.try_into()?,
            tx_lt: ctx.tx_data.logical_time().try_into()?,
        };

        let price_hist = decoded::NftPriceHistory {
            source: ctx.tx_data.get_account(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(ctx.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value0._price),
            price_token: Some(self.value0._payment_token.to_string()),
            nft: Some(self.value0.auction_subject.to_string()),
            collection: None,
        };

        Ok(Decoded::AuctionActive((auction, price_hist)))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionActive,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.value0.auction_subject.to_string()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for BidPlaced {
    fn decode(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        let bid = decoded::AuctionBid {
            address: msg_info.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: u128_to_bigdecimal(self.next_bid_value),
            buyer: self.buyer.to_string(),
            created_at: msg_info.tx_data.get_timestamp(),
            tx_lt: msg_info.tx_data.logical_time().try_into()?,
            declined: false,
        };

        let price_hist = decoded::NftPriceHistory {
            source: msg_info.tx_data.get_account(),
            source_type: NftPriceSource::AuctionBid,
            created_at: NaiveDateTime::from_timestamp_opt(msg_info.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            price: u128_to_bigdecimal(self.value),
            price_token: None,
            nft: None,
            collection: None,
        };

        Ok(Decoded::AuctionBidPlaced((bid, price_hist)))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidPlaced,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for BidDeclined {
    fn decode(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        let bid = decoded::AuctionBid {
            address: msg_info.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: Default::default(),
            buyer: self.buyer.to_string(),
            created_at: msg_info.tx_data.get_timestamp(),
            tx_lt: msg_info.tx_data.logical_time().try_into()?,
            declined: true,
        };

        Ok(Decoded::AuctionBidDeclined(bid))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidDeclined,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionComplete {
    fn decode(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        let auc = decoded::AuctionComplete {
            address: msg_info.tx_data.get_account(),
            max_bid: u128_to_bigdecimal(self.value),
        };

        Ok(Decoded::AuctionComplete(auc))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionComplete,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionCancelled {
    fn decode(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        let auc = decoded::AuctionCancelled {
            address: msg_info.tx_data.get_account(),
        };

        Ok(Decoded::AuctionCancelled(auc))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCancelled,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
