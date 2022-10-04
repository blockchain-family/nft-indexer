use crate::indexer::{events::*, traits::ContractEvent};
use anyhow::Result;
use futures::{future::BoxFuture, Future, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use storage::{actions, traits::*};
use transaction_consumer::{StreamFrom, TransactionConsumer};

const AUCTION_ROOT_TIP3: &str =
    "0:2cfcf5d7a3e27bce7fa9164981d026edb88098e62d07e1a4c494aa289b8869c7";
const FACTORY_DIRECT_BUY: &str =
    "0:87156bd63268bc24143df4c9878060bc54968f59a5782a6d98deaddc38462bbb";
const FACTORY_DIRECT_SELL: &str =
    "0:246515362cb259a4c2b79e35e7525088998d1771659fe05444ae4f32bd1e51d8";

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Stored).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;
    initialize_whitelist_addresses(&pool).await;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        for (parser, handler) in parsers_and_handlers.iter() {
            if let Ok(extracted) = parser.parse(&tx.transaction) {
                // TODO: async processing
                let extracted = extracted.into_iter().map(|ex| ex.into_owned()).collect();
                handler(extracted, pool.clone(), consumer.clone()).await;
            }
        }

        if let Err(e) = tx.commit() {
            // TODO: just skip?
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
        let record = match EventType::build_from(event, pool, consumer) {
            Ok(record) => record,
            Err(e) => {
                log::error!("Error creating record {}: {:#?}", event_name, e);
                return None;
            }
        };

        if let Err(e) = actions::save_event(&record, pool).await {
            log::error!("Error saving event {}: {:#?}", event_name, e);
            return None;
        }

        Some(record)
    } else {
        None
    }
}

async fn await_logging_error<F, T>(f: F, trace_id: &str)
where
    F: Future<Output = Result<T>> + Send,
{
    if let Err(e) = f.await {
        log::error!("[{}] Error: {:#?}", trace_id, e);
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
        if record.address == AUCTION_ROOT_TIP3.into() {
            if let Err(e) = actions::add_whitelist_address(&record.offer_address, &pool).await {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.offer_address,
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

    if let Some(record) =
        handle_event::<AuctionActive>("AuctionActive", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_auction(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
        await_logging_error(record.upsert_nft_price_history(), &record.address.0).await;
    }

    if let Some(record) = handle_event::<BidPlaced>("BidPlaced", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_bid(), &record.address.0).await;
        await_logging_error(record.upsert_auction(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
        await_logging_error(record.upsert_nft_price_history(), &record.address.0).await;
    }

    if let Some(record) =
        handle_event::<BidDeclined>("BidDeclined", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_bid(), &record.address.0).await;
        await_logging_error(record.upsert_auction(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
    }

    if let Some(record) =
        handle_event::<AuctionComplete>("AuctionComplete", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_auction(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
    }

    if let Some(record) =
        handle_event::<AuctionCancelled>("AuctionCancelled", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_auction(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
    }
}

async fn handle_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) =
        handle_event::<DirectBuyStateChanged>("DirectBuyStateChanged", &extracted, &pool, &consumer)
            .await
    {
        await_logging_error(record.upsert_direct_buy(), &record.address.0).await;
        await_logging_error(record.upsert_nft_price_history(), &record.address.0).await;
    }
}

async fn handle_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) = handle_event::<DirectSellStateChanged>(
        "DirectSellStateChanged",
        &extracted,
        &pool,
        &consumer,
    )
    .await
    {
        await_logging_error(record.upsert_direct_sell(), &record.address.0).await;
        await_logging_error(record.upsert_collection(), &record.address.0).await;
        await_logging_error(record.upsert_nft_price_history(), &record.address.0).await;
    }
}

async fn handle_factory_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) =
        handle_event::<DirectBuyDeployed>("DirectBuyDeployed", &extracted, &pool, &consumer).await
    {
        if record.address == FACTORY_DIRECT_BUY.into() {
            if let Err(e) = actions::add_whitelist_address(&record.direct_buy_address, &pool).await
            {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.direct_buy_address,
                    e
                );
            }
        }
    }
    handle_event::<DirectBuyDeclined>("DirectBuyDeclined", &extracted, &pool, &consumer).await;
    handle_event::<DirectBuyOwnershipTransferred>(
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
        if record.address == FACTORY_DIRECT_SELL.into() {
            if let Err(e) = actions::add_whitelist_address(&record.direct_sell_address, &pool).await
            {
                log::error!(
                    "Failed adding address {:#?} in whitelist: {:#?}",
                    &record.direct_sell_address,
                    e
                );
            }
        }
    }
    handle_event::<DirectSellDeclined>("DirectSellDeclined", &extracted, &pool, &consumer).await;
    handle_event::<DirectSellOwnershipTransferred>(
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
    if let Some(record) =
        handle_event::<NftOwnerChanged>("OwnerChanged", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_nft(), &record.address.0).await;
    }

    if let Some(record) =
        handle_event::<NftManagerChanged>("ManagerChanged", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_nft(), &record.address.0).await;
    }
}

async fn handle_collection(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) {
    if let Some(record) = handle_event::<CollectionOwnershipTransferred>(
        "OwnershipTransferred",
        &extracted,
        &pool,
        &consumer,
    )
    .await
    {
        await_logging_error(record.upsert_collection(), &record.address.0).await;
    }

    if let Some(record) =
        handle_event::<NftCreated>("NftCreated", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_collection(), &record.address.0).await;
        await_logging_error(record.upsert_nft(), &record.address.0).await;
    }

    if let Some(record) = handle_event::<NftBurned>("NftBurned", &extracted, &pool, &consumer).await
    {
        await_logging_error(record.upsert_collection(), &record.address.0).await;
        await_logging_error(record.upsert_nft(), &record.address.0).await;
    }
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
