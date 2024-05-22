use anyhow::Result;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::utils::{timestamp_to_datetime, u256_to_bigdecimal};
use crate::{
    models::events::{NftBurned, NftCreated},
    utils::{DecodeContext, KeyInfo},
};

use super::{to_address, types::Decoded, Decode};

impl Decode for NftCreated {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::CreateNft(decoded::NftCreated {
            id: u256_to_bigdecimal(&self.id),
            address: to_address(&self.nft),
            collection: ctx.tx_data.get_account(),
            owner: to_address(&self.owner),
            manager: to_address(&self.manager),
            updated: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            owner_update_lt: ctx.tx_data.logical_time(),
            manager_update_lt: ctx.tx_data.logical_time(),
        }))
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
            address: to_address(&self.nft),
            owner: to_address(&self.owner),
            manager: to_address(&self.manager),
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
