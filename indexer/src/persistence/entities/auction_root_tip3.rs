use anyhow::Result;
use indexer_repo::types::{decoded::EventRecord, EventCategory, EventType};

use crate::{
    models::events::{AuctionDeclined, AuctionDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

impl Decode for AuctionDeployed {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let emitter_address = ctx.tx_data.get_account();

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::AuctionRoot].contains(&emitter_address)
        {
            Ok(Decoded::AuctionDeployed(self.offer.to_string()))
        } else {
            Ok(Decoded::ShouldSkip)
        }
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionDeployed,
            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.offer_info.nft.to_string()),
            collection: Some(self.offer_info.collection.to_string()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for AuctionDeclined {
    fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionDeclined,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.nft.to_string()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
