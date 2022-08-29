use anyhow::Result;
use futures::{future::BoxFuture, StreamExt};
use nekoton_abi::{transaction_parser::ExtractedOwned, TransactionParser};
use sqlx::PgPool;
use std::sync::Arc;
use transaction_consumer::{StreamFrom, TransactionConsumer};

use crate::{database::actions, indexer::record_builder};

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

async fn handle_auction_root_tip3(extracted: Vec<ExtractedOwned>, pool: &PgPool) -> Result<()> {
    if let Some(event) = extracted.iter().find(|e| e.name == "AuctionDeployed") {
        match record_builder::build_auction_deployed_record(event) {
            Ok(record) => actions::put_auction_deployed_record(pool, &record)
                .await
                .map(|_| {})
                .map_err(|e| e.context("Couldn't save AuctionDeployed")),

            Err(e) => Err(e.context("Error creating AuctionDeclined record")),
        }
    } else if let Some(event) = extracted.iter().find(|e| e.name == "AuctionDeclined") {
        println!(
            "{:#?}",
            record_builder::build_auction_declined_record(event)
        );
        Ok(())
    } else {
        Ok(())
    }
}

async fn handle_auction_tip3(extracted: Vec<ExtractedOwned>, _pool: &PgPool) -> Result<()> {
    if let Some(event) = extracted.iter().find(|e| e.name == "AuctionActive") {
        println!("{:#?}", record_builder::build_auction_active_record(event));
    }

    Ok(())
}

async fn handle_direct_buy(_extracted: Vec<ExtractedOwned>, _pool: &PgPool) -> Result<()> {
    Ok(())
}

async fn handle_direct_sell(extracted: Vec<ExtractedOwned>, _pool: &PgPool) -> Result<()> {
    if let Some(event) = extracted
        .iter()
        .find(|e| e.name == "DirectSellStateChanged")
    {
        println!(
            "{:#?}",
            record_builder::build_direct_sell_state_changed_record(event)
        );
    }

    Ok(())
}

async fn handle_factory_direct_buy(_extracted: Vec<ExtractedOwned>, _pool: &PgPool) -> Result<()> {
    Ok(())
}

async fn handle_factory_direct_sell(_extracted: Vec<ExtractedOwned>, _pool: &PgPool) -> Result<()> {
    Ok(())
}
