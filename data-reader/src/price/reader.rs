use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use bigdecimal::{num_bigint::BigInt, BigDecimal};
use indexer_repo::{
    price::{NftPriceModel, RowWithoutUsdPrice},
    types::BcName,
};
use nekoton_utils::TrustMe;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::PgPool;
use tokio::sync::RwLock;

use super::price_requester::{request_prices, PriceInfo};

const NFT_PER_ITERATION: i64 = 1000;

pub struct PriceReader {
    pub model: NftPriceModel,
    pub http_client: reqwest::Client,
    pub bc: BcName,
    pub dex_host_url: String,
    pub idle_after_loop: u64,
    pub price_update_frequency: u64,
    pub current_prices: RwLock<HashMap<String, Option<BigDecimal>>>,
    pub last_update: AtomicU64,
}

impl PriceReader {
    pub async fn new(
        pool: PgPool,
        bc: BcName,
        dex_host_url: String,
        idle_after_loop: u64,
        price_update_frequency_secs: u64,
    ) -> Arc<Self> {
        let model = NftPriceModel::new(pool.clone());
        let http_client = reqwest::Client::new();

        let tokens = model
            .get_tokens_with_dex_pair(bc)
            .await
            .expect("Failed getting tokens from DB");

        let current_prices = RwLock::new(tokens.into_iter().map(|t| (t, None)).collect());

        let last_update = AtomicU64::new(
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .trust_me()
                .as_secs(),
        );

        let reader = Arc::new(Self {
            model,
            http_client,
            bc,
            dex_host_url,
            idle_after_loop,
            price_update_frequency: price_update_frequency_secs,
            current_prices,
            last_update,
        });

        tokio::spawn(reader.clone().run_current_price_updater());

        reader
    }

    pub async fn run_db_updater(self: Arc<Self>) {
        loop {
            let prices = self
                .model
                .get_offers_without_price_usd(NFT_PER_ITERATION)
                .await
                .expect("Failed to get prices for update");

            let iterator = prices.iter();

            let Some((from, to)) = PriceReader::get_price_time_bounds(iterator.clone()) else {
                tokio::time::sleep(Duration::from_secs(self.idle_after_loop)).await;
                continue;
            };

            let token_addresses = iterator
                .clone()
                .map(|v| v.token_addr.as_str())
                .collect::<HashSet<_>>();

            for token_addr in token_addresses {
                let Ok(pool_info) = self.model.get_dex_pair_address(token_addr, self.bc).await
                else {
                    log::error!("Error while reading dex pair by token address {token_addr}");
                    continue;
                };

                let Ok(prices) = request_prices(
                    &self.http_client,
                    from,
                    to,
                    &pool_info.address,
                    self.bc,
                    &self.dex_host_url,
                )
                .await
                else {
                    log::error!(
                        "Error while requesting prices to dex for address {}",
                        &pool_info.address
                    );
                    continue;
                };

                let price_dict = prices
                    .iter()
                    .map(|e| (e.timestamp, e))
                    .collect::<HashMap<i64, &PriceInfo>>();

                let multiplier =
                    BigDecimal::from(BigInt::from(10).pow(pool_info.decimals.try_into().unwrap()));

                for nft in iterator.clone().filter(|e| e.token_addr == token_addr) {
                    let closest_hour = PriceReader::get_closest_hour(nft.created_at);
                    let Some(hist_token_usd_price) = price_dict.get(&closest_hour) else {
                        log::error!("Can't find price for token {token_addr} time: {closest_hour}");

                        continue;
                    };

                    let token_usd_price = {
                        if pool_info.is_l2r {
                            &nft.token_amount * &hist_token_usd_price.close / &multiplier
                        } else {
                            &nft.token_amount / &hist_token_usd_price.close / &multiplier
                        }
                    };

                    if let Err(e) = self.model.update_usd_price(&nft.id, &token_usd_price).await {
                        log::error!("Error while saving token {token_addr} usd price: {e:?}");
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(self.idle_after_loop)).await;
        }
    }

    pub async fn get_current_usd_price(&self, token: &str, timestamp: u64) -> Option<BigDecimal> {
        let last_update = self.last_update.load(Ordering::Acquire);

        log::info!(
            "Current prices (last update {}): {:?}",
            NaiveDateTime::from_timestamp_opt(last_update as i64, 0).unwrap_or_default(),
            self.current_prices.read().await
        );

        if last_update.abs_diff(timestamp) > self.price_update_frequency {
            return None;
        }

        self.current_prices
            .read()
            .await
            .get(token)
            .unwrap_or(&None)
            .clone()
    }

    async fn run_current_price_updater(self: Arc<Self>) {
        loop {
            let now = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .trust_me()
                .as_secs();

            {
                let mut current_prices = self.current_prices.write().await;

                for (token, usd_price) in current_prices.iter_mut() {
                    let Ok(pool_info) = self.model.get_dex_pair_address(token, self.bc).await
                    else {
                        log::error!("Error while reading dex pair by token address {token}");

                        *usd_price = None;
                        continue;
                    };

                    let Ok(prices) = request_prices(
                        &self.http_client,
                        now as i64,
                        now as i64,
                        &pool_info.address,
                        self.bc,
                        &self.dex_host_url,
                    )
                    .await
                    else {
                        log::error!(
                            "Error while requesting prices to dex for address {}",
                            &pool_info.address
                        );

                        *usd_price = None;
                        continue;
                    };

                    log::info!("Current prices from dex (now = {}): {:#?}", now, prices);

                    let price_dict = prices.iter().map(|e| (e.timestamp, e)).collect::<HashMap<
                        i64,
                        &PriceInfo,
                    >>(
                    );

                    let multiplier = BigDecimal::from(
                        BigInt::from(10).pow(pool_info.decimals.try_into().unwrap()),
                    );

                    let Some(token_usd_price) = price_dict.get(&((now - (now % 3600)) as i64))
                    else {
                        log::error!("Can't find price for token {token} time: {now}");

                        *usd_price = None;
                        continue;
                    };

                    let one = BigDecimal::from(1);

                    let token_usd_price = {
                        if pool_info.is_l2r {
                            one * &token_usd_price.close / &multiplier
                        } else {
                            one / &token_usd_price.close / &multiplier
                        }
                    };

                    *usd_price = Some(token_usd_price);
                }
            }

            self.last_update.store(now, Ordering::Release);

            tokio::time::sleep(Duration::from_secs(self.price_update_frequency)).await;
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
            return None;
        };

        // min_by_key returns None when iterator is empty. This case has been handled in the previous step.
        let from = iter.min_by_key(|e| e.created_at).unwrap();
        let hour_in_sec = 3600;

        Some((
            PriceReader::get_closest_hour(from.created_at) - hour_in_sec,
            PriceReader::get_closest_hour(to.created_at) + hour_in_sec,
        ))
    }
}
