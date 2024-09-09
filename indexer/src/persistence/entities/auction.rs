use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{decoded, AuctionCachedInfo, EventCategory, EventType, NftPriceSource};

use crate::utils::{timestamp_to_datetime, u128_to_bigdecimal};
use crate::{
    models::events::{
        AuctionActive, AuctionCancelled, AuctionComplete, AuctionCreated, BidDeclined, BidPlaced,
    },
    utils::{DecodeContext, KeyInfo},
};

use super::{Decode, Decoded};

#[async_trait]
impl Decode for AuctionCreated {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let nft = self.value0.auction_subject.to_string();
        let collection = ctx
            .nft_cache_service
            .get_collection_of_nft(&address)
            .await?
            .unwrap_or_default();
        let nft_owner = self.value0.subject_owner.to_string();
        let price_token = Some(self.value0.payment_token.to_string());
        let start_time = Some(self.value0.start_time as i64);

        ctx.nft_cache_service.add_auction_cached_info(
            &address,
            AuctionCachedInfo {
                nft,
                collection,
                nft_owner,
                price_token,
                start_time,
            },
        );

        Ok(Decoded::ShouldSkip)
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft = self.value0.auction_subject.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCreated,

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

#[async_trait]
impl Decode for AuctionActive {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let auction = decoded::AuctionActive {
            address: ctx.tx_data.get_account(),
            nft: self.value0.auction_subject.to_string(),
            wallet_for_bids: self.value0.wallet_for_bids.to_string(),
            price_token: self.value0.payment_token.to_string(),
            start_price: u128_to_bigdecimal(self.value0.price),
            min_bid: u128_to_bigdecimal(self.value0.price),
            created_at: timestamp_to_datetime(self.value0.start_time.try_into()?),
            finished_at: timestamp_to_datetime(self.value0.finish_time.try_into()?),
            tx_lt: ctx.tx_data.logical_time() as i64,
        };

        Ok(Decoded::AuctionActive(auction))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let nft = self.value0.auction_subject.to_string();
        let collection = ctx.nft_cache_service.get_collection_of_nft(&nft).await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionActive,

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

#[async_trait]
impl Decode for BidPlaced {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        let bid = decoded::AuctionBid {
            address,
            collection: cached_info
                .as_ref()
                .map(|i| i.collection.clone())
                .unwrap_or_default(),
            nft: cached_info
                .as_ref()
                .map(|i| i.nft.clone())
                .unwrap_or_default(),
            nft_owner: cached_info
                .as_ref()
                .map(|i| i.nft_owner.clone())
                .unwrap_or_default(),
            price_token: cached_info
                .as_ref()
                .and_then(|i| i.price_token.clone())
                .unwrap_or_default(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: u128_to_bigdecimal(self.next_bid_value),
            buyer: self.buyer.to_string(),
            created_at: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            tx_lt: ctx.tx_data.logical_time().try_into()?,
            declined: false,
        };

        Ok(Decoded::AuctionBidPlaced(bid))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidPlaced,

            address: ctx.tx_data.get_account(),
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: None,
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

#[async_trait]
impl Decode for BidDeclined {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        let bid = decoded::AuctionBid {
            address,
            collection: cached_info
                .as_ref()
                .map(|i| i.collection.clone())
                .unwrap_or_default(),
            nft: cached_info
                .as_ref()
                .map(|i| i.nft.clone())
                .unwrap_or_default(),
            nft_owner: cached_info
                .as_ref()
                .map(|i| i.nft_owner.clone())
                .unwrap_or_default(),
            price_token: cached_info
                .as_ref()
                .and_then(|i| i.price_token.clone())
                .unwrap_or_default(),
            bid_value: u128_to_bigdecimal(self.value),
            next_value: Default::default(),
            buyer: self.buyer.to_string(),
            created_at: timestamp_to_datetime(ctx.tx_data.get_timestamp()),
            tx_lt: ctx.tx_data.logical_time().try_into()?,
            declined: true,
        };

        Ok(Decoded::AuctionBidDeclined(bid))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionBidDeclined,

            address,
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: cached_info.as_ref().map(|i| i.nft.clone()),
            collection: cached_info.map(|i| i.collection),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

#[async_trait]
impl Decode for AuctionComplete {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        let auc = decoded::AuctionComplete {
            address: address.clone(),
            max_bid: u128_to_bigdecimal(self.value),
        };

        let price_hist = decoded::NftPriceHistory {
            source: address,
            source_type: NftPriceSource::AuctionBid,
            created_at: timestamp_to_datetime(
                cached_info
                    .as_ref()
                    .and_then(|i| i.start_time)
                    .unwrap_or_default(),
            ),
            price: u128_to_bigdecimal(self.value),
            price_token: cached_info
                .as_ref()
                .and_then(|i| i.price_token.clone())
                .unwrap_or_default(),
            usd_price: None,
            nft: cached_info
                .as_ref()
                .map(|i| i.nft.clone())
                .unwrap_or_default(),
            collection: cached_info
                .as_ref()
                .map(|i| i.collection.clone())
                .unwrap_or_default(),
        };

        Ok(Decoded::AuctionComplete((auc, price_hist)))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionComplete,

            address,
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: cached_info.as_ref().map(|i| i.nft.clone()),
            collection: cached_info.as_ref().map(|i| i.collection.clone()),

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

#[async_trait]
impl Decode for AuctionCancelled {
    async fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let auc = decoded::AuctionCancelled {
            address: ctx.tx_data.get_account(),
        };

        Ok(Decoded::AuctionCancelled(auc))
    }

    async fn decode_event(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let address = ctx.tx_data.get_account();
        let cached_info = ctx
            .nft_cache_service
            .get_auction_cached_info(&address)
            .await?;

        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::Auction,
            event_type: EventType::AuctionCancelled,

            address,
            created_lt: ctx.tx_data.logical_time() as i64,
            created_at: ctx.tx_data.get_timestamp(),
            message_hash: ctx.message_hash.to_string(),
            nft: cached_info.as_ref().map(|i| i.nft.clone()),
            collection: cached_info.as_ref().map(|i| i.collection.clone()),
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
