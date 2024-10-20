use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, DirectBuyState, EventCategory, EventType};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::{DirectBuyDeclined, DirectBuyDeployed},
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for DirectBuyDeployed {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::DirectBuyDeployed((
            decoded::DirectBuy {
                address: self.direct_buy.to_string(),
                root: ctx.tx_data.get_account(),
                nft: self.nft.to_string(),
                collection: None,
                price_token: self.token.to_string(),
                price: u128_to_bigdecimal(self.amount),
                buyer: self.sender.to_string(),
                finished_at: None,
                expired_at: Default::default(),
                state: DirectBuyState::Create,
                created: Default::default(),
                updated: Default::default(),
                tx_lt: ctx.tx_data.logical_time() as i64,
            },
            decoded::OfferDeployed {
                address: self.direct_buy.to_string(),
                root: ctx.tx_data.get_account(),
                created: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            },
        )))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

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

#[async_trait]
impl Decode for DirectBuyDeclined {
    async fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyDeclined,

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
