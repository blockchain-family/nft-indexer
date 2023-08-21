use anyhow::Result;
use chrono::NaiveDateTime;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::{
    models::events::{NftBurned, NftCreated},
    utils::{DecodeContext, KeyInfo},
};

use super::{types::Decoded, Decode};

impl Decode for NftCreated {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let logical_time = ctx.tx_data.logical_time();
        let event_time =
            NaiveDateTime::from_timestamp_opt(ctx.tx_data.get_timestamp(), 0).unwrap_or_default();

        let record = decoded::NftCreated {
            address: self.nft.to_string(),
            collection: ctx.tx_data.get_account(),
            owner: self.owner.to_string(),
            manager: self.manager.to_string(),
            updated: event_time,
            owner_update_lt: logical_time,
            manager_update_lt: logical_time,
        };

        Ok(Decoded::CreateNft(record))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftCreated,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.nft.to_string()),
            collection: Some(ctx.tx_data.get_account()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for NftBurned {
    fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        let record = decoded::NftBurned {
            address: self.nft.to_string(),
            owner: self.owner.to_string(),
            manager: self.manager.to_string(),
        };

        Ok(Decoded::BurnNft(record))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Collection,
            event_type: EventType::NftBurned,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.nft.to_string()),
            collection: Some(ctx.tx_data.get_account()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
