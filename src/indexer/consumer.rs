use anyhow::Result;
use futures::{Future, StreamExt};
use nekoton_abi::{transaction_parser::Extracted, TransactionParser};
use sqlx::PgPool;
use std::{pin::Pin, sync::Arc};
use transaction_consumer::{StreamFrom, TransactionConsumer};

pub async fn serve(pool: Arc<PgPool>, consumer: Arc<TransactionConsumer>) -> Result<()> {
    let stream = consumer.stream_transactions(StreamFrom::Beginning).await?;
    let mut fs = futures::stream::StreamExt::fuse(stream);

    let parsers_and_handlers = initialize_parsers_and_handlers()?;

    log::info!("Start listening to kafka...");
    while let Some(tx) = fs.next().await {
        if let Some(f) = parsers_and_handlers.iter().find_map(|(parser, handler)| {
            parser
                .parse(&tx.transaction)
                .map_or(None, |extracted| Some(handler(extracted, &pool)))
        }) {
            if let Some(e) = f.await.err() {
                log::error!("Error processing transaction: {:#?}", e);
            }
        }

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

type Handler = fn(Vec<Extracted<'_>>, &PgPool) -> Pin<Box<dyn Future<Output = Result<()>>>>;

fn initialize_parsers_and_handlers() -> Result<Vec<(TransactionParser, Handler)>> {
    Ok(vec![
        (
            get_contract_parser("./abi/AuctionRootTip3.abi.json")?,
            handle_auction_root_tip3,
        ),
        (
            get_contract_parser("./abi/AuctionTip3.abi.json")?,
            handle_auction_tip3,
        ),
        (
            get_contract_parser("./abi/DirectBuy.abi.json")?,
            handle_direct_buy,
        ),
        (
            get_contract_parser("./abi/DirectSell.abi.json")?,
            handle_direct_sell,
        ),
        (
            get_contract_parser("./abi/FactoryDirectBuy.abi.json")?,
            handle_factory_direct_buy,
        ),
        (
            get_contract_parser("./abi/FactoryDirectSell.abi.json")?,
            handle_factory_direct_sell,
        ),
    ])
}

fn handle_auction_root_tip3(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}

fn handle_auction_tip3(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}

fn handle_direct_buy(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}

fn handle_direct_sell(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}

fn handle_factory_direct_buy(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}

fn handle_factory_direct_sell(
    _extracted: Vec<Extracted<'_>>,
    _pool: &PgPool,
) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    todo!()
}
