use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, DirectSellState, EventCategory, EventType};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::{DirectSellDeclined, DirectSellDeployed},
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for DirectSellDeployed {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::DirectSellDeployed((
            decoded::DirectSell {
                address: self.direct_sell.to_string(),
                root: ctx.tx_data.get_account(),
                nft: self.nft.to_string(),
                collection: None,
                price_token: self.payment_token.to_string(),
                price: u128_to_bigdecimal(self.price),
                seller: self.sender.to_string(),
                finished_at: None,
                expired_at: Default::default(),
                state: DirectSellState::Create,
                created: Default::default(),
                updated: Default::default(),
                tx_lt: ctx.tx_data.logical_time() as i64,
            },
            decoded::OfferDeployed {
                address: self.direct_sell.to_string(),
                root: ctx.tx_data.get_account(),
                created: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            },
        )))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeployed,

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
impl Decode for DirectSellDeclined {
    async fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeclined,

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
