use sqlx::types::chrono;
use crate::types::Address;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NftCollection {
    pub address: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}