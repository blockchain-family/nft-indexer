use anyhow::Result;
use indexer_repo::types::{decoded, DirectBuyState, EventCategory, EventType};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::u128_to_bigdecimal;
use crate::{
    models::events::{DirectBuyDeclined, DirectBuyDeployed},
    utils::{DecodeContext, KeyInfo},
};

impl Decode for DirectBuyDeployed {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::DirectBuyDeployed((
            decoded::DirectBuy {
                address: self.direct_buy.to_string(),
                root: ctx.tx_data.get_account(),
                nft: self.nft.to_string(),
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
            },
        )))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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

impl Decode for DirectBuyDeclined {
    fn decode(&self, _ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
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
