use crate::database::records::*;
use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use sqlx::PgPool;
use std::sync::Arc;
use ton_block::MsgAddressInt;
use transaction_consumer::{StreamFrom, TransactionConsumer};

pub async fn serve(pool: PgPool, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;

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

async fn handle_event<T>(
    event_name: &str,
    extracted: &[ExtractedOwned],
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<Option<T>>
where
    T: Build + Put + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        return match T::build_record(event) {
            Ok(record) => {
                {
                    let pool = pool.clone();
                    // TODO: better
                    if let Some(nft) = record.get_nft() {
                        tokio::spawn(async move {
                            match fetch_metadata(&nft, consumer.clone()).await {
                                Ok(data) => {
                                    if let Err(e) = (NftMetadataRecord {
                                        nft: nft.to_string(),
                                        data,
                                    }
                                    .put_record(&pool)
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
                    .put_record(&pool)
                    .await
                    .map(|_| Some(record))
                    .map_err(|e| e.context(format!("Couldn't save {event_name}")))
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
    handle_event::<AuctionDeployedRecord>(
        "AuctionDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionDeclinedRecord>(
        "AuctionDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionOwnershipTransferredRecord>(
        "OwnershipTransferred",
        &extracted,
        pool,
        consumer,
    )
    .await
    .map(|_| {})
}

async fn handle_auction_tip3(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<AuctionCreatedRecord>(
        "AuctionCreated",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionActiveRecord>(
        "AuctionActive",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<BidPlacedRecord>("BidPlaced", &extracted, pool.clone(), consumer.clone())
        .await?;
    handle_event::<BidDeclinedRecord>("BidDeclined", &extracted, pool.clone(), consumer.clone())
        .await?;
    handle_event::<AuctionCompleteRecord>(
        "AuctionComplete",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<AuctionCancelledRecord>(
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
    handle_event::<DirectBuyStateChangedRecord>("DirectBuyStateChanged", &extracted, pool, consumer)
        .await
        .map(|_| {})
}

async fn handle_direct_sell(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<DirectSellStateChangedRecord>(
        "DirectSellStateChanged",
        &extracted,
        pool,
        consumer,
    )
    .await
    .map(|_| {})
}

async fn handle_factory_direct_buy(
    extracted: Vec<ExtractedOwned>,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> Result<()> {
    handle_event::<DirectBuyDeployedRecord>(
        "DirectBuyDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectBuyDeclinedRecord>(
        "DirectBuyDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectBuyOwnershipTransferredRecord>(
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
    handle_event::<DirectSellDeployedRecord>(
        "DirectSellDeployed",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectSellDeclinedRecord>(
        "DirectSellDeclined",
        &extracted,
        pool.clone(),
        consumer.clone(),
    )
    .await?;
    handle_event::<DirectSellOwnershipTransferredRecord>(
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
