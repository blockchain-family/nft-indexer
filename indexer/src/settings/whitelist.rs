use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::collections::HashMap;

use super::config::Config;

#[derive(PartialEq, Eq, Hash)]
pub enum OfferRootType {
    AuctionRoot = 0,
    FactoryDirectBuy,
    FactoryDirectSell,
}

pub static TRUSTED_ADDRESSES: OnceCell<HashMap<OfferRootType, Vec<String>>> = OnceCell::new();

pub fn init_trusted_addresses(config: Config) -> Result<()> {
    let mut m = HashMap::new();
    m.insert(OfferRootType::AuctionRoot, config.trusted_auction_roots);
    m.insert(
        OfferRootType::FactoryDirectBuy,
        config.trusted_direct_buy_factories,
    );
    m.insert(
        OfferRootType::FactoryDirectSell,
        config.trusted_direct_sell_factories,
    );

    TRUSTED_ADDRESSES
        .set(m)
        .map_err(|_| anyhow!("Unable to inititalize trusted addresses"))
}

pub async fn init_whitelist_addresses(pg_pool: &PgPool) -> Result<()> {
    let mut pg_pool_tx = pg_pool.begin().await?;

    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::AuctionRoot].iter() {
        indexer_repo::actions::add_whitelist_address(addr, &mut pg_pool_tx).await?;
    }

    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectBuy].iter() {
        indexer_repo::actions::add_whitelist_address(addr, &mut pg_pool_tx).await?;
    }

    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell].iter() {
        indexer_repo::actions::add_whitelist_address(addr, &mut pg_pool_tx).await?;
    }

    pg_pool_tx.commit().await?;

    Ok(())
}
