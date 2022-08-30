use anyhow::Result;
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use sqlx::PgPool;
use std::sync::Arc;
use transaction_consumer::{StreamFrom, TransactionConsumer};

use crate::database::records::*;

pub async fn serve(pool: Arc<PgPool>, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        for (parser, handler) in parsers_and_handlers.iter() {
            if let Ok(extracted) = parser.parse(&tx.transaction) {
                if let Some(e) = handler(
                    extracted.into_iter().map(|ex| ex.into_owned()).collect(),
                    &pool,
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

type Handler = fn(Vec<ExtractedOwned>, &PgPool) -> BoxFuture<Result<()>>;

fn initialize_parsers_and_handlers() -> Result<Vec<(TransactionParser, Handler)>> {
    Ok(vec![
        (
            get_contract_parser("./abi/AuctionTip3.abi.json")?,
            |extracted, pool| Box::pin(handle_auction_tip3(extracted, pool)),
        ),
        (
            get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
            |extracted, pool| Box::pin(handle_auction_root_tip3(extracted, pool)),
        ),
        (
            get_contract_parser("./abi/DirectBuy.abi.json")?,
            |extracted, pool| Box::pin(handle_direct_buy(extracted, pool)),
        ),
        (
            get_contract_parser("./abi/DirectSell.abi.json")?,
            |extracted, pool| Box::pin(handle_direct_sell(extracted, pool)),
        ),
        (
            get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
            |extracted, pool| Box::pin(handle_factory_direct_buy(extracted, pool)),
        ),
        (
            get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
            |extracted, pool| Box::pin(handle_factory_direct_sell(extracted, pool)),
        ),
    ])
}

async fn handle_event<T>(
    event_name: &str,
    extracted: &[ExtractedOwned],
    pool: &PgPool,
) -> Result<()>
where
    T: Build + Put + Sync,
{
    if let Some(event) = extracted.iter().find(|e| e.name == event_name) {
        return match T::build_record(event) {
            Ok(record) => record
                .put_record(pool)
                .await
                .map(|_| {})
                .map_err(|e| e.context(format!("Couldn't save {event_name}"))),

            Err(e) => Err(e.context(format!("Error creating {event_name} record"))),
        };
    }

    Ok(())
}

async fn handle_auction_root_tip3(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    handle_event::<AuctionDeployedRecord>("AuctionDeployed", &extracted, pool).await?;
    handle_event::<AuctionDeclinedRecord>("AuctionDeclined", &extracted, pool).await?;
    handle_event::<AuctionOwnershipTransferredRecord>("OwnershipTransferred", &extracted, pool)
        .await
}

async fn handle_auction_tip3(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    // TODO: AuctionActive?
    handle_event::<AuctionActiveRecord>("AuctionActive", &extracted, pool).await?;
    handle_event::<BidPlacedRecord>("BidPlaced", &extracted, pool).await?;
    handle_event::<BidDeclinedRecord>("BidDeclined", &extracted, pool).await?;
    handle_event::<AuctionCompleteRecord>("AuctionComplete", &extracted, pool).await?;
    handle_event::<AuctionCancelledRecord>("AuctionCancelled", &extracted, pool).await
}

async fn handle_direct_buy(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    handle_event::<DirectBuyStateChangedRecord>("DirectBuyStateChanged", &extracted, pool).await
}

async fn handle_direct_sell(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    handle_event::<DirectSellStateChangedRecord>("DirectSellStateChanged", &extracted, pool).await
}

async fn handle_factory_direct_buy(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    handle_event::<DirectBuyDeployedRecord>("DirectBuyDeployed", &extracted, pool).await?;
    handle_event::<DirectBuyDeclinedRecord>("DirectBuyDeclined", &extracted, pool).await?;
    handle_event::<DirectBuyOwnershipTransferredRecord>("OwnershipTransferred", &extracted, pool)
        .await
}

async fn handle_factory_direct_sell(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    handle_event::<DirectSellDeployedRecord>("DirectSellDeployed", &extracted, pool).await?;
    handle_event::<DirectSellDeclinedRecord>("DirectSellDeclined", &extracted, pool).await?;
    handle_event::<DirectSellOwnershipTransferredRecord>("OwnershipTransferred", &extracted, pool)
        .await
}
