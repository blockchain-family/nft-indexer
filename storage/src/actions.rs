use crate::{traits::EventRecord, types::*};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, types::BigDecimal, PgPool, Postgres, Transaction};

pub async fn save_event<T: EventRecord + Serialize>(
    record: &T,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    log::trace!(
        "Trying to save event with message {:?}",
        record.get_message_hash()
    );
    let response = sqlx::query!(
        r#"
        insert into nft_events (event_cat, event_type, address, nft, collection, created_lt, created_at, args, message_hash)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        on conflict (message_hash) do nothing
        "#,
        record.get_event_category() as EventCategory,
        record.get_event_type() as EventType,
        record.get_address() as Address,
        record.get_nft() as Option<Address>,
        record.get_collection() as Option<Address>,
        record.get_created_lt(),
        record.get_created_at(),
        serde_json::to_value(record).unwrap_or_default(),
        record.get_message_hash()
    )
    .execute(tx)
    .await?;

    if response.rows_affected() == 0 {
        log::trace!(
            "Event already present with message_hash {}",
            record.get_message_hash()
        );
    }

    Ok(response)
}

pub async fn get_owners_count(
    collection: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Option<i64> {
    sqlx::query_scalar!(
        r#"
        select count(*) from (
            select distinct owner from nft
            where collection = $1
        ) as owners
        "#,
        collection as &Address,
    )
    .fetch_one(tx)
    .await
    .unwrap_or_default()
}

pub async fn get_prices(
    collection: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(BigDecimal, BigDecimal), sqlx::Error> {
    struct PricePair {
        price: BigDecimal,
        price_token: String,
    }

    let pairs = sqlx::query_as!(
        PricePair,
        r#"
        select price as "price!: BigDecimal", price_token as "price_token!: String" from nft
        inner join nft_direct_sell as direct_sell
        on nft.address = direct_sell.nft
        where nft.collection = $1 and direct_sell.state = 'active'
        union
        select price as "price!: BigDecimal", price_token as "price_token!: String" from nft
        inner join nft_auction as auction
        on nft.address = auction.nft
        inner join nft_auction_bid as bid
        on auction.address = bid.auction
        where nft.collection = $1 and auction.status = 'active'
        "#,
        collection as &Address,
    )
    .fetch_all(tx)
    .await?;

    let mut total_price = BigDecimal::default();
    let mut max_price = BigDecimal::default();
    for pair in pairs {
        let (price, price_token) = (pair.price, pair.price_token);
        let usdt_price = price * rpc::token_to_usd(&price_token).await.unwrap_or_default();

        max_price = std::cmp::max(max_price, usdt_price.clone());
        total_price += usdt_price;
    }

    Ok((total_price, max_price))
}

pub async fn upsert_collection(
    collection: &NftCollection,
    tx: &mut Transaction<'_, Postgres>,
    nft_created_at: Option<NaiveDateTime>,
) -> Result<PgQueryResult, sqlx::Error> {
    let owners_count = get_owners_count(&collection.address, tx).await;
    let (total_price, max_price) = get_prices(&collection.address, tx).await?;

    sqlx::query!(
        r#"
        insert into nft_collection (address, owner, name, description, created, updated, logo, wallpaper,
            total_price, max_price, owners_count, first_mint)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        on conflict (address) do update
        set owner = $2, name = coalesce($3, nft_collection.name), 
            description = coalesce($4, nft_collection.description), 
            created = case when nft_collection.created < $5 then nft_collection.created else $5 end, updated = $6,
            logo = coalesce($7, nft_collection.logo), wallpaper = coalesce($8, nft_collection.wallpaper), total_price = $9,
            max_price = $10, owners_count = $11, first_mint = least($12, nft_collection.first_mint)
        "#,
        &collection.address as &Address,
        &collection.owner as &Address,
        collection.name,
        collection.description,
        collection.created,
        collection.updated,
        &collection.logo as &Option<Uri>,
        &collection.wallpaper as &Option<Uri>,
        total_price,
        max_price,
        owners_count.unwrap_or_default() as i32,
        nft_created_at
    )
    .execute(tx)
    .await
}

pub async fn upsert_nft_meta(
    nft_meta: &NftMeta,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft_metadata (nft, meta, updated)
        values ($1, $2, $3)
        on conflict (nft) where updated < $3 do update
        set meta = coalesce($2, nft_metadata.meta), updated = $3
        "#,
        &nft_meta.nft as &Address,
        nft_meta.meta,
        nft_meta.updated
    )
    .execute(tx)
    .await
}

pub async fn upsert_nft_meta_columns(
    address: &str,
    name: &str,
    description: &str,
    updated: NaiveDateTime,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            update nft
            set name = $1, description = $2, updated = $3
            where address = $4
        "#,
        name,
        description,
        updated,
        address
    )
    .execute(tx)
    .await
}

pub async fn upsert_nft_attributes(
    nft_attributes: &[NftAttribute],
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    for nft_attribute in nft_attributes.iter() {
        sqlx::query!(
            r#"
            insert into nft_attributes (nft, collection, raw, trait_type, value)
            values ($1, $2, $3, $4, $5)
            "#,
            &nft_attribute.nft as &Address,
            &nft_attribute.collection as &Option<Address>,
            nft_attribute.raw,
            nft_attribute.trait_type,
            nft_attribute.value,
        )
        .execute(&mut *tx)
        .await?;
    }

    Ok(())
}

pub async fn update_collection_by_nft(
    table_name: &str,
    nft: &Address,
    collection: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let query = format!(
        r#"
        update {table_name} set collection = '{}' where nft = '{}'
        "#,
        collection.0, nft.0,
    );

    sqlx::query(&query).execute(tx).await
}

pub async fn update_nft_by_auction(
    table_name: &str,
    auction: &Address,
    nft: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let query = format!(
        r#"
        update {table_name} set nft = '{}' where address = '{}'
        "#,
        nft.0, auction.0,
    );

    sqlx::query(&query).execute(tx).await
}

pub async fn upsert_nft(
    nft: &Nft,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let nft = if let Some(mut saved_nft) = sqlx::query_as!(
        Nft,
        r#"
        select address as "address!: Address", collection as "collection?: Address", owner as "owner?: Address", 
            manager as "manager?: Address", name as "name?", description as "description?", burned as "burned!", 
            updated as "updated!", owner_update_lt as "owner_update_lt!", manager_update_lt as "manager_update_lt!"
        from nft where address = $1
        "#,
        &nft.address as &Address
    )
    .fetch_optional(&mut *tx)
    .await?
    {
        if saved_nft.owner.is_none() || (saved_nft.owner_update_lt <= nft.owner_update_lt && nft.owner.is_some()) {
            saved_nft.owner = nft.owner.clone();
            saved_nft.owner_update_lt = nft.owner_update_lt;
        }

        if saved_nft.manager.is_none() || (saved_nft.manager_update_lt <= nft.manager_update_lt && nft.manager.is_some()) {
            saved_nft.manager = nft.manager.clone();
            saved_nft.manager_update_lt = nft.manager_update_lt;
        }

        if nft.collection.is_some() {
            saved_nft.collection = nft.collection.clone();
        }

        if nft.name.is_some() {
            saved_nft.name = nft.name.clone();
        }

        if nft.description.is_some() {
            saved_nft.description = nft.description.clone();
        }

        saved_nft.burned |= nft.burned;
        saved_nft.updated = nft.updated;

        saved_nft
    } else {
        nft.clone()
    };

    sqlx::query!(
        r#"
        insert into nft (address, collection, owner, manager, name, description, burned, updated, owner_update_lt,
            manager_update_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        on conflict (address) do update
        set collection = coalesce($2, nft.collection), owner = $3, manager = $4, name = coalesce($5, nft.name),
            description = coalesce($6, nft.description), burned = nft.burned or $7, updated = $8, owner_update_lt = $9,
            manager_update_lt = $10
        "#,
        nft.address as Address,
        nft.collection as Option<Address>,
        nft.owner as Option<Address>,
        nft.manager as Option<Address>,
        nft.name,
        nft.description,
        nft.burned,
        nft.updated,
        nft.owner_update_lt,
        nft.manager_update_lt,
    )
    .execute(tx)
    .await
}

pub async fn upsert_auction(
    auction: &NftAuction,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let auction = if let Some(mut saved_auction) = sqlx::query_as!(
        NftAuction,
        r#"
        select address as "address!: Address", nft as "nft?: Address", wallet_for_bids as "wallet_for_bids?: Address",
            price_token as "price_token?: Address", start_price as "start_price?", 
            closing_price_usd as "closing_price_usd?", min_bid as "min_bid?", max_bid as "max_bid?",
            status as "status?: AuctionStatus", created_at as "created_at?", finished_at as "finished_at?",
            tx_lt as "tx_lt!"
        from nft_auction where address = $1
        "#,
        &auction.address as &Address
        )
        .fetch_optional(&mut *tx)
        .await?
        {
            if saved_auction.nft.is_none() {
                saved_auction.nft = auction.nft.clone();
            }

            if saved_auction.wallet_for_bids.is_none() {
                saved_auction.wallet_for_bids = auction.wallet_for_bids.clone();
            }

            if saved_auction.price_token.is_none() {
                saved_auction.price_token = auction.price_token.clone();
            }

            if saved_auction.start_price.is_none() {
                saved_auction.start_price = auction.start_price.clone();
            }

            if saved_auction.closing_price_usd.is_none() {
                saved_auction.closing_price_usd = auction.closing_price_usd.clone();
            }

            if saved_auction.min_bid.is_none() || (saved_auction.tx_lt <= auction.tx_lt && auction.min_bid.is_some()) {
                saved_auction.min_bid = auction.min_bid.clone();
            }

            if saved_auction.max_bid.is_none() || (saved_auction.tx_lt <= auction.tx_lt && auction.max_bid.is_some()) {
                saved_auction.max_bid = auction.max_bid.clone();
            }

            if saved_auction.status.is_none() || (saved_auction.tx_lt <= auction.tx_lt && auction.status.is_some()) {
                saved_auction.status = auction.status.clone();
            }

            if saved_auction.created_at.is_none() {
                saved_auction.created_at = auction.created_at;
            }

            if saved_auction.finished_at.is_none() {
                saved_auction.finished_at = auction.finished_at;
            }

            saved_auction.tx_lt = std::cmp::max(saved_auction.tx_lt, auction.tx_lt);

            saved_auction
        } else {
            auction.clone()
        };

    sqlx::query!(
        r#"
        insert into nft_auction (address, nft, wallet_for_bids, price_token, start_price, closing_price_usd, min_bid,
            max_bid, status, created_at, finished_at, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        on conflict (address) do update
        set nft = $2, wallet_for_bids = $3, price_token = $4, start_price = $5, closing_price_usd = $6, min_bid = $7,
            max_bid = $8, status = $9, created_at = $10, finished_at = $11, tx_lt = $12
        "#,
        &auction.address as &Address,
        &auction.nft as &Option<Address>,
        &auction.wallet_for_bids as &Option<Address>,
        &auction.price_token as &Option<Address>,
        auction.start_price,
        auction.closing_price_usd,
        auction.min_bid,
        auction.max_bid,
        &auction.status as &Option<AuctionStatus>,
        auction.created_at,
        auction.finished_at,
        auction.tx_lt,
        )
        .execute(tx)
        .await
}

pub async fn insert_auction_bid(
    bid: &NftAuctionBid,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft_auction_bid (auction, buyer, price, next_bid_value, declined, created_at, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
        &bid.auction as &Address,
        &bid.buyer as &Address,
        bid.price,
        bid.next_bid_value,
        bid.declined,
        bid.created_at,
        bid.tx_lt,
    )
    .execute(tx)
    .await
}

pub async fn upsert_direct_sell(
    direct_sell: &NftDirectSell,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft_direct_sell (address, nft, collection, price_token, price, sell_price_usd, seller, finished_at,
            expired_at, state, created, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        on conflict (address) where tx_lt <= $13 do update
        set collection = $3, price = $5, sell_price_usd = coalesce($6, nft_direct_sell.sell_price_usd),
            finished_at = $8, expired_at = $9, state = $10, created = $11, updated = $12, tx_lt = $13
        "#,
        &direct_sell.address as &Address,
        &direct_sell.nft as &Address,
        &direct_sell.collection as &Option<Address>,
        &direct_sell.price_token as &Address,
        direct_sell.price,
        direct_sell.sell_price_usd,
        &direct_sell.seller as &Address,
        direct_sell.finished_at,
        direct_sell.expired_at,
        &direct_sell.state as &DirectSellState,
        direct_sell.created,
        direct_sell.updated,
        direct_sell.tx_lt,
    )
    .execute(tx)
    .await
}

pub async fn upsert_direct_buy(
    direct_buy: &NftDirectBuy,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft_direct_buy (address, nft, collection, price_token, price, buy_price_usd, buyer, finished_at,
            expired_at, state, created, updated, tx_lt)    
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        on conflict (address) where tx_lt <= $13 do update
        set collection = $3, price = $5, buy_price_usd = coalesce($6, nft_direct_buy.buy_price_usd), finished_at = $8,
            expired_at = $9, state = $10, created = $11, updated = $12, tx_lt = $13
        "#,
        &direct_buy.address as &Address,
        &direct_buy.nft as &Address,
        &direct_buy.collection as &Option<Address>,
        &direct_buy.price_token as &Address,
        direct_buy.price,
        direct_buy.buy_price_usd,
        &direct_buy.buyer as &Address,
        direct_buy.finished_at,
        direct_buy.expired_at,
        &direct_buy.state as &DirectBuyState,
        direct_buy.created,
        direct_buy.updated,
        direct_buy.tx_lt,
    )
    .execute(tx)
    .await
}

pub async fn upsert_nft_price_history(
    price_history: &NftPriceHistory,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft_price_history (source, source_type, ts, price, price_token, nft, collection)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
        &price_history.source as &Address,
        &price_history.source_type as &NftPriceSource,
        price_history.created_at,
        price_history.price,
        &price_history.price_token as &Option<Address>,
        &price_history.nft as &Option<Address>,
        &price_history.collection as &Option<Address>,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        update nft_price_history as nph
        set price_token = coalesce(nph.price_token, $2), nft = coalesce(nph.nft, $3), 
            collection = coalesce(nph.collection, $4)
        where source = $1
        "#,
        &price_history.source as &Address,
        &price_history.price_token as &Option<Address>,
        &price_history.nft as &Option<Address>,
        &price_history.collection as &Option<Address>,
    )
    .execute(tx)
    .await
}

pub async fn get_collection_by_nft(
    nft: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Option<Address> {
    sqlx::query_scalar!(
        r#"
        select collection as "collection?: Address" from nft
        where nft.address = $1
        "#,
        nft as &Address
    )
    .fetch_one(tx)
    .await
    .unwrap_or(None)
    .filter(|a| !a.0.is_empty())
}

pub async fn get_nft_and_collection_by_auction(
    auction: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> (Option<Address>, Option<Address>) {
    #[derive(Default)]
    struct NftCollectionPair {
        nft: Option<Address>,
        collection: Option<Address>,
    }

    let pair = sqlx::query_as!(
        NftCollectionPair,
        r#"
        select nft.address as "nft?: Address", collection as "collection?: Address" from nft
        inner join nft_auction
        on nft_auction.nft = nft.address
        where nft_auction.address = $1
        "#,
        auction as &Address
    )
    .fetch_one(tx)
    .await
    .unwrap_or_default();

    (
        pair.nft.filter(|a| !a.0.is_empty()),
        pair.collection.filter(|a| !a.0.is_empty()),
    )
}

pub async fn get_auction_price_token(
    auction: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Option<Address> {
    sqlx::query_scalar!(
        r#"
        select price_token as "price_token?: Address" from nft_auction
        where address = $1
        "#,
        auction as &Address
    )
    .fetch_one(tx)
    .await
    .unwrap_or(None)
    .filter(|a| !a.0.is_empty())
}

pub async fn add_whitelist_address(
    address: &Address,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        "#,
        address as &Address
    )
    .execute(pool)
    .await
}

pub async fn update_offers_status(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let now = chrono::Utc::now().naive_utc();
    let begin_of_epoch = NaiveDateTime::default();

    sqlx::query!(
        r#"
        update nft_auction set status = $1
        where finished_at != $2 and finished_at < $3 and nft_auction.status = $4
        "#,
        AuctionStatus::Expired as AuctionStatus,
        begin_of_epoch,
        now,
        AuctionStatus::Active as AuctionStatus,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        r#"
        update nft_direct_sell set state = $1
        where expired_at != $2 and expired_at < $3 and nft_direct_sell.state = $4
        "#,
        DirectSellState::Expired as DirectSellState,
        begin_of_epoch,
        now,
        DirectSellState::Active as DirectSellState,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        r#"
        update nft_direct_buy set state = $1
        where expired_at != $2 and expired_at < $3 and nft_direct_buy.state = $4
        "#,
        DirectBuyState::Expired as DirectBuyState,
        begin_of_epoch,
        now,
        DirectBuyState::Active as DirectBuyState,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await
}
