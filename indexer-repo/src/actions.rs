use crate::types::*;
use chrono::NaiveDateTime;
use sqlx::{postgres::PgQueryResult, types::BigDecimal, PgPool, Postgres, Transaction};

pub async fn save_event(
    record: &EventRecord,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    log::debug!(
        "Trying to save event with message {:?}",
        record.message_hash
    );
    let response = sqlx::query!(
        r#"
        insert into nft_events (
            event_cat,  event_type, 
            address, 
            nft,        collection, 
            created_lt, created_at, 
            args, 
            message_hash
        )
        values (
            $1,         $2, 
            $3, 
            $4,         $5, 
            $6,         $7, 
            $8, 
            $9
        )
        on conflict (message_hash) do nothing
        "#,
        record.event_category as _,
        record.event_type as _,
        record.address as _,
        record.nft as _,
        record.collection as _,
        record.created_lt,
        record.created_at,
        record.raw_data,
        record.message_hash as _
    )
    .execute(tx)
    .await?;

    if response.rows_affected() == 0 {
        log::warn!(
            "Event already present with message_hash {:?} {:?}",
            record.message_hash,
            record.raw_data,
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
        select 
            count(distinct owner) as owners
        from 
            nft
        where 
            collection = $1
        "#,
        collection as _,
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
        select sum(ag.price)  as "price!: BigDecimal", ag.price_token as "price_token!: String" from (
                            select price, price_token
                            from nft
                                     inner join nft_direct_sell as direct_sell
                                                on nft.address = direct_sell.nft
                            where nft.collection = $1
                              and direct_sell.state = 'active'
                            union
                            select price as "price!: BigDecimal", price_token as "price_token!: String"
                            from nft
                                     inner join nft_auction as auction
                                                on nft.address = auction.nft
                                     inner join nft_auction_bid as bid
                                                on auction.address = bid.auction
                            where nft.collection = $1
                              and auction.status = 'active'
                        ) ag
        group by ag.price_token
        "#,
        collection as _,
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
        insert into nft_collection (
            address, 
            owner, 
            name,         description, 
            created,      updated, 
            logo,         wallpaper,
            total_price,  max_price, 
            owners_count, 
            first_mint
        )
        values (
            $1, 
            $2, 
            $3,           $4, 
            $5,           $6, 
            $7,           $8, 
            $9,           $10, 
            $11, 
            $12
        )
        on conflict (address) do update
        set 
            owner        = $2, 
            name         = coalesce($3, nft_collection.name),
            description  = coalesce($4, nft_collection.description), 
            created      = case when nft_collection.created < $5 then nft_collection.created else $5 end, 
            updated      = $6,
            logo         = coalesce($7, nft_collection.logo),
            wallpaper    = coalesce($8, nft_collection.wallpaper), 
            total_price  = $9,
            max_price    = $10, 
            owners_count = $11, 
            first_mint   = least($12, nft_collection.first_mint)
        "#,
        &collection.address as _,
        &collection.owner as _,
        collection.name,
        collection.description,
        collection.created,
        collection.updated,
        &collection.logo as _,
        &collection.wallpaper as _,
        total_price,
        max_price,
        owners_count.unwrap_or_default() as _,
        nft_created_at
    )
    .execute(tx)
    .await
}
pub async fn refresh_collection_owners_count(
    address: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let owners_count = get_owners_count(address, tx).await;

    sqlx::query!(
        r#"
        update nft_collection
        set owners_count = $2
        where address = $1
        "#,
        address as _,
        owners_count.unwrap_or_default() as _,
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
        insert into nft_metadata (
            nft, 
            meta, 
            updated
        )
        values (
            $1, 
            $2, 
            $3
        )
        on conflict (nft) where updated < $3 do update
        set 
            meta    = coalesce($2, nft_metadata.meta), 
            updated = $3
        "#,
        &nft_meta.nft as _,
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
            set 
                name        = $1, 
                description = $2, 
                updated     = $3
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
            insert into nft_attributes (
                nft,        collection, 
                raw, 
                trait_type, 
                value
            )
            values (
                $1,         $2, 
                $3, 
                $4, 
                $5
            )
            "#,
            &nft_attribute.nft as _,
            &nft_attribute.collection as _,
            nft_attribute.raw,
            nft_attribute.trait_type,
            nft_attribute.value,
        )
        .execute(&mut *tx)
        .await?;
    }

    Ok(())
}

pub async fn upsert_nft_info(
    owner: &Address,
    manager: &Address,
    nft: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            update nft
            set 
                owner = $1, 
                manager = $2
            where address = $3
        "#,
        owner as _,
        manager as _,
        nft as _,
    )
    .execute(&mut *tx)
    .await?;

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
        update {table_name} 
        set collection = '{}' 
        where nft = '{}'
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
        update {table_name} 
        set nft = '{}' 
        where address = '{}'
        "#,
        nft.0, auction.0,
    );

    sqlx::query(&query).execute(tx).await
}

pub async fn upsert_nft(
    nft: &Nft,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into nft (
            address, 
            collection, 
            owner,           manager, 
            name,            description, 
            burned, 
            updated, 
            owner_update_lt, manager_update_lt
        )
        values (
            $1, 
            $2, 
            $3,             $4, 
            $5,             $6, 
            $7, 
            $8, 
            $9,             $10)
        on conflict (address) do update
        set 
            collection        = coalesce($2, nft.collection), 
            owner             = case when 
                                         (nft.owner is null) or ((nft.owner_update_lt <= $9) and ($3 is not null))
                                     then 
                                         $3
                                     else 
                                         nft.owner 
                                     end,
                manager       = case when 
                                         (nft.manager is null) or ((nft.manager_update_lt <= $10) and ($4 is not null))
                                     then 
                                         $4
                                     else 
                                         nft.owner 
                                     end, 
            name              = coalesce($5, nft.name),
            description       = coalesce($6, nft.description), 
            burned            = nft.burned or $7, 
            updated           = $8, 
            owner_update_lt   = case when 
                                         (nft.owner is null) or ((nft.owner_update_lt <= $9) and ($3 is not null))
                                     then 
                                         $9
                                     else 
                                         nft.owner_update_lt 
                                     end,
            manager_update_lt = case when 
                                         (nft.manager is null) or ((nft.manager_update_lt <= $10) and ($4 is not null))
                                     then 
                                         $10
                                     else 
                                         nft.manager_update_lt 
                                     end
        "#,
        &nft.address as _,
        &nft.collection as _,
        &nft.owner as _,
        &nft.manager as _,
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
    sqlx::query!(
        r#"
        insert into nft_auction (
            address, 
            nft, 
            wallet_for_bids, 
            price_token,     start_price, closing_price_usd, 
            min_bid,         max_bid, 
            status, 
            created_at,      finished_at, 
            tx_lt
        )
        values (
            $1, 
            $2, 
            $3, 
            $4,              $5,          $6, 
            $7,              $8, 
            $9, 
            $10,             $11, 
            $12
        )
        on conflict (address) do update
        set 
            nft = coalesce($2, nft_auction.nft), 
            wallet_for_bids   = coalesce($3, nft_auction.wallet_for_bids), 
            price_token       = coalesce($4, nft_auction.price_token), 
            start_price       = coalesce($5, nft_auction.start_price), 
            closing_price_usd = coalesce($6, nft_auction.closing_price_usd),
            min_bid           = case when 
                                    (nft_auction.min_bid is null) or ((nft_auction.tx_lt <= $12) and ($7 is not null))
                                then 
                                    $7
                                else 
                                    nft_auction.min_bid 
                                end,
            max_bid           = case when 
                                    (nft_auction.max_bid is null) or ((nft_auction.tx_lt <= $12) and ($8 is not null))
                                then 
                                    $8
                                else 
                                    nft_auction.max_bid 
                                end, 
            status            = case when 
                                    (nft_auction.status is null) or ((nft_auction.tx_lt <= $12) and ($9 is not null))
                                then 
                                    $9
                                else 
                                    nft_auction.status 
                                end,
            created_at        = coalesce($10, nft_auction.created_at), 
            finished_at       = coalesce($11, nft_auction.finished_at), 
            tx_lt             = greatest($12, nft_auction.tx_lt)
        "#,
        &auction.address as _,
        &auction.nft as _,
        &auction.wallet_for_bids as _,
        &auction.price_token as _,
        auction.start_price,
        auction.closing_price_usd,
        auction.min_bid,
        auction.max_bid,
        &auction.status as _,
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
        insert into nft_auction_bid (
            auction,
            buyer,
            price,      next_bid_value, 
            declined, 
            created_at,
            tx_lt
        )
        values (
            $1, 
            $2, 
            $3,         $4, 
            $5, 
            $6, 
            $7
        )
        "#,
        &bid.auction as _,
        &bid.buyer as _,
        bid.price,
        bid.next_bid_value,
        bid.declined,
        bid.created_at,
        bid.tx_lt,
    )
    .execute(tx)
    .await
}

pub async fn update_collection_fee(
    numerator: Option<i32>,
    denominator: Option<i32>,
    collection_address: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        update nft_collection
        set 
            fee_numerator   = $1, 
            fee_denominator = $2
        where address = $3
        "#,
        numerator,
        denominator,
        collection_address as _
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
        insert into nft_direct_sell (
            address, 
            nft,         collection, 
            price_token, price,      sell_price_usd, 
            seller, 
            finished_at, expired_at, 
            state, 
            created,     updated, 
            tx_lt
        )
        values (
            $1, 
            $2,          $3, 
            $4,          $5,         $6, 
            $7, 
            $8,          $9, 
            $10, 
            $11,         $12, 
            $13
        )
        on conflict (address) where tx_lt <= $13 do update
        set 
            collection     = $3, 
            price          = $5, 
            sell_price_usd = coalesce($6, nft_direct_sell.sell_price_usd),
            finished_at    = $8, 
            expired_at     = $9, 
            state          = $10, 
            created        = $11, 
            updated        = $12, 
            tx_lt          = $13
        "#,
        &direct_sell.address as _,
        &direct_sell.nft as _,
        &direct_sell.collection as _,
        &direct_sell.price_token as _,
        direct_sell.price,
        direct_sell.sell_price_usd,
        &direct_sell.seller as _,
        direct_sell.finished_at,
        direct_sell.expired_at,
        &direct_sell.state as _,
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
        insert into nft_direct_buy (
            address, 
            nft,         collection, 
            price_token, price,      buy_price_usd, 
            buyer, 
            finished_at, expired_at, 
            state, 
            created,     updated, 
            tx_lt
        )    
        values (
            $1, 
            $2,          $3, 
            $4,          $5,         $6, 
            $7, 
            $8,          $9, 
            $10, 
            $11,         $12, 
            $13
        )
        on conflict (address) where tx_lt <= $13 do update
        set 
            collection    = $3, 
            price         = $5, 
            buy_price_usd = coalesce($6, nft_direct_buy.buy_price_usd), 
            finished_at   = $8,
            expired_at    = $9, 
            state         = $10, 
            created       = $11, 
            updated       = $12, 
            tx_lt         = $13
        "#,
        &direct_buy.address as _,
        &direct_buy.nft as _,
        &direct_buy.collection as _,
        &direct_buy.price_token as _,
        direct_buy.price,
        direct_buy.buy_price_usd,
        &direct_buy.buyer as _,
        direct_buy.finished_at,
        direct_buy.expired_at,
        &direct_buy.state as _,
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
        insert into nft_price_history (
            source, source_type, 
            ts, 
            price,  price_token, 
            nft,    collection
        )
        values (
            $1,     $2, 
            $3, 
            $4,     $5, 
            $6,     $7
        )
        "#,
        &price_history.source as _,
        &price_history.source_type as _,
        price_history.created_at,
        price_history.price,
        &price_history.price_token as _,
        &price_history.nft as _,
        &price_history.collection as _,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        update 
            nft_price_history as nph
        set 
            price_token = coalesce(nph.price_token, $2), 
            nft         = coalesce(nph.nft, $3), 
            collection  = coalesce(nph.collection, $4)
        where source = $1
        "#,
        &price_history.source as _,
        &price_history.price_token as _,
        &price_history.nft as _,
        &price_history.collection as _,
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
        select collection as "collection?: Address" 
        from nft
        where nft.address = $1
        "#,
        nft as _,
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
        select 
            nft.address as "nft?: Address", 
            collection as "collection?: Address" 
        from nft
        inner join nft_auction
        on nft_auction.nft = nft.address
        where nft_auction.address = $1
        "#,
        auction as _,
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
        select price_token as "price_token?: Address" 
        from nft_auction
        where address = $1
        "#,
        auction as _,
    )
    .fetch_one(tx)
    .await
    .unwrap_or(None)
    .filter(|a| !a.0.is_empty())
}

pub async fn add_whitelist_address(
    address: &Address,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into events_whitelist (address)
        values ($1)
        on conflict (address) do nothing
        "#,
        address as _,
    )
    .execute(tx)
    .await
}

pub async fn update_offers_status(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let now = chrono::Utc::now().naive_utc();
    let begin_of_epoch = NaiveDateTime::default();

    sqlx::query!(
        r#"
        update nft_auction 
        set status = $1
        where finished_at != $2 and finished_at < $3 and nft_auction.status = $4
        "#,
        AuctionStatus::Expired as _,
        begin_of_epoch,
        now,
        AuctionStatus::Active as _,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        r#"
        update nft_direct_sell 
        set state = $1
        where expired_at != $2 and expired_at < $3 and nft_direct_sell.state = $4
        "#,
        DirectSellState::Expired as _,
        begin_of_epoch,
        now,
        DirectSellState::Active as _,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        r#"
        update nft_direct_buy 
        set state = $1
        where expired_at != $2 and expired_at < $3 and nft_direct_buy.state = $4
        "#,
        DirectBuyState::Expired as _,
        begin_of_epoch,
        now,
        DirectBuyState::Active as _,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await
}

pub async fn get_nfts_by_collection(
    collection: &str,
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<String>> {
    #[derive(Default)]
    struct NftRecord {
        pub address: String,
    }

    let nfts: Vec<NftRecord> = sqlx::query_as!(
        NftRecord,
        r#"
        select address 
        from nft 
        where collection = $1 and name is null
        "#,
        collection,
    )
    .fetch_all(tx)
    .await?;

    Ok(nfts.into_iter().map(|it| it.address).collect())
}
