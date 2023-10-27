use anyhow::Result;
use indexer_repo::types::{decoded, EventCategory, EventType, NftPriceSource};

use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::RoyaltySet,
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

impl Decode for RoyaltySet {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let st = decoded::SetRoyalty {
            address: ctx.tx_data.get_account(),
            denominator: self._royalty.denominator.try_into()?,
            numerator: self._royalty.numerator.try_into()?,
        };
        Ok(Decoded::RoyaltySet(st))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCreated,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self._royalty.denominator.to_string()),
            collection: Some(self._royalty.denominator.to_string()),
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
