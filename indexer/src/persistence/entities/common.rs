use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::utils::timestamp_to_datetime;
use crate::{
    models::events::{
        AddCollectionRules, MarketFeeChanged, MarketFeeDefaultChanged, OwnershipTransferred,
        RemoveCollectionRules,
    },
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

#[async_trait]
impl Decode for OwnershipTransferred {
    async fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Common,
            event_type: EventType::OwnershipTransferred,

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

#[async_trait]
impl Decode for MarketFeeDefaultChanged {
    async fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::MarketFeeDefaultChanged,

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

#[async_trait]
impl Decode for MarketFeeChanged {
    async fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::MarketFeeChanged,

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

#[async_trait]
impl Decode for AddCollectionRules {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionRulesChanged(decoded::CollectionFee {
            address: self.collection.to_string(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            numerator: Some(self.collection_fee_info.numerator.try_into()?),
            denominator: Some(self.collection_fee_info.denominator.try_into()?),
        }))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::AddCollectionRules,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

#[async_trait]
impl Decode for RemoveCollectionRules {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionRulesChanged(decoded::CollectionFee {
            address: self.collection.to_string(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            numerator: None,
            denominator: None,
        }))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::RemoveCollectionRules,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
