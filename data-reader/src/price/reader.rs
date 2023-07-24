use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use bigdecimal::{num_bigint::BigInt, BigDecimal};
use indexer_repo::{
    price::{NftPriceModel, RowWithoutUsdPrice},
    types::BcName,
};
use sqlx::PgPool;

use super::price_requester::{request_prices, PriceInfo};

const NFT_PER_ITERATION: i64 = 100;

pub struct PriceReaderContext {
    pub pool: PgPool,
    pub bc: BcName,
    pub idle_after_loop: u64,
}

pub async fn run_price_reader(ctx: PriceReaderContext) {
    let http_client = reqwest::Client::new();
    let model = NftPriceModel::new(ctx.pool);

    loop {
        // WARN: there are a lot of rows with created_at in the future or equal zero
        let direct_buy_without_prices = model
            .get_direct_buy_without_price_usd(NFT_PER_ITERATION)
            .await;
        let direct_sell_without_prices = model
            .get_direct_sell_without_price_usd(NFT_PER_ITERATION)
            .await;
        let auction_without_prices = model.get_auction_without_price_usd(NFT_PER_ITERATION).await;

        let iterator = direct_buy_without_prices
            .iter()
            .chain(direct_sell_without_prices.iter())
            .chain(auction_without_prices.iter())
            .flat_map(|e| e.iter());

        let Some((from, to)) = get_price_time_bounds(iterator.clone()) else {
            log::debug!("Empty iterator - waiting for the next loop");
            tokio::time::sleep(Duration::from_secs(ctx.idle_after_loop)).await;

            continue;
        };

        let token_addresses = iterator
            .clone()
            .map(|v| v.token_addr.as_str())
            .collect::<HashSet<_>>();

        for token_addr in token_addresses {
            let Ok(pool_info) = model.get_dex_pair_address(token_addr, ctx.bc).await else {
                log::error!("Error while reading dex pair by token address {token_addr}");
                continue;
            };

            let Ok(prices) =
                request_prices(&http_client, from, to, &pool_info.address, ctx.bc).await else {
                log::error!("Error while requesting prices to dex for address {}", &pool_info.address);
                continue;
            };

            log::debug!("Req time {} - {}", from, to);
            let price_dict = prices
                .iter()
                .map(|e| (e.timestamp, e))
                .collect::<HashMap<i64, &PriceInfo>>();

            let multiplier =
                BigDecimal::from(BigInt::from(10).pow(pool_info.decimals.try_into().unwrap()));

            for nft in iterator.clone().filter(|e| e.token_addr == token_addr) {
                let closest_hour = get_closest_hour(nft.created_at);
                let Some(hist_token_usd_price) = price_dict.get(&closest_hour) else {
                    log::debug!("Can't find price for token {token_addr} time: {closest_hour}");

                    continue;
                };

                let token_usd_price = {
                    if pool_info.is_l2r {
                        &nft.token_amount * &hist_token_usd_price.close / &multiplier
                    } else {
                        &nft.token_amount / &hist_token_usd_price.close / &multiplier
                    }
                };

                if let Err(e) = model
                    .update_usd_price(&nft.id, &token_usd_price, &nft.source)
                    .await
                {
                    log::error!("Error while saving token {token_addr} usd price: {e:?}");
                }
            }
        }

        log::debug!("Loop finished");
        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}

fn get_closest_hour(time: i64) -> i64 {
    let seconds = time % 3600;
    if seconds > 1800 {
        time + 3600 - seconds
    } else {
        time - seconds
    }
}

fn get_price_time_bounds<'a, I>(iter: I) -> Option<(i64, i64)>
where
    I: Iterator<Item = &'a RowWithoutUsdPrice> + Clone,
{
    let Some(to) = iter.clone().max_by_key(|e| e.created_at) else {
        return None
    };

    // min_by_key returns None when iterator is empty. This case has been handled in the previous step.
    let from = iter.min_by_key(|e| e.created_at).unwrap();
    let hour_in_sec = 3600;

    Some((
        get_closest_hour(from.created_at) - hour_in_sec,
        get_closest_hour(to.created_at) + hour_in_sec,
    ))
}
