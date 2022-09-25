use crate::{traits::EventRecord, types::*};
use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;

pub async fn save_event<T: EventRecord + Serialize>(record: &T, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
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
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_collection(collection: &NftCollection, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into nft_collection (address, owner, name, description, updated)
        values ($1, $2, $3, $4, $5)
        on conflict (address) where updated < $5 do update
        set owner = $2, name = $3, description = $4, updated = $5
        "#,
        &collection.address as &Address,
        &collection.owner as &Address,
        collection.name,
        collection.description,
        collection.updated
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_nft_meta(nft_meta: &NftMeta, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into nft_metadata (nft, meta, updated)
        values ($1, $2, $3)
        on conflict (nft) where updated < $3 do update
        set meta = $2, updated = $3
        "#,
        &nft_meta.nft as &Address,
        nft_meta.meta,
        nft_meta.updated
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_nft(nft: &Nft, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    let nft = if let Some(mut saved_nft) = sqlx::query_as!(
        Nft,
        r#"
        select address as "address!: Address", collection as "collection?: Address", owner as "owner?: Address", 
            manager as "manager?: Address", name as "name!", description as "description!", burned as "burned!", 
            updated as "updated!", tx_lt as "tx_lt!"
        from nft where address = $1
        "#,
        &nft.address as &Address
    )
    .fetch_optional(&mut tx)
    .await?
    {
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

        if saved_nft.name.is_empty() {
            saved_nft.name = nft.name.clone();
        }

        if saved_nft.description.is_empty() {
            saved_nft.description = nft.description.clone();
        }

        saved_nft
    } else {
        nft.clone()
    };

    sqlx::query!(
        r#"
        insert into nft (address, collection, owner, manager, name, description, burned, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        on conflict (address) where tx_lt < $9 do update
        set collection = $2, owner = $3, manager = $4, name = $5, description = $6, burned = $7, updated = $8, 
            tx_lt = $9
        "#,
        nft.address as Address,
        nft.collection as Option<Address>,
        nft.owner as Option<Address>,
        nft.manager as Option<Address>,
        nft.name,
        nft.description,
        nft.burned,
        nft.updated,
        nft.tx_lt
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_auction(auction: &NftAuction, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    let auction = if let Some(mut saved_auction) = sqlx::query_as!(
        NftAuction,
        r#"
        select address as "address!: Address", nft as "nft?: Address", price_token as "price_token?: Address", 
            start_price as "start_price?", max_bid as "max_bid?", status as "status?: AuctionStatus", 
            created_at as "created_at?", finished_at as "finished_at?", tx_lt as "tx_lt!"
        from nft_auction where address = $1
        "#,
        &auction.address as &Address
    )
    .fetch_optional(&mut tx)
    .await?
    {
        if auction.tx_lt > saved_auction.tx_lt {
            saved_auction.max_bid = auction.max_bid.clone();
            saved_auction.tx_lt = auction.tx_lt;

            if auction.status.is_some() {
                saved_auction.status = auction.status.clone();
            }

            if auction.created_at.is_some() {
                saved_auction.created_at = auction.created_at;
            }

            if auction.finished_at.is_some() {
                saved_auction.finished_at = auction.finished_at;
            }
        }

        if saved_auction.nft.is_none() {
            saved_auction.nft = auction.nft.clone();
        }

        if saved_auction.price_token.is_none() {
            saved_auction.price_token = auction.price_token.clone();
        }

        if saved_auction.start_price.is_none() {
            saved_auction.start_price = auction.start_price.clone();
        }

        if saved_auction.max_bid.is_none() {
            saved_auction.max_bid = sqlx::query_scalar!(
                r#"
                select max(price) from nft_auction_bid
                where auction = $1 and declined = false
                "#,
                &auction.address as &Address
            )
            .fetch_one(&mut tx)
            .await
            .unwrap_or_default();
        }

        if saved_auction.status.is_none() {
            saved_auction.status = auction.status.clone();
        }

        if saved_auction.created_at.is_none() {
            saved_auction.created_at = auction.created_at;
        }

        if saved_auction.finished_at.is_none() {
            saved_auction.finished_at = auction.finished_at;
        }

        saved_auction
    } else {
        auction.clone()
    };

    sqlx::query!(
        r#"
        insert into nft_auction (address, nft, price_token, start_price, max_bid, status, created_at, finished_at, 
            tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        on conflict (address) where tx_lt < $9 do update
        set nft = $2, price_token = $3, start_price = $4, max_bid = $5, status = $6, created_at = $7, finished_at = $8,
            tx_lt = $9
        "#,
        &auction.address as &Address,
        &auction.nft as &Option<Address>,
        &auction.price_token as &Option<Address>,
        auction.start_price,
        auction.max_bid,
        &auction.status as &Option<AuctionStatus>,
        auction.created_at,
        auction.finished_at,
        auction.tx_lt,
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_bid(bid: &NftAuctionBid, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into nft_auction_bid (auction, buyer, price, declined, created_at)
        values ($1, $2, $3, $4, $5)
        on conflict (auction, buyer, price) where declined = false do update
        set declined = $4
        "#,
        &bid.auction as &Address,
        &bid.buyer as &Address,
        bid.price,
        bid.declined,
        bid.created_at,
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_direct_sell(direct_sell: &NftDirectSell, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into nft_direct_sell (address, nft, price_token, price, state, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7)
        on conflict (address) where tx_lt < $7 do update
        set state = $5, updated = $6, tx_lt = $7
        "#,
        &direct_sell.address as &Address,
        &direct_sell.nft as &Address,
        &direct_sell.price_token as &Address,
        direct_sell.price,
        &direct_sell.state as &DirectSellState,
        direct_sell.updated,
        direct_sell.tx_lt,
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn upsert_direct_buy(direct_buy: &NftDirectBuy, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into nft_direct_buy (address, nft, price_token, price, state, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7)
        on conflict (address) where tx_lt < $7 do update
        set state = $5, updated = $6, tx_lt = $7
        "#,
        &direct_buy.address as &Address,
        &direct_buy.nft as &Address,
        &direct_buy.price_token as &Address,
        direct_buy.price,
        &direct_buy.state as &DirectBuyState,
        direct_buy.updated,
        direct_buy.tx_lt,
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}

pub async fn add_whitelist_address(address: &Address, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        "#,
        address as &Address
    )
    .execute(&mut tx)
    .await?;

    Ok(tx.commit().await?)
}
