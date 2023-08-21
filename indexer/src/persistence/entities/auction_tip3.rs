use anyhow::Result;
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
    fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
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

        Ok(Decoded::AuctionActive(auction))
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
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let bid = decoded::AuctionBid {
            address: ctx.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: u128_to_bigdecimal(self.next_bid_value),
            buyer: self.buyer.to_string(),
            created_at: ctx.tx_data.get_timestamp(),
            tx_lt: ctx.tx_data.logical_time().try_into()?,
            declined: false,
        };

        Ok(Decoded::AuctionBidPlaced(bid))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidPlaced,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for BidDeclined {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let bid = decoded::AuctionBid {
            address: ctx.tx_data.get_account(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: Default::default(),
            buyer: self.buyer.to_string(),
            created_at: ctx.tx_data.get_timestamp(),
            tx_lt: ctx.tx_data.logical_time().try_into()?,
            declined: true,
        };

        Ok(Decoded::AuctionBidDeclined(bid))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidDeclined,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionComplete {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let auc = decoded::AuctionComplete {
            address: ctx.tx_data.get_account(),
            max_bid: u128_to_bigdecimal(self.value),
        };

        let price_hist = decoded::NftPriceHistory {
            source: ctx.tx_data.get_account(),
            source_type: NftPriceSource::AuctionBid,
            created_at: None,
            price: u128_to_bigdecimal(self.value),
            price_token: None,
            nft: None,
        };

        Ok(Decoded::AuctionComplete((auc, price_hist)))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionComplete,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionCancelled {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let auc = decoded::AuctionCancelled {
            address: ctx.tx_data.get_account(),
        };

        Ok(Decoded::AuctionCancelled(auc))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCancelled,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
