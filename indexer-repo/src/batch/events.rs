use anyhow::{anyhow, Result};
use sqlx::{Postgres, Transaction};

use crate::types::decoded::{EventRecord, OfferDeployed};

pub async fn save_raw_event(
    tx: &mut Transaction<'_, Postgres>,
    events: &[EventRecord],
) -> Result<()> {
    let categories = events.iter().map(|e| e.event_category).collect::<Vec<_>>();
    let types = events.iter().map(|e| e.event_type).collect::<Vec<_>>();
    let addresses = events
        .iter()
        .map(|e| e.address.as_str())
        .collect::<Vec<_>>();
    let nfts = events.iter().map(|e| e.nft.as_deref()).collect::<Vec<_>>();
    let collections = events
        .iter()
        .map(|e| e.collection.as_deref())
        .collect::<Vec<_>>();
    let created_lt = events.iter().map(|e| e.created_lt).collect::<Vec<_>>();
    let created_at = events.iter().map(|e| e.created_at).collect::<Vec<_>>();
    let args = events
        .iter()
        .map(|e| e.raw_data.clone())
        .collect::<Vec<_>>();
    let hashes = events
        .iter()
        .map(|e| e.message_hash.as_str())
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into nft_events (
                event_cat,  
                event_type, 
                address, 
                nft,
                collection, 
                created_lt,
                created_at, 
                args, 
                message_hash
            )
            select 
                unnest($1::event_category[]),
                unnest($2::event_type[]), 
                unnest($3::varchar[]), 
                unnest($4::varchar[]), 
                unnest($5::varchar[]),
                unnest($6::bigint[]), 
                unnest($7::bigint[]),
                unnest($8::jsonb[]),
                unnest($9::text[])
            on conflict(message_hash) do nothing
        "#,
        categories as _,
        types as _,
        addresses as _,
        nfts as _,
        collections as _,
        created_lt as _,
        created_at as _,
        args as _,
        hashes as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}

pub async fn save_deployed_offers(
    tx: &mut Transaction<'_, Postgres>,
    offers: &[OfferDeployed],
) -> Result<()> {
    let addresses = offers
        .iter()
        .map(|of| of.address.as_str())
        .collect::<Vec<_>>();
    let roots = offers.iter().map(|of| of.root.as_str()).collect::<Vec<_>>();
    let created = offers.iter().map(|of| of.created).collect::<Vec<_>>();

    sqlx::query!(
        r#"
            insert into deployed_offers (
                address,
                root,
                created
            )
            select
                unnest($1::varchar[]),
                unnest($2::varchar[]),
                unnest($3::timestamp[])
            on conflict (address) do nothing
        "#,
        addresses as _,
        roots as _,
        created as _,
    )
    .execute(tx)
    .await
    .map_err(|e| anyhow!(e))
    .map(|_| ())
}
