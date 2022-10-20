use crate::{traits::EventRecord, types::*};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{types::BigDecimal, PgPool, Postgres, QueryBuilder};
use std::str::FromStr;

pub async fn save_event<T: EventRecord + Serialize>(record: &T, pool: &PgPool) -> Result<()> {
    Ok(sqlx::query!(
        r#"
        insert into nft_events (event_cat, event_type, address, nft, collection, created_lt, created_at, args)
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        record.get_event_category() as EventCategory,
        record.get_event_type() as EventType,
        record.get_address() as Address,
        record.get_nft() as Option<Address>,
        record.get_collection() as Option<Address>,
        record.get_created_lt(),
        record.get_created_at(),
        serde_json::to_value(record)?,
    )
    .execute(pool)
    .await.map(|_| {})?)
}

const USDT_TOKEN_ROOT: &str = "0:a519f99bb5d6d51ef958ed24d337ad75a1c770885dcd42d51d6663f9fcdacfb2";

pub async fn get_owners_count(collection: &Address, pool: &PgPool) -> Result<Option<i64>> {
    Ok(sqlx::query_scalar!(
        r#"
        select count(*) from (
            select distinct owner from nft
            where collection = $1
        ) as owners
        "#,
        collection as &Address,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn token_to_usdt(token: &str) -> Result<BigDecimal> {
    let request_body = format!(
        r#"
        {{
            "fromCurrencyAddress": "{token}",
            "toCurrencyAddresses": ["{USDT_TOKEN_ROOT}"]
        }}
        "#
    );

    let client = reqwest::Client::new();
    let response = rpc::retrier::Retrier::new(move || {
        let request = client
            .post("https://api.flatqube.io/v1/pairs/cross_pairs")
            .body(request_body.clone());
        Box::pin(request.send())
    })
    .attempts(5)
    .backoff(50)
    .factor(2)
    .trace_id(format!("usdt price for {}", token))
    .run()
    .await?;

    let object: serde_json::Value = serde_json::from_slice(&response.bytes().await?)?;
    let usdt = object
        .get("pairs")
        .ok_or_else(|| anyhow!("No 'pairs' in response"))?
        .as_array()
        .ok_or_else(|| anyhow!("Error converting response to array"))?
        .first()
        .ok_or_else(|| anyhow!("Response is empty"))?
        .as_object()
        .ok_or_else(|| anyhow!("Couldn't convert data to object"))?
        .get("leftPrice")
        .ok_or_else(|| anyhow!("Couldn't get token price in USDT"))?;

    Ok(BigDecimal::from_str(usdt.as_str().ok_or_else(|| {
        anyhow!("Couldn't convert value to str")
    })?)?)
}

pub async fn get_prices(collection: &Address, pool: &PgPool) -> Result<(BigDecimal, BigDecimal)> {
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
    .fetch_all(pool)
    .await?;

    let mut total_price = BigDecimal::default();
    let mut max_price = BigDecimal::default();
    for pair in pairs {
        let (price, price_token) = (pair.price, pair.price_token);
        let usdt_price = price * token_to_usdt(&price_token).await?;

        max_price = std::cmp::max(max_price, usdt_price.clone());
        total_price += usdt_price;
    }

    Ok((total_price, max_price))
}

pub async fn upsert_collection(collection: &NftCollection, pool: &PgPool) -> Result<()> {
    let owners_count = get_owners_count(&collection.address, pool)
        .await?
        .unwrap_or_default();
    let (total_price, max_price) = get_prices(&collection.address, pool).await?;

    sqlx::query!(
        r#"
        insert into nft_collection (address, owner, name, description, created, updated, logo, wallpaper,
            total_price, max_price, owners_count)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (address) where updated <= $6 do update
        set owner = $2, name = coalesce($3, nft_collection.name), 
            description = coalesce($4, nft_collection.description), 
            created = case when nft_collection.created < $5 then nft_collection.created else $5 end, updated = $6,
            logo = coalesce($7, nft_collection.logo), wallpaper = coalesce($8, nft_collection.wallpaper), total_price = $9,
            max_price = $10, owners_count = $11
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
        owners_count as i32,
    )
    .execute(pool)
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", collection)) }).map(|_| {})
}

pub async fn upsert_nft_meta(nft_meta: &NftMeta, pool: &PgPool) -> Result<()> {
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
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e).context(format!("value: {:#?}", nft_meta)))
    .map(|_| {})
}

pub async fn upsert_nft_attributes(
    nft_attributes: &Vec<NftAttribute>,
    pool: &PgPool,
) -> Result<()> {
    let mut query = QueryBuilder::<Postgres>::new(
        "insert into nft_attributes (nft, collection, raw, trait_type, value) ",
    );

    query.push_values(nft_attributes, |mut b, nft_attribute| {
        b.push_bind(&nft_attribute.nft)
            .push_bind(&nft_attribute.collection)
            .push_bind(&nft_attribute.raw)
            .push_bind(&nft_attribute.trait_type)
            .push_bind(&nft_attribute.value);
    });

    query
        .build()
        .execute(pool)
        .await
        .map_err(|e| anyhow!(e).context(format!("value: {:#?}", nft_attributes)))
        .map(|_| {})
}

pub async fn update_collection_by_nft(
    table_name: &str,
    nft: &Address,
    collection: &Address,
    pool: &PgPool,
) -> Result<()> {
    let query = format!(
        r#"
        update {table_name} set collection = '{}' where nft = '{}'
        "#,
        collection.0, nft.0,
    );

    Ok(sqlx::query(&query).execute(pool).await.map(|_| {})?)
}

pub async fn update_nft_by_auction(
    table_name: &str,
    auction: &Address,
    nft: &Address,
    pool: &PgPool,
) -> Result<()> {
    let query = format!(
        r#"
        update {table_name} set nft = '{}' where address = '{}'
        "#,
        nft.0, auction.0,
    );

    Ok(sqlx::query(&query).execute(pool).await.map(|_| {})?)
}

pub async fn upsert_nft(nft: &Nft, pool: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        insert into nft (address, collection, owner, manager, name, description, burned, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        on conflict (address) where tx_lt <= $9 do update
        set collection = coalesce($2, nft.collection), owner = coalesce($3, nft.owner),
            manager = coalesce($4, nft.manager), name = coalesce($5, nft.name), 
            description = coalesce($6, nft.description), burned = $7 or nft.burned, updated = $8, tx_lt = $9
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
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", nft)) }).map(|_| {})
}

pub async fn upsert_auction(auction: &NftAuction, pool: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        insert into nft_auction (address, nft, wallet_for_bids, price_token, start_price, min_bid, max_bid, status,
            created_at, finished_at, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (address) where tx_lt <= $11 do update
        set nft = coalesce($2, nft_auction.nft), wallet_for_bids = coalesce($3, nft_auction.wallet_for_bids),
            price_token = coalesce($4, nft_auction.price_token), start_price = coalesce($5, nft_auction.start_price),
            min_bid = coalesce($6, nft_auction.min_bid), max_bid = coalesce($7, nft_auction.max_bid),
            status = coalesce($8, nft_auction.status), created_at = coalesce($9, nft_auction.created_at), 
            finished_at = coalesce($10, nft_auction.finished_at), tx_lt = $11
        "#,
        &auction.address as &Address,
        &auction.nft as &Option<Address>,
        &auction.wallet_for_bids as &Option<Address>,
        &auction.price_token as &Option<Address>,
        auction.start_price,
        auction.min_bid,
        auction.max_bid,
        &auction.status as &Option<AuctionStatus>,
        auction.created_at,
        auction.finished_at,
        auction.tx_lt,
    )
    .execute(pool)
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", auction)) }).map(|_| {})
}

pub async fn upsert_bid(bid: &NftAuctionBid, pool: &PgPool) -> Result<()> {
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
    .execute(pool)
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", bid)) })
    .map(|_| {})
}

pub async fn upsert_direct_sell(direct_sell: &NftDirectSell, pool: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        insert into nft_direct_sell (address, nft, collection, price_token, price, seller, finished_at, expired_at,
            state, created, updated, tx_lt)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        on conflict (address) where tx_lt <= $12 do update
        set collection = $3, price = $5, finished_at = $7, expired_at = $8, 
        state = case when nft_direct_sell.state = 'expired' then 'expired' else $9 end, created = $10, updated = $11,
            tx_lt = $12
        "#,
        &direct_sell.address as &Address,
        &direct_sell.nft as &Address,
        &direct_sell.collection as &Option<Address>,
        &direct_sell.price_token as &Address,
        direct_sell.price,
        &direct_sell.seller as &Address,
        direct_sell.finished_at,
        direct_sell.expired_at,
        &direct_sell.state as &DirectSellState,
        direct_sell.created,
        direct_sell.updated,
        direct_sell.tx_lt,
    )
    .execute(pool)
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", direct_sell)) }).map(|_| {})
}

pub async fn upsert_direct_buy(direct_buy: &NftDirectBuy, pool: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        insert into nft_direct_buy (address, nft, collection, price_token, price, buyer, finished_at, expired_at,
            state, created, updated, tx_lt)    
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        on conflict (address) where tx_lt <= $12 do update
        set collection = $3, price = $5, finished_at = $7, expired_at = $8, 
        state = case when nft_direct_buy.state = 'expired' then 'expired' else $9 end, created = $10, updated = $11,
            tx_lt = $12
        "#,
        &direct_buy.address as &Address,
        &direct_buy.nft as &Address,
        &direct_buy.collection as &Option<Address>,
        &direct_buy.price_token as &Address,
        direct_buy.price,
        &direct_buy.buyer as &Address,
        direct_buy.finished_at,
        direct_buy.expired_at,
        &direct_buy.state as &DirectBuyState,
        direct_buy.created,
        direct_buy.updated,
        direct_buy.tx_lt,
    )
    .execute(pool)
    .await.map_err(|e| { anyhow!(e).context(format!("value: {:#?}", direct_buy)) }).map(|_| {})
}

pub async fn upsert_nft_price_history(
    price_history: &NftPriceHistory,
    pool: &PgPool,
) -> Result<()> {
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
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e).context(format!("value: {:#?}", price_history)))?;

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
    .execute(pool)
    .await
    .map_err(|e| anyhow!(e).context(format!("value: {:#?}", price_history)))
    .map(|_| {})
}

pub async fn get_collection_by_nft(nft: &Address, pool: &PgPool) -> Option<Address> {
    sqlx::query_scalar!(
        r#"
        select collection from nft
        where nft.address = $1
        "#,
        nft as &Address
    )
    .fetch_one(pool)
    .await
    .unwrap_or_default()
    .map(|s| s.into())
}

pub async fn get_nft_and_collection_by_auction(
    auction: &Address,
    pool: &PgPool,
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
    .fetch_one(pool)
    .await
    .unwrap_or_default();

    (pair.nft, pair.collection)
}

pub async fn add_whitelist_address(address: &Address, pool: &PgPool) -> Result<()> {
    Ok(sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        "#,
        address as &Address
    )
    .execute(pool)
    .await
    .map(|_| {})?)
}

pub async fn update_offers_status(pool: &PgPool) -> Result<()> {
    let now = chrono::Utc::now().naive_utc();
    let begin_of_epoch = NaiveDateTime::default();

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
    .execute(pool)
    .await?;

    Ok(sqlx::query!(
        r#"
        update nft_direct_buy set state = $1
        where expired_at != $2 and expired_at < $3 and nft_direct_buy.state = $4
        "#,
        DirectBuyState::Expired as DirectBuyState,
        begin_of_epoch,
        now,
        DirectBuyState::Active as DirectBuyState,
    )
    .execute(pool)
    .await
    .map(|_| {})?)
}
