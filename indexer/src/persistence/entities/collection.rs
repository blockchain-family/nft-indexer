use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, EventCategory, EventType};

use super::{to_address, types::Decoded, Decode};
use crate::utils::{timestamp_to_datetime, u256_to_bigdecimal};
use crate::{
    models::events::{NftBurned, NftCreated},
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for NftCreated {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = to_address(&self.nft);
        let collection = ctx.tx_data.get_account();

        ctx.nft_cache_service
            .add_collection_of_nft(&address, &collection);

        Ok(Decoded::CreateNft(decoded::NftCreated {
            id: u256_to_bigdecimal(&self.id),
            address,
            collection,
            owner: to_address(&self.owner),
            manager: to_address(&self.manager),
            updated: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            owner_update_lt: ctx.tx_data.logical_time(),
            manager_update_lt: ctx.tx_data.logical_time(),
        }))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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

#[async_trait]
impl Decode for NftBurned {
    async fn decode(&self, _: &DecodeContext) -> Result<Decoded> {
        let record = decoded::NftBurned {
            address: to_address(&self.nft),
            owner: to_address(&self.owner),
            manager: to_address(&self.manager),
        };

        Ok(Decoded::BurnNft(record))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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
