use crate::database::{
    actions::{self, add_whitelist_address},
    records::*,
};
use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use sqlx::PgPool;
use std::sync::Arc;
use ton_block::MsgAddressInt;
use transaction_consumer::{StreamFrom, TransactionConsumer};

const AUCTION_ROOT_TIP3: &str = "9b10e4ab6be3ad33ce621d2c7aec866bdf81983e7e2ce660423fae9b29f0ca65";
const FACTORY_DIRECT_BUY: &str = "e7df75e4a0dafe74179ae8c588d02f98283d4fe73a2f08e0da9e6d9184bbe2ee";
const FACTORY_DIRECT_SELL: &str =
    "0ff6c5cc1dc1888b8d74a5ec13d652815e8f25043c22a2d8895c6d4b4d1dbe52";

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>) -> Result<()> {
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

type Handler =
    fn(Vec<ExtractedOwned>, PgPool, Arc<TransactionConsumer>) -> BoxFuture<'static, Result<()>>;

fn initialize_parsers_and_handlers() -> Result<Vec<(TransactionParser, Handler)>> {
    Ok(vec![
        (
            get_contract_parser("./abi/AuctionTip3.abi.json")?,
            |extracted, pool, consumer| Box::pin(handle_auction_tip3(extracted, pool, consumer)),
        ),
        (
            get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
            |extracted, pool, consumer| {
                Box::pin(handle_auction_root_tip3(extracted, pool, consumer))
            },
        ),
        (
            get_contract_parser("./abi/DirectBuy.abi.json")?,
            |extracted, pool, consumer| Box::pin(handle_direct_buy(extracted, pool, consumer)),
        ),
        (
            get_contract_parser("./abi/DirectSell.abi.json")?,
            |extracted, pool, consumer| Box::pin(handle_direct_sell(extracted, pool, consumer)),
        ),
        (
            get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
            |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_buy(extracted, pool, consumer))
            },
        ),
        (
            get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
            |extracted, pool, consumer| {
                Box::pin(handle_factory_direct_sell(extracted, pool, consumer))
            },
        ),
    ])
}

async fn handle_event<Event>(
    event_name: &str,
    extracted: &[ExtractedOwned],
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<Option<Event>>
where
    Event: EventRecord + DatabaseRecord + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        return match Event::build_from(event) {
            Ok(record) => {
                {
                    let pool = pool.clone();
                    if let Some(nft) = record.get_nft() {
                        tokio::spawn(async move {
                            match fetch_metadata(&nft, consumer.clone()).await {
                                Ok(data) => {
                                    if let Err(e) = (NftMetadata {
                                        nft: nft.to_string().into(),
                                        data,
                                    }
                                    .put_in(&pool)
                                    .await)
                                    {
                                        log::error!(
                                            "Couldn't save metadata for {}: {:#?}",
                                            nft.to_string(),
                                            e
                                        );
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "Error fetching metadata for {}: {:#?}",
                                        nft.to_string(),
                                        e
                                    );
                                }
                            }
                        });
                    }
                }

                record
                    .put_in(&pool)
                    .await
                    .map(|_| {})
                    .map_err(|e| e.context(format!("Couldn't save {event_name}")))?;

                Ok(Some(record))
            }

            Err(e) => Err(e.context(format!("Error creating {event_name} record"))),
        };
    }

    Ok(None)
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
            // TODO?
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
    handle_event::<AuctionCancelled>(
        "AuctionCancelled",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
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

async fn fetch_metadata(
    nft: &MsgAddressInt,
    consumer: Arc<TransactionConsumer>,
) -> Result<serde_json::Value> {
    let contract = consumer
        .get_contract_state(nft)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let metadata = nekoton_contracts::tip4_2::MetadataContract(
        contract.as_context(&nekoton_utils::SimpleClock),
    );

    Ok(serde_json::from_str::<serde_json::Value>(
        &metadata.get_json()?,
    )?)
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
