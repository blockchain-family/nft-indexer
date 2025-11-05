use anyhow::Result;
use indexer_repo::types::{decoded, DirectSellState, EventCategory, EventType, NftPriceSource};

use crate::models::events::DirectSellStateChanged;
use crate::persistence::entities::{Decode, Decoded};
use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal, DecodeContext, KeyInfo};

impl Decode for DirectSellStateChanged {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let state = self.to.into();

        if state == DirectSellState::Create || state == DirectSellState::AwaitNft {
            return Ok(Decoded::ShouldSkip);
        }

        let finished_at = if state == DirectSellState::Filled {
            Some(timestamp_to_datetime(ctx.tx_data.get_timestamp()))
        } else {
            None
        };

        let price_history = if state == DirectSellState::Filled {
            Some(decoded::NftPriceHistory {
                source: ctx.tx_data.get_account(),
                source_type: NftPriceSource::DirectSell,
                created_at: finished_at.unwrap(),
                price: u128_to_bigdecimal(self.value2._price),
                price_token: self.value2.token.to_string(),
                usd_price: None,
                nft: self.value2.nft.to_string(),
                collection: self.value2.collection.to_string(),
            })
        } else {
            None
        };

        let direct_sell = decoded::DirectSell {
            address: ctx.tx_data.get_account(),
            root: self.value2.factory.to_string(),
            nft: self.value2.nft.to_string(),
            collection: Some(self.value2.collection.to_string()),
            price_token: self.value2.token.to_string(),
            price: u128_to_bigdecimal(self.value2._price),
            seller: self.value2.creator.to_string(),
            finished_at,
            expired_at: timestamp_to_datetime(self.value2.end.try_into()?),
            state,
            created: timestamp_to_datetime(self.value2.start.try_into()?),
            updated: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            tx_lt: ctx.tx_data.logical_time() as i64,
        };

        Ok(Decoded::DirectSellStateChanged((
            direct_sell,
            price_history,
        )))
    }

    fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellStateChanged,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(self.value2.nft.to_string()),
            collection: Some(self.value2.collection.to_string()),
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
