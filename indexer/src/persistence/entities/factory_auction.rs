use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::utils::timestamp_to_datetime;
use crate::{
    models::events::{AuctionDeclined, AuctionDeployed},
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

#[async_trait]
impl Decode for AuctionDeployed {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::AuctionDeployed((
            decoded::AuctionDeployed {
                address: self.offer_info.offer.to_string(),
                root: ctx.tx_data.get_account(),
                nft: self.offer_info.nft.to_string(),
                collection: self.offer_info.collection.to_string(),
                nft_owner: self.offer_info.nft_owner.to_string(),
                tx_lt: ctx.tx_data.logical_time() as i64,
            },
            decoded::OfferDeployed {
                address: self.offer_info.offer.to_string(),
                root: ctx.tx_data.get_account(),
                created: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            },
        )))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
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

#[async_trait]
impl Decode for AuctionDeclined {
    async fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
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
