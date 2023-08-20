use anyhow::Result;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::{
    models::events::{
        AddCollectionRules, MarketFeeChanged, MarketFeeDefaultChanged, OwnershipTransferred,
        RemoveCollectionRules,
    },
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

impl Decode for OwnershipTransferred {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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

impl Decode for MarketFeeDefaultChanged {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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

impl Decode for MarketFeeChanged {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::MarketFeeChanged,

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

impl Decode for AddCollectionRules {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionRulesChanged(decoded::CollectionFee {
            address: self.collection.to_string(),
            numerator: Some(self.collection_fee_info.numerator.try_into()?),
            denominator: Some(self.collection_fee_info.denominator.try_into()?),
        }))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::AddCollectionRules,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for RemoveCollectionRules {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionRulesChanged(decoded::CollectionFee {
            address: self.collection.to_string(),
            numerator: None,
            denominator: None,
        }))
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::RemoveCollectionRules,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: None,
            collection: Some(self.collection.to_string()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
