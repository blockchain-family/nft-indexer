use crate::{database::actions, indexer::record_builder};
use anyhow::Result;
use futures::StreamExt;
use nekoton_abi::{transaction_parser::Extracted, TransactionParser};
use sqlx::PgPool;
use std::sync::Arc;
use transaction_consumer::{StreamFrom, TransactionConsumer};

pub async fn serve(pool: Arc<PgPool>, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers = initialize_parsers()?;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        if let Some(extracted) = parsers
            .iter()
            .find_map(|parser| parser.parse(&tx.transaction).ok())
        {
            // TODO: handle by name
            for ex in extracted {
                println!("{:#?}", ex.name);
            }
        }

        // if let Err(e) = process_event... {
        //     log::error!("Error processing %event%: {:#?}", e);
        // }

        if let Err(e) = tx.commit() {
            return Err(e.context("Failed committing consumed transacton."));
        }
    }

    log::warn!("Transactions stream terminated.");

    Ok(())
}

// TODO: process event

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

fn initialize_parsers() -> Result<Vec<TransactionParser>> {
    Ok(vec![
        get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
        get_contract_parser("./abi/AuctionTip3.abi.json")?,
        get_contract_parser("./abi/DirectBuy.abi.json")?,
        get_contract_parser("./abi/DirectSell.abi.json")?,
        get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
        get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
    ])
}
