use crate::indexer::{events::*, traits::ContractEvent};
use anyhow::Result;
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use serde::Serialize;
use sqlx::{types::chrono::NaiveDateTime, PgPool};
use std::sync::Arc;
use storage::{
    actions::{self, add_whitelist_address},
    traits::*,
    types::Nft,
};
use transaction_consumer::{StreamFrom, TransactionConsumer};

// TODO: make enum for Direct{Buy\Sell}StateChanged status?
// TODO: check for updates
// TODO: nft\collection whitelist?
const AUCTION_ROOT_TIP3: &str = "57254b68950120413f38d14ef4944772d5a729c3c3d352fecc892d67280ac180";
const FACTORY_DIRECT_BUY: &str = "fd44c88376033bed4f686711c7dadc0f25b6fa1b1d9193d2a6041112ae98cbd2";
const FACTORY_DIRECT_SELL: &str =
    "3eec916163fd1826f085c291777d1d8316fab7eaa70990dfd0c9254c1450f2df";

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>) -> Result<()> {
    // TODO: StreamFrom::Stored
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;
    initialize_whitelist_addresses(&pool).await;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        for (parser, handler) in parsers_and_handlers.iter() {
            if let Ok(extracted) = parser.parse(&tx.transaction) {
                if let Some(e) = handler(
                    extracted.into_iter().map(|ex| ex.into_owned()).collect(),
                    pool.clone(),
                    consumer.clone(),
                )
                .await
                .err()
                {
                    log::error!("Error processing transaction: {:#?}", e);
                }
            }
        }

        if let Err(e) = tx.commit() {
            return Err(e.context("Failed committing consumed transacton."));
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

type Handler = Box<
    dyn Fn(Vec<ExtractedOwned>, PgPool, Arc<TransactionConsumer>) -> BoxFuture<'static, Result<()>>,
>;

fn initialize_parsers_and_handlers() -> Result<Vec<(TransactionParser, Handler)>> {
    Ok(vec![
        (
            get_contract_parser("./abi/AuctionTip3.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_auction_tip3(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_auction_root_tip3(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/DirectBuy.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_direct_buy(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/DirectSell.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_direct_sell(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_buy(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_sell(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/Nft.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_nft(extracted, pool, consumer))
            }),
        ),
        (
            get_contract_parser("./abi/Collection.abi.json")?,
            Box::new(move |extracted, pool, consumer| {
                Box::pin(handle_collection(extracted, pool, consumer))
            }),
        ),
    ])
}

// TODO: remove consumer?
async fn handle_event<EventType>(
    event_name: &str,
    extracted: &[ExtractedOwned],
    pool: PgPool,
    _consumer: Arc<TransactionConsumer>,
) -> Result<Option<EventType>>
where
    EventType: ContractEvent + EventRecord + Serialize + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        let record = EventType::build_from(event)
            .map_err(|e| e.context(format!("Error creating {event_name} record")))?;

        // {
        //     let pool = pool.clone();
        //     if let Some(nft) = record.get_nft() {
        //         tokio::spawn(async move {
        //             if let Err(e) = handle_nft(nft.clone(), pool, consumer).await {
        //                 log::error!("Error fetching data for nft {}: {:#?}", nft.to_string(), e);
        //             }
        //         });
        //     }
        // }

        actions::save_event(&record, &pool)
            .await
            .map(|_| {})
            .map_err(|e| e.context(format!("Couldn't save {event_name}")))?;

        Ok(Some(record))
    } else {
        Ok(None)
    }
}

async fn handle_auction_root_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) = handle_event::<AuctionDeployed>(
        "AuctionDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?
    {
        if record.address == AUCTION_ROOT_TIP3.into() {
            if let Err(e) = add_whitelist_address(&record.offer_address, &pool).await {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }

    handle_event::<AuctionDeclined>(
        "AuctionDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionOwnershipTransferred>("OwnershipTransferred", &extracted, pool, consumer)
        .await
        .map(|_| {})
}

async fn handle_auction_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<AuctionCreated>("AuctionCreated", &extracted, pool.clone(), consumer.clone())
        .await?;
    handle_event::<AuctionActive>("AuctionActive", &extracted, pool.clone(), consumer.clone())
        .await?;
    handle_event::<BidPlaced>("BidPlaced", &extracted, pool.clone(), consumer.clone()).await?;
    handle_event::<BidDeclined>("BidDeclined", &extracted, pool.clone(), consumer.clone()).await?;
    handle_event::<AuctionComplete>(
        "AuctionComplete",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionCancelled>("AuctionCancelled", &extracted, pool, consumer)
        .await
        .map(|_| {})
}

async fn handle_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<DirectBuyStateChanged>("DirectBuyStateChanged", &extracted, pool, consumer)
        .await
        .map(|_| {})
}

async fn handle_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<DirectSellStateChanged>("DirectSellStateChanged", &extracted, pool, consumer)
        .await
        .map(|_| {})
}

async fn handle_factory_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) = handle_event::<DirectBuyDeployed>(
        "DirectBuyDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?
    {
        if record.address == FACTORY_DIRECT_BUY.into() {
            if let Err(e) = add_whitelist_address(&record.direct_buy_address, &pool).await {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }
    handle_event::<DirectBuyDeclined>(
        "DirectBuyDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectBuyOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        pool,
        consumer,
    )
    .await
    .map(|_| {})
}

async fn handle_factory_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) = handle_event::<DirectSellDeployed>(
        "DirectSellDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?
    {
        if record.address == FACTORY_DIRECT_SELL.into() {
            if let Err(e) = add_whitelist_address(&record._direct_sell_address, &pool).await {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }
    handle_event::<DirectSellDeclined>(
        "DirectSellDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectSellOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        pool,
        consumer,
    )
    .await
    .map(|_| {})
}

async fn handle_nft(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) =
        handle_event::<NftOwnerChanged>("OwnerChanged", &extracted, pool.clone(), consumer.clone())
            .await?
    {
        let nft = Nft {
            address: record.address,
            collection: None,
            owner: Some(record.new_owner),
            manager: None,
            name: None,        // TODO:
            description: None, // TODO:
            burned: false,
            updated: NaiveDateTime::from_timestamp(record.created_at, 0),
            tx_lt: record.created_lt,
        };

        actions::upsert_nft(&nft, &pool).await?;
    }
    if let Some(record) =
        handle_event::<NftManagerChanged>("ManagerChanged", &extracted, pool.clone(), consumer)
            .await?
    {
        let nft = Nft {
            address: record.address,
            collection: None,
            owner: None,
            manager: Some(record.new_manager),
            name: None,        // TODO:
            description: None, // TODO:
            burned: false,
            updated: NaiveDateTime::from_timestamp(record.created_at, 0),
            tx_lt: record.created_lt,
        };

        actions::upsert_nft(&nft, &pool).await?;
    }

    Ok(())
}

async fn handle_collection(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<CollectionOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    if let Some(record) =
        handle_event::<NftCreated>("NftCreated", &extracted, pool.clone(), consumer.clone()).await?
    {
        let nft = Nft {
            address: record.nft,
            collection: Some(record.address),
            owner: Some(record.owner),
            manager: Some(record.manager),
            name: None,        // TODO:
            description: None, // TODO:
            burned: false,
            updated: NaiveDateTime::from_timestamp(record.created_at, 0),
            tx_lt: record.created_lt,
        };

        actions::upsert_nft(&nft, &pool).await?;
    }
    if let Some(record) =
        handle_event::<NftBurned>("NftBurned", &extracted, pool.clone(), consumer).await?
    {
        let nft = Nft {
            address: record.nft,
            collection: Some(record.address),
            owner: Some(record.owner),
            manager: Some(record.manager),
            name: None,        // TODO:
            description: None, // TODO:
            burned: true,
            updated: NaiveDateTime::from_timestamp(record.created_at, 0),
            tx_lt: record.created_lt,
        };

        actions::upsert_nft(&nft, &pool).await?;
    }

    Ok(())
}

async fn initialize_whitelist_addresses(pool: &PgPool) {
    if let Err(e) = actions::add_whitelist_address(&AUCTION_ROOT_TIP3.into(), pool).await {
        log::error!("Failed adding AuctionTip3 address in whitelist: {:#?}", e);
    }

    if let Err(e) = actions::add_whitelist_address(&FACTORY_DIRECT_BUY.into(), pool).await {
        log::error!(
            "Failed adding FactoryDirectBuy address in whitelist: {:#?}",
            e
        );
    }

    if let Err(e) = actions::add_whitelist_address(&FACTORY_DIRECT_SELL.into(), pool).await {
        log::error!(
            "Failed adding FactoryDirectSell address in whitelist: {:#?}",
            e
        );
    }
}
