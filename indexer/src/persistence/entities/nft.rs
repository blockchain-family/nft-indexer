use anyhow::Result;
use indexer_repo::types::{decoded, EventCategory, EventType};

use super::{types::Decoded, Decode};
use crate::models::events::MetadataUpdated;
use crate::utils::timestamp_to_datetime;
use crate::{
    models::events::{ManagerChanged, OwnerChanged},
    utils::{DecodeContext, KeyInfo},
};

impl Decode for OwnerChanged {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft_new_owner = decoded::AddressChanged {
            id_address: ctx.tx_data.get_account(),
            new_address: self.new_owner.to_string(),
            logical_time: ctx.tx_data.logical_time(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
        };

        Ok(Decoded::OwnerChangedNft(nft_new_owner))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Nft,
            event_type: EventType::NftOwnerChanged,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(ctx.tx_data.get_account()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for ManagerChanged {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft_new_manager = decoded::AddressChanged {
            id_address: ctx.tx_data.get_account(),
            new_address: self.new_manager.to_string(),
            logical_time: ctx.tx_data.logical_time(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
        };

        Ok(Decoded::ManagerChangedNft(nft_new_manager))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Nft,
            event_type: EventType::NftManagerChanged,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(ctx.tx_data.get_account()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for MetadataUpdated {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let metadata_updated = decoded::MetadataUpdated {
            address: ctx.tx_data.get_account(),
            tx_lt: ctx.tx_data.logical_time(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
        };

        Ok(Decoded::MetadataUpdatedNft(metadata_updated))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Nft,
            event_type: EventType::MetadataUpdated,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(ctx.tx_data.get_account()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
