use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, DirectBuyState, EventCategory, EventType, NftPriceSource};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::DirectBuyStateChanged,
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for DirectBuyStateChanged {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let state = self.to.into();

        if state == DirectBuyState::Create || state == DirectBuyState::AwaitTokens {
            return Ok(Decoded::ShouldSkip);
        }

        let finished_at = if state == DirectBuyState::Filled {
            Some(timestamp_to_datetime(ctx.tx_data.get_timestamp()))
        } else {
            None
        };

        let nft = self.value2.nft.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        let price_history = if state == DirectBuyState::Filled {
            Some(decoded::NftPriceHistory {
                source: ctx.tx_data.get_account(),
                source_type: NftPriceSource::DirectBuy,
                created_at: finished_at.unwrap(),
                price: u128_to_bigdecimal(self.value2._price),
                price_token: self.value2.spent_token.to_string(),
                usd_price: None,
                nft: nft.clone(),
                collection: collection.clone().unwrap_or_default(),
            })
        } else {
            None
        };

        let direct_buy = decoded::DirectBuy {
            address: ctx.tx_data.get_account(),
            root: self.value2.factory.to_string(),
            nft,
            collection,
            price_token: self.value2.spent_token.to_string(),
            price: u128_to_bigdecimal(self.value2._price),
            buyer: self.value2.creator.to_string(),
            finished_at,
            expired_at: timestamp_to_datetime(self.value2.end_time_buy.try_into()?),
            state,
            created: timestamp_to_datetime(self.value2.start_time_buy.try_into()?),
            updated: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            tx_lt: ctx.tx_data.logical_time() as i64,
        };

        Ok(Decoded::DirectBuyStateChanged((direct_buy, price_history)))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft = self.value2.nft.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectBuy,
            event_type: EventType::DirectBuyStateChanged,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: Some(nft),
            collection,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
