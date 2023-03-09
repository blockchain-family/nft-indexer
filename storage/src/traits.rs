use crate::types::*;
use sqlx::types::BigDecimal;

pub trait EventRecord {
    fn get_address(&self) -> Address;
    fn get_nft(&self) -> Option<Address>;
    fn get_collection(&self) -> Option<Address>;
    fn get_created_at(&self) -> i64;
    fn get_created_lt(&self) -> i64;
    fn get_event_category(&self) -> EventCategory;
    fn get_event_type(&self) -> EventType;
    fn get_message_hash(&self) -> BigDecimal;
}
