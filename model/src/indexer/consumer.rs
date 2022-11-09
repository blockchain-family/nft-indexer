use crate::{
    indexer::{events::*, traits::ContractEvent},
    settings::config::Config,
};
use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use once_cell::sync::OnceCell;
use serde::Serialize;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use storage::{actions, traits::*};
use transaction_consumer::{StreamFrom, TransactionConsumer};

#[derive(PartialEq, Eq, Hash)]
enum OfferRootType {
    AuctionRoot = 0,
    FactoryDirectBuy,
    FactoryDirectSell,
}

static TRUSTED_ADDRESSES: OnceCell<HashMap<OfferRootType, Vec<String>>> = OnceCell::new();

fn init_trusted_addresses(config: Config) -> Result<()> {
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

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>, config: Config) -> Result<()> {
    let from = if config.reset {
        StreamFrom::Beginning
    } else {
        StreamFrom::Stored
    };

    init_trusted_addresses(config)?;

    let stream = consumer.stream_transactions(from).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;
    initialize_whitelist_addresses(&pool).await;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        for (parser, handler) in parsers_and_handlers.iter() {
            if let Ok(extracted) = parser.parse(&tx.transaction) {
                let extracted = extracted.into_iter().map(|ex| ex.into_owned()).collect();
                handler(extracted, pool.clone(), consumer.clone()).await;
            }
        }

        if let Err(e) = tx.commit() {
            return Err(e.context("Failed committing transacton"));
        }
    }

    log::warn!("Transactions stream terminated.");

    Ok(())
}

fn get_contract_parser(abi_path: &str) -> Result<TransactionParser> {
    let abi_json = std::fs::read_to_string(abi_path)?;
    let abi = ton_abi::Contract::load(&abi_json)?;

    let events = abi.events.into_values();
    let funs = abi.functions.into_values();
    TransactionParser::builder()
        .function_in_list(funs, false)
        .events_list(events)
        .build_with_external_in()
}

type Handler = Arc<
    dyn Fn(Vec<ExtractedOwned>, PgPool, Arc<TransactionConsumer>) -> BoxFuture<'static, ()>
        + Send
        + Sync,
>;

fn initialize_parsers_and_handlers() -> Result<Vec<(TransactionParser, Handler)>> {
    Ok(vec![
        (
            get_contract_parser("./abi/AuctionTip3.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_auction_tip3(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_auction_root_tip3(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/DirectBuy.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_direct_buy(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/DirectSell.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_direct_sell(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_buy(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_sell(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/Nft.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_nft(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/Collection.abi.json")?,
            Arc::new(move |extracted, pool, consumer| {
                Box::pin(handle_collection(extracted, pool, consumer))
            }),
        ),
    ])
}

async fn handle_event<EventType>(
    event_name: &str,
    extracted: &[ExtractedOwned],
    pool: &PgPool,
    consumer: &Arc<TransactionConsumer>,
) -> Option<EventType>
where
    EventType: ContractEvent + EventRecord + Serialize + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        let mut record = match EventType::build_from(event, pool, consumer) {
            Ok(record) => record,
            Err(e) => {
                log::error!("Error creating record {}: {:#?}", event_name, e);
                return None;
            }
        };

        if let Err(e) = record.update_dependent_tables().await {
            log::error!(
                "Error updating dependent tables of {}: {:#?}",
                event_name,
                e
            );
            return None;
        }

        Some(record)
    } else {
        None
    }
}

async fn handle_auction_root_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) =
        handle_event::<AuctionDeployed>("AuctionDeployed", &extracted, &pool, &consumer).await
    {
        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::AuctionRoot].contains(&record.address.0)
        {
            if let Err(e) = actions::add_whitelist_address(&record.offer, &pool).await {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.offer,
                    e
                );
            }
        }
    }

    handle_event::<AuctionDeclined>("AuctionDeclined", &extracted, &pool, &consumer).await;

    handle_event::<AuctionRootOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await;
}

async fn handle_auction_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    handle_event::<AuctionCreated>("AuctionCreated", &extracted, &pool, &consumer).await;
    handle_event::<AuctionActive>("AuctionActive", &extracted, &pool, &consumer).await;
    handle_event::<AuctionBidPlaced>("BidPlaced", &extracted, &pool, &consumer).await;
    handle_event::<AuctionBidDeclined>("BidDeclined", &extracted, &pool, &consumer).await;
    handle_event::<AuctionComplete>("AuctionComplete", &extracted, &pool, &consumer).await;
    handle_event::<AuctionCancelled>("AuctionCancelled", &extracted, &pool, &consumer).await;
}

async fn handle_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    handle_event::<DirectBuyStateChanged>("DirectBuyStateChanged", &extracted, &pool, &consumer)
        .await;
}

async fn handle_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    handle_event::<DirectSellStateChanged>("DirectSellStateChanged", &extracted, &pool, &consumer)
        .await;
}

async fn handle_factory_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) =
        handle_event::<DirectBuyDeployed>("DirectBuyDeployed", &extracted, &pool, &consumer).await
    {
        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectBuy]
            .contains(&record.address.0)
        {
            if let Err(e) = actions::add_whitelist_address(&record.direct_buy, &pool).await {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.direct_buy,
                    e
                );
            }
        }
    }
    handle_event::<DirectBuyDeclined>("DirectBuyDeclined", &extracted, &pool, &consumer).await;
    handle_event::<FactoryDirectBuyOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await;
}

async fn handle_factory_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) =
        handle_event::<DirectSellDeployed>("DirectSellDeployed", &extracted, &pool, &consumer).await
    {
        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell]
            .contains(&record.address.0)
        {
            if let Err(e) = actions::add_whitelist_address(&record.direct_sell, &pool).await {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.direct_sell,
                    e
                );
            }
        }
    }
    handle_event::<DirectSellDeclined>("DirectSellDeclined", &extracted, &pool, &consumer).await;
    handle_event::<FactoryDirectSellOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await;
}

async fn handle_nft(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    handle_event::<NftOwnerChanged>("OwnerChanged", &extracted, &pool, &consumer).await;
    handle_event::<NftManagerChanged>("ManagerChanged", &extracted, &pool, &consumer).await;
}

async fn handle_collection(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    handle_event::<CollectionOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await;
    handle_event::<NftCreated>("NftCreated", &extracted, &pool, &consumer).await;
    handle_event::<NftBurned>("NftBurned", &extracted, &pool, &consumer).await;
}

async fn initialize_whitelist_addresses(pool: &PgPool) {
    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::AuctionRoot].iter() {
        if let Err(e) = actions::add_whitelist_address(&(*addr.clone()).into(), pool).await {
            log::error!("Failed adding AuctionTip3 address in whitelist: {:#?}", e);
        }
    }

    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectBuy].iter() {
        if let Err(e) = actions::add_whitelist_address(&(*addr.clone()).into(), pool).await {
            log::error!(
                "Failed adding FactoryDirectBuy address in whitelist: {:#?}",
                e
            );
        }
    }

    for addr in TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell].iter() {
        if let Err(e) = actions::add_whitelist_address(&(*addr.clone()).into(), pool).await {
            log::error!(
                "Failed adding FactoryDirectSell address in whitelist: {:#?}",
                e
            );
        }
    }
}
