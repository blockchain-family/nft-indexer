use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use indexer_repo::types::BcName;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PriceInfo {
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub timestamp: i64,
}

pub async fn request_prices(
    client: &reqwest::Client,
    from: i64,
    to: i64,
    pair_addr: &str,
    bc: BcName,
) -> Result<Vec<PriceInfo>> {
    match bc {
        BcName::Venom => web3world_request_prices(client, from, to, pair_addr).await,
        BcName::Everscale => flatqube_request_prices(client, from, to, pair_addr).await,
    }
}

async fn flatqube_request_prices(
    client: &reqwest::Client,
    from: i64,
    to: i64,
    pool_address: &str,
) -> Result<Vec<PriceInfo>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct FlatqubePriceRequest<'a> {
        from: i64,
        to: i64,
        timeframe: &'static str,
        ohlcv_kind: &'static str,
        pool_address: &'a str,
    }

    client
        .post("https://api.flatqube.io/v2/pools/ohlcv")
        .json(&FlatqubePriceRequest {
            from,
            to,
            timeframe: "H1",
            ohlcv_kind: "Price",
            pool_address,
        })
        .send()
        .await?
        .json::<Vec<PriceInfo>>()
        .await
        .map_err(|e| anyhow!(e))
}

async fn web3world_request_prices(
    client: &reqwest::Client,
    from: i64,
    to: i64,
    pair_address: &str,
) -> Result<Vec<PriceInfo>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Web3WorldPriceRequest {
        from: i64,
        to: i64,
        timeframe: &'static str,
    }
    impl Web3WorldPriceRequest {
        fn new(from: i64, to: i64) -> Self {
            Self {
                from: from * 1000,
                to: to * 1000,
                timeframe: "H1",
            }
        }
    }
    let url = format!("https://testnetapi.web3.world/v1/pairs/address/{pair_address}/ohlcv");

    client
        .post(url)
        .json(&Web3WorldPriceRequest::new(from, to))
        .send()
        .await?
        .json::<Vec<PriceInfo>>()
        .await
        .map_err(|e| anyhow!(e))
        .map(|v| {
            v.into_iter()
                .map(|v| PriceInfo {
                    timestamp: v.timestamp / 1000,
                    ..v
                })
                .collect()
        })
}
