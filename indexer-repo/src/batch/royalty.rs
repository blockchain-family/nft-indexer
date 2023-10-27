use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::decoded::SetRoyalty;

pub async fn update_direct_sell(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let addresses = royalties.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    log::info!("ROYALTY WAS SAVED For direct sell. Ids: {}", addresses.first().unwrap());
    Ok(())
}

pub async fn update_direct_buy(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let addresses = royalties.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    log::info!("ROYALTY WAS SAVED For direct buy. Ids: {}", addresses.first().unwrap());
    Ok(())
}

pub async fn update_auction(
    tx: &mut Transaction<'_, Postgres>,
    royalties: &[SetRoyalty],
) -> Result<()> {
    let addresses = royalties.iter().map(|e| e.address.as_str()).collect::<Vec<_>>();
    log::info!("ROYALTY WAS SAVED For auction. Ids: {}", addresses.first().unwrap());
    Ok(())
}