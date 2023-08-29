use anyhow::Result;
use chrono::NaiveDateTime;
use indexer_repo::types::{decoded, DirectBuyState, EventCategory, EventType, NftPriceSource};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::u128_to_bigdecimal;
use crate::{
    models::events::DirectBuyStateChanged,
    utils::{DecodeContext, KeyInfo},
};

impl Decode for DirectBuyStateChanged {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let state = self.to.into();

        if state == DirectBuyState::Create || state == DirectBuyState::AwaitTokens {
            return Ok(Decoded::ShouldSkip);
        }

        let finished_at = if state == DirectBuyState::Filled {
            Some(
                NaiveDateTime::from_timestamp_opt(ctx.tx_data.get_timestamp(), 0)
                    .unwrap_or_default(),
            )
        } else {
            None
        };

        let price_history = if state == DirectBuyState::Filled {
            Some(decoded::NftPriceHistory {
                source: ctx.tx_data.get_account(),
                source_type: NftPriceSource::DirectBuy,
                created_at: finished_at,
                price: u128_to_bigdecimal(self.value2._price),
                price_token: Some(self.value2.spent_token.to_string()),
                nft: Some(self.value2.nft.to_string()),
            })
        } else {
            None
        };

        let direct_buy = decoded::DirectBuy {
            address: ctx.tx_data.get_account(),
            root: self.value2.factory.to_string(),
            nft: self.value2.nft.to_string(),
            price_token: self.value2.spent_token.to_string(),
            price: u128_to_bigdecimal(self.value2._price),
            buyer: self.value2.creator.to_string(),
            finished_at,
            expired_at: NaiveDateTime::from_timestamp_opt(self.value2.end_time_buy.try_into()?, 0)
                .unwrap_or_default(),
            state,
            created: NaiveDateTime::from_timestamp_opt(self.value2.start_time_buy.try_into()?, 0)
                .unwrap_or_default(),
            updated: NaiveDateTime::from_timestamp_opt(ctx.tx_data.get_timestamp(), 0)
                .unwrap_or_default(),
            tx_lt: ctx.tx_data.logical_time() as i64,
        };

        Ok(Decoded::DirectBuyStateChanged((direct_buy, price_history)))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyStateChanged,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.value2.nft.to_string()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
