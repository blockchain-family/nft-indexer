use anyhow::Result;
use indexer_repo::types::{decoded, EventCategory, EventType};

use crate::persistence::entities::{Decode, Decoded};
use crate::{
    models::events::{DirectSellDeclined, DirectSellDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{DecodeContext, KeyInfo},
};

impl Decode for DirectSellDeployed {
    fn decode(&self, ctx: &DecodeContext) -> Result<Decoded> {
        let emitter_address = ctx.tx_data.get_account();

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell]
            .contains(&emitter_address)
        {
            Ok(Decoded::DirectSellDeployed(emitter_address))
        } else {
            Ok(Decoded::ShouldSkip)
        }
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeployed,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for DirectSellDeclined {
    fn decode(&self, _msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, msg_info: &DecodeContext) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(decoded::EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeclined,

            address: msg_info.tx_data.get_account(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
