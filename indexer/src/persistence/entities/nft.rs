use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, EventCategory, EventType};

use super::{types::Decoded, Decode};
use crate::utils::timestamp_to_datetime;
use crate::{
    models::events::{ManagerChanged, OwnerChanged},
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for OwnerChanged {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft_new_owner = decoded::AddressChanged {
            id_address: ctx.tx_data.get_account(),
            new_address: self.new_owner.to_string(),
            logical_time: ctx.tx_data.logical_time(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
        };

        Ok(Decoded::OwnerChangedNft(nft_new_owner))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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

#[async_trait]
impl Decode for ManagerChanged {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft_new_manager = decoded::AddressChanged {
            id_address: ctx.tx_data.get_account(),
            new_address: self.new_manager.to_string(),
            logical_time: ctx.tx_data.logical_time(),
            timestamp: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
        };

        Ok(Decoded::ManagerChangedNft(nft_new_manager))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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
