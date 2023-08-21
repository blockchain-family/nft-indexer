use crate::types::{decoded::AuctionDeployed, AuctionStatus};
use anyhow::{anyhow, Result};
use sqlx::PgPool;

pub async fn save_auc_deployed(pool: &PgPool, data: Vec<AuctionDeployed>) -> Result<()> {
    let addresses = data.iter().map(|a| a.address.as_str()).collect::<Vec<_>>();
    let roots = data.iter().map(|a| a.root.as_str()).collect::<Vec<_>>();
    let nfts = data.iter().map(|a| a.nft.as_str()).collect::<Vec<_>>();
    let tx_lts = data.iter().map(|a| a.tx_lt).collect::<Vec<_>>();

    sqlx::query!(
        r#"
        insert into nft_auction (
            address, 
            root,
            nft,
            tx_lt,
            status
        )
        select *
        from unnest(
            $1::varchar[],
            $2::varchar[],
            $3::varchar[],
            $4::bigint[],
            $5::auction_status[]
        ) 
        on conflict(address) do nothing
        "#,
        addresses as _,
        roots as _,
        nfts as _,
        tx_lts as _,
        AuctionStatus::Created as _,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
