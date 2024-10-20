use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, DirectSellState, EventCategory, EventType, NftPriceSource};

use crate::persistence::entities::{Decode, Decoded};
use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::DirectSellStateChanged,
    utils::{DecodeContext, KeyInfo},
};

#[async_trait]
impl Decode for DirectSellStateChanged {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let state = self.to.into();

        if state == DirectSellState::Create || state == DirectSellState::AwaitNft {
            return Ok(Decoded::ShouldSkip);
        }

        let finished_at = if state == DirectSellState::Filled {
            Some(timestamp_to_datetime(ctx.tx_data.get_timestamp()))
        } else {
            None
        };

        let nft = self.value2.nft.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        let price_history = if state == DirectSellState::Filled {
            Some(decoded::NftPriceHistory {
                source: ctx.tx_data.get_account(),
                source_type: NftPriceSource::DirectSell,
                created_at: finished_at.unwrap(),
                price: u128_to_bigdecimal(self.value2._price),
                price_token: self.value2.token.to_string(),
                usd_price: None,
                nft: nft.clone(),
                collection: collection.clone().unwrap_or_default(),
            })
        } else {
            None
        };

        let direct_sell = decoded::DirectSell {
            address: ctx.tx_data.get_account(),
            root: self.value2.factory.to_string(),
            nft,
            collection,
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

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft = self.value2.nft.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellStateChanged,

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
