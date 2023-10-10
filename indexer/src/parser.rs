use crate::models::events::*;
use crate::persistence::collections_queue::CollectionsQueue;
use crate::persistence::entities::*;
use crate::settings;
use crate::utils::DecodeContext;
use anyhow::Result;
use data_reader::PriceReader;
use futures::channel::mpsc::{Receiver, Sender};
use futures::{future, SinkExt, StreamExt};
use indexer_repo::batch::*;
use indexer_repo::types::NftCollection;
use nekoton_abi::transaction_parser::{ExtractedOwned, ParsedType};
use nekoton_abi::UnpackAbiPlain;
use sqlx::PgPool;
use std::sync::Arc;
use transaction_buffer::models::{BufferedConsumerChannels, RawTransaction};

const EVENTS_PER_ITERATION: usize = 1000;

pub async fn start_parsing(
    config: settings::config::Config,
    pg_pool: PgPool,
    price_reader: Arc<PriceReader>,
) -> Result<()> {
    let BufferedConsumerChannels {
        rx_parsed_events,
        tx_commit,
        notify_for_services,
    } = settings::init_transaction_buffer(&config, &pg_pool).await?;

    log::info!("Connected to kafka");

    tokio::spawn(run_nft_indexer(
        rx_parsed_events,
        tx_commit,
        pg_pool,
        price_reader,
    ));

    notify_for_services.notified().await;

    future::pending().await
}

pub async fn run_nft_indexer(
    mut rx_raw_transactions: Receiver<Vec<(Vec<ExtractedOwned>, RawTransaction)>>,
    mut tx_commit: Sender<()>,
    pool: PgPool,
    price_reader: Arc<PriceReader>,
) {
    log::info!("Start nft indexer...");

    let mut collection_queue = CollectionsQueue::new(pool.clone()).await;

    while let Some(message) = rx_raw_transactions.next().await {
        let mut data = Vec::with_capacity(EVENTS_PER_ITERATION * 3);

        for (out, tx) in message {
            let mut events = Vec::new();
            let mut function_inputs = Vec::new();

            for extractable in out {
                match extractable.parsed_type {
                    ParsedType::Event => {
                        events.push(extractable);
                    }
                    ParsedType::FunctionInput => {
                        function_inputs.extend(extractable.tokens.into_iter());
                    }
                    _ => {}
                }
            }

            for event in events {
                let ctx = DecodeContext {
                    tx_data: tx.data.clone(),
                    function_inputs: function_inputs.clone(),
                    message_hash: event.message_hash,
                };

                if let Ok(Some(entity)) = unpack_entity(&event) {
                    if let Ok(decoded) = entity.decode(&ctx) {
                        data.push(decoded);
                    } else {
                        log::error!("Error while decode");
                    }
                    if let Ok(event) = entity.decode_event(&ctx) {
                        data.push(event);
                    } else {
                        log::error!("Error while decode_event");
                    }
                }
            }
        }

        let now = std::time::Instant::now();
        save_to_db(&pool, &price_reader, data, &mut collection_queue)
            .await
            .expect("Error saving to DB");
        let elapsed = now.elapsed();

        log::info!("METRIC | Saving to db, elapsed {}ms", elapsed.as_millis());

        tx_commit.send(()).await.expect("dead commit sender");
    }

    panic!("rip kafka consumer");
}

async fn save_to_db(
    pool: &PgPool,
    price_reader: &PriceReader,
    data: Vec<Decoded>,
    collections_queue: &mut CollectionsQueue,
) -> Result<()> {
    let mut collections = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut nft_created = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut nft_burned = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut nft_owner_changed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut nft_manager_changed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut auc_deployed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut auc_active = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut prices = Vec::with_capacity(EVENTS_PER_ITERATION * 2);
    let mut auc_bid_placed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut auc_bid_declined = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut auc_complete = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut auc_cancelled = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut raw_events = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut fees_update = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut direct_sell_deployed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut direct_sell_state_changed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut direct_buy_deployed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut direct_buy_state_changed = Vec::with_capacity(EVENTS_PER_ITERATION);
    let mut deployed_offers = Vec::with_capacity(EVENTS_PER_ITERATION);

    for element in data {
        match element {
            Decoded::CreateNft(nft) => {
                collections.push(NftCollection {
                    address: nft.collection.clone(),
                    nft_first_mint: nft.updated,
                });
                nft_created.push(nft);
            }
            Decoded::BurnNft(nft) => nft_burned.push(nft),
            Decoded::OwnerChangedNft(addr) => nft_owner_changed.push(addr),
            Decoded::ManagerChangedNft(addr) => nft_manager_changed.push(addr),
            Decoded::AuctionDeployed((a, offer)) => {
                auc_deployed.push(a);
                deployed_offers.push(offer);
            }
            Decoded::AuctionActive(auc) => auc_active.push(auc),
            Decoded::AuctionBidPlaced(auc) => auc_bid_placed.push(auc),
            Decoded::AuctionBidDeclined(a) => auc_bid_declined.push(a),
            Decoded::AuctionComplete((a, price)) => {
                auc_complete.push(a);
                prices.push(price);
            }
            Decoded::AuctionCancelled(a) => auc_cancelled.push(a),
            Decoded::RawEventRecord(e) => raw_events.push(e),
            Decoded::AuctionRulesChanged(rules) => fees_update.push(rules),
            Decoded::DirectSellDeployed((ds, offer)) => {
                direct_sell_deployed.push(ds);
                deployed_offers.push(offer);
            }
            Decoded::DirectSellStateChanged((ds, price)) => {
                direct_sell_state_changed.push(ds);
                if let Some(price) = price {
                    prices.push(price);
                }
            }
            Decoded::DirectBuyDeployed((db, offer)) => {
                direct_buy_deployed.push(db);
                deployed_offers.push(offer);
            }
            Decoded::DirectBuyStateChanged((db, price)) => {
                direct_buy_state_changed.push(db);
                if let Some(price) = price {
                    prices.push(price);
                }
            }
            Decoded::ShouldSkip => (),
        }
    }

    log::info!(
        r#"
        Saving events (total raw events: {}),
        collections: {},
        nft_created: {},
        nft_burned: {},
        nft_owner_changed: {},
        nft_manager_changed: {},
        auc_deployed: {},
        auc_active: {},
        prices: {},
        auc_bid_placed: {},
        auc_bid_declined: {},
        auc_complete: {},
        auc_cancelled: {},
        fees_update: {},
        direct_sell_deployed: {},
        direct_sell_state_changed: {},
        direct_buy_deployed: {},
        direct_buy_state_changed: {},
        deployed_offers: {},
        "#,
        raw_events.len(),
        collections.len(),
        nft_created.len(),
        nft_burned.len(),
        nft_owner_changed.len(),
        nft_manager_changed.len(),
        auc_deployed.len(),
        auc_active.len(),
        prices.len(),
        auc_bid_placed.len(),
        auc_bid_declined.len(),
        auc_complete.len(),
        auc_cancelled.len(),
        fees_update.len(),
        direct_sell_deployed.len(),
        direct_sell_state_changed.len(),
        direct_buy_deployed.len(),
        direct_buy_state_changed.len(),
        deployed_offers.len(),
    );

    // IMPORTANT: Order matters!

    collections_queue.add_collections(collections).await?;

    let mut pg_pool_tx = pool.begin().await?;

    if !fees_update.is_empty() {
        update_collection_fee(&mut pg_pool_tx, &fees_update).await?;
    }

    if !raw_events.is_empty() {
        save_raw_event(&mut pg_pool_tx, &raw_events).await?;
    }

    if !nft_created.is_empty() {
        save_nft_created(&mut pg_pool_tx, &nft_created).await?;
    };

    if !nft_burned.is_empty() {
        save_nft_burned(&mut pg_pool_tx, &nft_burned).await?;
    }

    if !nft_owner_changed.is_empty() {
        save_nft_owner_changed(&mut pg_pool_tx, &mut nft_owner_changed).await?;
    }

    if !nft_manager_changed.is_empty() {
        save_nft_manager_changed(&mut pg_pool_tx, &mut nft_manager_changed).await?;
    }

    if !deployed_offers.is_empty() {
        save_deployed_offers(&mut pg_pool_tx, &deployed_offers).await?;
    }

    if !auc_deployed.is_empty() {
        save_auc_deployed(&mut pg_pool_tx, &auc_deployed).await?;
    }

    if !auc_active.is_empty() {
        save_auc_active(&mut pg_pool_tx, &auc_active).await?;
    }

    if !auc_bid_placed.is_empty() {
        save_auc_bid(&mut pg_pool_tx, &auc_bid_placed).await?;
        update_auc_maxmin(&mut pg_pool_tx, &auc_bid_placed).await?;
    }

    if !auc_bid_declined.is_empty() {
        save_auc_bid(&mut pg_pool_tx, &auc_bid_declined).await?;
    }

    if !auc_complete.is_empty() {
        save_auc_complete(&mut pg_pool_tx, &auc_complete).await?;
    }

    if !auc_cancelled.is_empty() {
        save_auc_cancelled(&mut pg_pool_tx, &auc_cancelled).await?;
    }

    if !direct_buy_deployed.is_empty() {
        save_direct_buy(&mut pg_pool_tx, &direct_buy_deployed).await?;
    }

    if !direct_sell_deployed.is_empty() {
        save_direct_sell(&mut pg_pool_tx, &direct_sell_deployed).await?;
    }

    if !direct_sell_state_changed.is_empty() {
        update_direct_sell_state(&mut pg_pool_tx, &mut direct_sell_state_changed).await?;
    }

    if !direct_buy_state_changed.is_empty() {
        update_direct_buy_state(&mut pg_pool_tx, &mut direct_buy_state_changed).await?;
    }

    if !prices.is_empty() {
        for price in prices.iter_mut() {
            price.usd_price = price_reader
                .get_current_usd_price(
                    price.price_token.as_str(),
                    price.created_at.timestamp() as u64,
                )
                .await;
        }
        save_price_history(&mut pg_pool_tx, &prices).await?;
    }

    pg_pool_tx.commit().await?;

    Ok(())
}

macro_rules! try_unpack_entity {
    ($msg:ident, $($entity:ty),+) => {
        match $msg.name.as_str() {
            $(stringify!($entity) => Ok(
                Some(Box::new(UnpackAbiPlain::<$entity>::unpack($msg.tokens.clone())?))
            ),)+
            _ => Ok(None),
        }
    };
}

fn unpack_entity(event: &ExtractedOwned) -> Result<Option<Box<dyn Decode>>> {
    try_unpack_entity!(
        event,
        /* FactoryAuction */
        AuctionDeployed,
        AuctionDeclined,
        /* Auction */
        AuctionCreated,
        AuctionActive,
        BidPlaced,
        BidDeclined,
        AuctionComplete,
        AuctionCancelled,
        /* Collection */
        NftCreated,
        NftBurned,
        /* DirectBuy */
        DirectBuyStateChanged,
        /* DirectSell */
        DirectSellStateChanged,
        /* FactoryDirectBuy */
        DirectBuyDeployed,
        DirectBuyDeclined,
        /* FactoryDirectSell */
        DirectSellDeployed,
        DirectSellDeclined,
        /* Nft */
        ManagerChanged,
        OwnerChanged,
        /* common for all events */
        OwnershipTransferred,
        MarketFeeDefaultChanged,
        MarketFeeChanged,
        AddCollectionRules,
        RemoveCollectionRules
    )
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use nekoton_abi::{transaction_parser::ExtractedOwned, PackAbiPlain, UnpackAbiPlain};
    use num::{BigInt, BigUint};
    use ton_abi::{Int, Param, ParamType, Token, TokenValue, Uint};
    use ton_block::{Grams, Message, MsgAddrStd, MsgAddress, Transaction};
    use ton_types::{Cell, UInt256};

    use crate::{abi::scope::events, models::events::*, parser::unpack_entity};

    fn create_default_token_value(param_kind: &ParamType) -> TokenValue {
        match &param_kind {
            ParamType::Uint(size) => TokenValue::Uint(Uint::new(1, *size)),
            ParamType::Int(size) => TokenValue::Int(Int::new(1, *size)),
            ParamType::VarUint(size) => TokenValue::VarUint(*size, BigUint::new(vec![1])),
            ParamType::VarInt(size) => TokenValue::VarInt(
                *size,
                BigInt::new(bigdecimal::num_bigint::Sign::Plus, Vec::default()),
            ),
            ParamType::Bool => TokenValue::Bool(false),
            ParamType::Tuple(v) => {
                let mut tokens = Vec::with_capacity(v.len());
                for p in v {
                    let token_value = create_default_token_value(&p.kind);
                    tokens.push(Token {
                        name: p.name.clone(),
                        value: token_value,
                    });
                }
                TokenValue::Tuple(tokens)
            }
            ParamType::Array(v) => TokenValue::Array((**v).clone(), Vec::default()),
            ParamType::FixedArray(v, size) => {
                TokenValue::FixedArray((**v).clone(), vec![create_default_token_value(v); *size])
            }
            ParamType::Cell => TokenValue::Cell(Cell::default()),
            ParamType::Map(k, v) => {
                TokenValue::Map((**k).clone(), (**v).clone(), BTreeMap::default())
            }
            ParamType::Address => TokenValue::Address(MsgAddress::AddrStd(MsgAddrStd::default())),
            ParamType::Bytes => TokenValue::Bytes(Vec::default()),
            ParamType::FixedBytes(_) => TokenValue::FixedBytes(Vec::default()),
            ParamType::String => TokenValue::String(String::default()),
            ParamType::Token => TokenValue::Token(Grams::default()),
            ParamType::Time => TokenValue::Time(0),
            ParamType::Expire => TokenValue::Expire(0),
            ParamType::PublicKey => TokenValue::PublicKey(None),
            ParamType::Optional(v) => {
                TokenValue::Optional((**v).clone(), Some(Box::new(create_default_token_value(v))))
            }
            ParamType::Ref(v) => TokenValue::Ref(Box::new(create_default_token_value(v))),
        }
    }

    fn build_default_event(params: &Vec<Param>) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(params.len());

        for param in params {
            let token_value = create_default_token_value(&param.kind);
            tokens.push(Token {
                name: param.name.clone(),
                value: token_value,
            });
        }

        tokens
    }

    #[test]
    fn test_correct_parsing() {
        let mut total_events_parsed = 0;

        let auction_root_tip3_contract =
            ton_abi::Contract::load(include_str!("abi/json/FactoryAuction.abi.json")).unwrap();
        let auction_tip3_contract =
            ton_abi::Contract::load(include_str!("abi/json/Auction.abi.json")).unwrap();
        let callbacks_contract =
            ton_abi::Contract::load(include_str!("abi/json/Callbacks.abi.json")).unwrap();
        let collection_contract =
            ton_abi::Contract::load(include_str!("abi/json/Collection.abi.json")).unwrap();
        let direct_buy_contract =
            ton_abi::Contract::load(include_str!("abi/json/DirectBuy.abi.json")).unwrap();
        let direct_sell_contract =
            ton_abi::Contract::load(include_str!("abi/json/DirectSell.abi.json")).unwrap();
        let factory_direct_buy_contract =
            ton_abi::Contract::load(include_str!("abi/json/FactoryDirectBuy.abi.json")).unwrap();
        let factory_direct_sell_contract =
            ton_abi::Contract::load(include_str!("abi/json/FactoryDirectSell.abi.json")).unwrap();
        let mint_and_sell_contract =
            ton_abi::Contract::load(include_str!("abi/json/MintAndSell.abi.json")).unwrap();
        let nft_contract = ton_abi::Contract::load(include_str!("abi/json/Nft.abi.json")).unwrap();

        let mut nft_events = auction_root_tip3_contract.events;
        nft_events.extend(auction_tip3_contract.events);
        nft_events.extend(callbacks_contract.events);
        nft_events.extend(nft_contract.events);
        nft_events.extend(collection_contract.events);
        nft_events.extend(direct_buy_contract.events);
        nft_events.extend(direct_sell_contract.events);
        nft_events.extend(factory_direct_buy_contract.events);
        nft_events.extend(factory_direct_sell_contract.events);
        nft_events.extend(mint_and_sell_contract.events);

        for (name, event) in nft_events {
            let event_raw = build_default_event(event.input_params());

            let extracted = ExtractedOwned {
                function_id: 0,
                name: name.clone(),
                bounced: false,
                tokens: event_raw.clone(),
                message_hash: UInt256::default(),
                message: Message::default(),
                tx: Transaction::default(),
                is_in_message: false,
                parsed_type: nekoton_abi::transaction_parser::ParsedType::Event,
                decoded_headers: Vec::default(),
            };

            let unpacked_event = match unpack_entity(&extracted) {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed parsing event {}: {:#?}", name, e);
                    continue;
                }
            };

            macro_rules! repack_event {
                ($name:ident, $event_raw:ident, $($entity:ty),+) => {
                    match $name.as_str() {
                        $(stringify!($entity) => {
                            let unpacked_static =
                                UnpackAbiPlain::<$entity>::unpack(event_raw.clone()).unwrap();
                            PackAbiPlain::pack(unpacked_static)
                        })+
                        _ => panic!("Unknown event, might be missing"),
                    }
                };
            }

            if unpacked_event.is_some() {
                total_events_parsed += 1;

                let packed_event = repack_event!(
                    name,
                    event_raw,
                    /* FactoryAuction */
                    AuctionDeployed,
                    AuctionDeclined,
                    /* Auction */
                    AuctionCreated,
                    AuctionActive,
                    BidPlaced,
                    BidDeclined,
                    AuctionComplete,
                    AuctionCancelled,
                    /* Collection */
                    NftCreated,
                    NftBurned,
                    /* DirectBuy */
                    DirectBuyStateChanged,
                    /* DirectSell */
                    DirectSellStateChanged,
                    /* FactoryDirectBuy */
                    DirectBuyDeployed,
                    DirectBuyDeclined,
                    /* FactoryDirectSell */
                    DirectSellDeployed,
                    DirectSellDeclined,
                    /* Nft */
                    ManagerChanged,
                    OwnerChanged,
                    /* common for all events */
                    OwnershipTransferred,
                    MarketFeeDefaultChanged,
                    MarketFeeChanged,
                    AddCollectionRules,
                    RemoveCollectionRules
                );

                assert_eq!(packed_event.len(), event_raw.len());
            }
        }

        assert_eq!(total_events_parsed, events().len())
    }
}
