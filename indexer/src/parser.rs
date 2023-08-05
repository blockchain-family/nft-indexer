use crate::models::events::*;
use crate::persistence::entities::*;
use crate::settings;
use crate::utils::EventMessageInfo;
use anyhow::Result;
use futures::channel::mpsc::{Receiver, Sender};
use futures::{future, SinkExt, StreamExt};
use nekoton_abi::transaction_parser::{ExtractedOwned, ParsedType};
use nekoton_abi::UnpackAbiPlain;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::PgPool;
use ton_block::GetRepresentationHash;
use ton_types::UInt256;
use transaction_buffer::models::{BufferedConsumerChannels, RawTransaction};

pub async fn start_parsing(config: settings::config::Config, pg_pool: PgPool) -> Result<()> {
    let BufferedConsumerChannels {
        rx_parsed_events,
        tx_commit,
        notify_for_services,
    } = settings::init_transaction_buffer(&config, &pg_pool).await?;

    log::info!("Connected to kafka");

    tokio::spawn(run_nft_indexer(rx_parsed_events, tx_commit, pg_pool));

    notify_for_services.notified().await;

    future::pending().await
}

pub async fn run_nft_indexer(
    mut rx_raw_transactions: Receiver<Vec<(Vec<ExtractedOwned>, RawTransaction)>>,
    mut tx_commit: Sender<()>,
    pool: PgPool,
) {
    log::info!("Start nft indexer...");

    while let Some(message) = rx_raw_transactions.next().await {
        let mut jobs = Vec::with_capacity(1000);

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

            let msg_info = EventMessageInfo {
                tx_data: tx.data,
                function_inputs,
                message_hash: UInt256::default(),
            };

            for event in events {
                let mut msg_info = msg_info.clone();
                let pool = pool.clone();
                jobs.push(tokio::spawn(async move {
                    if let Err(e) = process_event(event, &mut msg_info, &pool).await {
                        // TODO: check error kind; exit if critical
                        log::error!("Error processing event: {:#?}. Exiting.", e);
                    }
                }));
            }
        }

        log::debug!("Events in transaction: {}", jobs.len());

        futures::future::join_all(jobs).await;

        tx_commit.send(()).await.expect("dead commit sender");
    }

    panic!("rip kafka consumer");
}

async fn process_event(
    event: ExtractedOwned,
    msg_info: &mut EventMessageInfo,
    pool: &PgPool,
) -> Result<()> {
    if let Some((entity, message_hash)) = unpack_entity(&event)? {
        msg_info.message_hash = message_hash;
        log::info!(
            "saving {}, tx hash {:?}, timestamp: {}",
            &event.name,
            msg_info.tx_data.hash().unwrap_or_default(),
            NaiveDateTime::from_timestamp_opt(msg_info.tx_data.now as i64, 0).unwrap_or_default()
        );
        entity.save_to_db(pool, msg_info).await?;
    }

    Ok(())
}

macro_rules! try_unpack_entity {
    ($msg:ident, $($entity:ty),+) => {
        match $msg.name.as_str() {
            $(stringify!($entity) => Ok(
                Some(
                    (Box::new(UnpackAbiPlain::<$entity>::unpack($msg.tokens.clone())?), $msg.message_hash)
                )
            ),)+
            _ => Ok(None),
        }
    };
}

fn unpack_entity(event: &ExtractedOwned) -> Result<Option<(Box<dyn Entity>, UInt256)>> {
    try_unpack_entity!(
        event,
        /* AuctionRootTip3 */
        AuctionDeployed,
        AuctionDeclined,
        /* AuctionTip3 */
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
                TokenValue::FixedArray((**v).clone(), vec![create_default_token_value(&**v); *size])
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
            ParamType::Optional(v) => TokenValue::Optional(
                (**v).clone(),
                Some(Box::new(create_default_token_value(&**v))),
            ),
            ParamType::Ref(v) => TokenValue::Ref(Box::new(create_default_token_value(&**v))),
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
            ton_abi::Contract::load(include_str!("abi/json/AuctionRootTip3.abi.json")).unwrap();
        let auction_tip3_contract =
            ton_abi::Contract::load(include_str!("abi/json/AuctionTip3.abi.json")).unwrap();
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
                        _ => panic!("Unknow event, might be missing"),
                    }
                };
            }

            if unpacked_event.is_some() {
                total_events_parsed += 1;

                let packed_event = repack_event!(
                    name,
                    event_raw,
                    /* AuctionRootTip3 */
                    AuctionDeployed,
                    AuctionDeclined,
                    /* AuctionTip3 */
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
