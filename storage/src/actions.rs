use crate::{traits::EventRecord, types::*};
use anyhow::Result;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn save_event<T: EventRecord + Serialize>(
    record: &T,
    pool: &PgPool,
) -> Result<PgQueryResult> {
    Ok(sqlx::query!(
        r#"
        insert into nft_events (event_cat, event_type, address, created_lt, created_at, args)
        values ($1, $2, $3, $4, $5, $6)
        "#,
        record.get_event_category() as EventCategory,
        record.get_event_type() as EventType,
        record.get_address() as Address,
        record.get_created_lt(),
        record.get_created_at(),
        serde_json::to_value(record)?,
    )
    .execute(pool)
    .await?)
}

pub async fn upsert_nft_meta(nft_meta: &NftMeta, pool: &PgPool) -> Result<PgQueryResult> {
    Ok(sqlx::query!(
        r#"
        insert into nft_metadata (nft, meta, updated)
        values ($1, $2, $3)
        on conflict (nft) where updated < $3 do update
        set meta = $2,
            updated = $3
        "#,
        &nft_meta.nft as &Address,
        nft_meta.meta,
        nft_meta.updated
    )
    .execute(pool)
    .await?)
}

pub async fn upsert_nft(nft: &Nft, pool: &PgPool) -> Result<PgQueryResult> {
    if let Some(mut saved_nft) = sqlx::query_as!(
        Nft,
        r#"
        select address as "address!: Address", 
            collection as "collection?: Address",
            owner as "owner?: Address", 
            manager as "manager?: Address",
            name as "name!", 
            description as "description!",
            burned as "burned!", 
            updated as "updated!", 
            tx_lt as "tx_lt!"
        from nft where address = $1
        "#,
        &nft.address as &Address
    )
    .fetch_optional(pool)
    .await?
    {
        // TODO: name? description? collection?
        if nft.tx_lt > saved_nft.tx_lt {
            if nft.owner.is_some() {
                saved_nft.owner = nft.owner.clone();
            }

            if nft.manager.is_some() {
                saved_nft.manager = nft.manager.clone();
            }

            saved_nft.burned = nft.burned;
            saved_nft.updated = nft.updated;
            saved_nft.tx_lt = nft.tx_lt;
        }

        if saved_nft.collection.is_none() {
            saved_nft.collection = nft.collection.clone();
        }

        if saved_nft.owner.is_none() {
            saved_nft.owner = nft.owner.clone();
        }

        if saved_nft.manager.is_none() {
            saved_nft.manager = nft.manager.clone();
        }

        saved_nft.name = nft.name.clone();
        saved_nft.description = nft.description.clone();

        Ok(sqlx::query!(
            r#"
            update nft 
            set collection = $2,
                owner = $3,
                manager = $4,
                name = $5,
                description = $6,
                burned = $7,
                updated = $8,
                tx_lt = $9
            where address = $1
            "#,
            saved_nft.address as Address,
            saved_nft.collection as Option<Address>,
            saved_nft.owner as Option<Address>,
            saved_nft.manager as Option<Address>,
            saved_nft.name,
            saved_nft.description,
            saved_nft.burned,
            saved_nft.updated,
            saved_nft.tx_lt
        )
        .execute(pool)
        .await?)
    } else {
        Ok(sqlx::query!(
            r#"
            insert into nft (address, collection, owner, manager, name, description, burned, updated, tx_lt)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            &nft.address as &Address,
            &nft.collection as &Option<Address>,
            &nft.owner as &Option<Address>,
            &nft.manager as &Option<Address>,
            nft.name,
            nft.description,
            nft.burned,
            nft.updated,
            nft.tx_lt
        )
        .execute(pool)
        .await?)
    }
}

pub async fn add_whitelist_address(address: &Address, pool: &PgPool) -> Result<PgQueryResult> {
    Ok(sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        "#,
        address as &Address
    )
    .execute(pool)
    .await?)
}
