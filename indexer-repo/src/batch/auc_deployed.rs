use crate::types::{decoded::AuctionDeployed, AuctionStatus};
use anyhow::{anyhow, Result};
use sqlx::PgConnection;

pub async fn save_auc_deployed(tx: &mut PgConnection, data: &[AuctionDeployed]) -> Result<()> {
    let addresses = data.iter().map(|a| a.address.as_str()).collect::<Vec<_>>();
    let roots = data.iter().map(|a| a.root.as_str()).collect::<Vec<_>>();
    let nfts = data.iter().map(|a| a.nft.as_str()).collect::<Vec<_>>();
    let collections = data
        .iter()
        .map(|c| c.collection.as_str())
        .collect::<Vec<_>>();
    let tx_lts = data.iter().map(|a| a.tx_lt).collect::<Vec<_>>();
    let nft_owners = data
        .iter()
        .map(|a| a.nft_owner.as_str())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
        insert into nft_auction (
            address, 
            root,
            nft,
            collection,
            tx_lt,
            nft_owner,
            status
        )
        select 
            unnest($1::varchar[]),
            unnest($2::varchar[]),
            unnest($3::varchar[]),
            unnest($4::varchar[]),
            unnest($5::bigint[]),
            unnest($6::varchar[]),
            $7::auction_status
        on conflict(address) do nothing
        "#,
        addresses as _,
        roots as _,
        nfts as _,
        collections as _,
        tx_lts as _,
        nft_owners as _,
        AuctionStatus::Created as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
