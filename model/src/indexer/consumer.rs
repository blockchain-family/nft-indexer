use crate::indexer::{events::*, traits::ContractEvent};
use anyhow::Result;
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use storage::{actions, traits::*};
use transaction_consumer::{StreamFrom, TransactionConsumer};

const AUCTION_ROOT_TIP3: &str =
    "0:0799e562ed3a26b82c2533a6f7a6e12b276084ad4d9ad3f2b3ec2a125eb7d57c";
const FACTORY_DIRECT_BUY: &str =
    "0:63e293bcc8f1f2f311d2334e19ea9ad3633e32c3aa47dd438af7f2a8e0d92a12";
const FACTORY_DIRECT_SELL: &str =
    "0:7574093078e57213d541ee0f7f6719f319b70228345bc2edbab2d0df1496bdf3";

// TODO: async tx processing

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;
    initialize_whitelist_addresses(&pool).await;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        for (parser, handler) in parsers_and_handlers.iter() {
            if let Ok(extracted) = parser.parse(&tx.transaction) {
                let extracted = extracted.into_iter().map(|ex| ex.into_owned()).collect();
                if let Err(e) = handler(extracted, pool.clone(), consumer.clone()).await {
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

type Handler = Arc<
    dyn Fn(Vec<ExtractedOwned>, PgPool, Arc<TransactionConsumer>) -> BoxFuture<'static, Result<()>>
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
) -> Result<Option<EventType>>
where
    EventType: ContractEvent + EventRecord + Serialize + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        let record = EventType::build_from(event, pool, consumer)
            .map_err(|e| e.context(format!("Error creating {event_name} record")))?;

        actions::save_event(&record, pool)
            .await
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
    if let Some(record) =
        handle_event::<AuctionDeployed>("AuctionDeployed", &extracted, &pool, &consumer).await?
    {
        if record.address == AUCTION_ROOT_TIP3.into() {
            if let Err(e) = actions::add_whitelist_address(&record.offer_address, &pool).await {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }

    handle_event::<AuctionDeclined>("AuctionDeclined", &extracted, &pool, &consumer).await?;
    handle_event::<AuctionOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await?;

    Ok(())
}

async fn handle_auction_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<AuctionCreated>("AuctionCreated", &extracted, &pool, &consumer).await?;

    if let Some(record) =
        handle_event::<AuctionActive>("AuctionActive", &extracted, &pool, &consumer).await?
    {
        record.upsert_auction().await?;
        record.upsert_collection().await?;
    }

    if let Some(record) =
        handle_event::<BidPlaced>("BidPlaced", &extracted, &pool, &consumer).await?
    {
        record.upsert_bid().await?;
        record.upsert_auction().await?;
        record.upsert_collection().await?;
    }

    if let Some(record) =
        handle_event::<BidDeclined>("BidDeclined", &extracted, &pool, &consumer).await?
    {
        record.upsert_bid().await?;
        record.upsert_auction().await?;
        record.upsert_collection().await?;
    }

    if let Some(record) =
        handle_event::<AuctionComplete>("AuctionComplete", &extracted, &pool, &consumer).await?
    {
        record.upsert_auction().await?;
        record.upsert_collection().await?;
    }

    if let Some(record) =
        handle_event::<AuctionCancelled>("AuctionCancelled", &extracted, &pool, &consumer).await?
    {
        record.upsert_auction().await?;
        record.upsert_collection().await?;
    }

    Ok(())
}

async fn handle_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) =
        handle_event::<DirectBuyStateChanged>("DirectBuyStateChanged", &extracted, &pool, &consumer)
            .await?
    {
        record.upsert_direct_buy().await?;
    }

    Ok(())
}

async fn handle_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) = handle_event::<DirectSellStateChanged>(
        "DirectSellStateChanged",
        &extracted,
        &pool,
        &consumer,
    )
    .await?
    {
        record.upsert_direct_sell().await?;
        record.upsert_collection().await?;
    }

    Ok(())
}

async fn handle_factory_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) =
        handle_event::<DirectBuyDeployed>("DirectBuyDeployed", &extracted, &pool, &consumer).await?
    {
        if record.address == FACTORY_DIRECT_BUY.into() {
            if let Err(e) = actions::add_whitelist_address(&record.direct_buy_address, &pool).await
            {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }
    handle_event::<DirectBuyDeclined>("DirectBuyDeclined", &extracted, &pool, &consumer).await?;
    handle_event::<DirectBuyOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await?;

    Ok(())
}

async fn handle_factory_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) =
        handle_event::<DirectSellDeployed>("DirectSellDeployed", &extracted, &pool, &consumer)
            .await?
    {
        if record.address == FACTORY_DIRECT_SELL.into() {
            if let Err(e) = actions::add_whitelist_address(&record.direct_sell_address, &pool).await
            {
                log::error!("Failed adding address in whitelist: {:#?}", e);
            }
        }
    }
    handle_event::<DirectSellDeclined>("DirectSellDeclined", &extracted, &pool, &consumer).await?;
    handle_event::<DirectSellOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await?;

    Ok(())
}

async fn handle_nft(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) =
        handle_event::<NftOwnerChanged>("OwnerChanged", &extracted, &pool, &consumer).await?
    {
        record.upsert_nft().await?;
    }

    if let Some(record) =
        handle_event::<NftManagerChanged>("ManagerChanged", &extracted, &pool, &consumer).await?
    {
        record.upsert_nft().await?;
    }

    Ok(())
}

async fn handle_collection(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    if let Some(record) = handle_event::<CollectionOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await?
    {
        record.upsert_collection().await?;
    }

    if let Some(record) =
        handle_event::<NftCreated>("NftCreated", &extracted, &pool, &consumer).await?
    {
        record.upsert_collection().await?;
        record.upsert_nft().await?;
    }

    if let Some(record) =
        handle_event::<NftBurned>("NftBurned", &extracted, &pool, &consumer).await?
    {
        record.upsert_collection().await?;
        record.upsert_nft().await?;
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
