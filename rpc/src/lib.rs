use anyhow::{anyhow, Result};
use nekoton_abi::{num_bigint::BigUint, FunctionBuilder, FunctionExt, UnpackFirst};
use nekoton_contracts::tip4_1::nft_contract::GetInfoOutputs;
use nekoton_utils::SimpleClock;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use ton_block::{MsgAddrStd, MsgAddressInt};
use transaction_consumer::JrpcClient;

pub mod retrier;

pub async fn get_json(
    address: MsgAddressInt,
    jrpc_client: JrpcClient,
) -> Result<serde_json::Value> {
    log::debug!("metadata 1");
    let contract = jrpc_client
        .get_contract_state(&address)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    log::debug!("metadata 2");
    let metadata = nekoton_contracts::tip4_2::MetadataContract(
        contract.as_context(&nekoton_utils::SimpleClock),
    );

    log::debug!("metadata 2 json {:?}", &metadata.get_json());

    Ok(serde_json::from_str::<serde_json::Value>(
        &metadata.get_json()?,
    )?)
}

pub async fn get_info(nft: MsgAddressInt, jrpc_client: JrpcClient) -> Result<GetInfoOutputs> {
    let contract = jrpc_client
        .get_contract_state(&nft)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let nft_contract =
        nekoton_contracts::tip4_1::NftContract(contract.as_context(&nekoton_utils::SimpleClock));

    nft_contract.get_info()
}

fn get_owner_function() -> ton_abi::Function {
    FunctionBuilder::new("owner")
        .abi_version(ton_abi::contract::ABI_VERSION_2_2)
        .default_headers()
        .output("value0", ton_abi::ParamType::Address)
        .build()
}

fn get_next_bid_value_function() -> ton_abi::Function {
    FunctionBuilder::new("nextBidValue")
        .abi_version(ton_abi::contract::ABI_VERSION_2_2)
        .default_headers()
        .output("nextBidValue", ton_abi::ParamType::Uint(128))
        .build()
}

pub async fn owner(collection: MsgAddressInt, jrpc_client: JrpcClient) -> Result<String> {
    let contract = jrpc_client
        .get_contract_state(&collection)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let output = get_owner_function().run_local(&SimpleClock, contract.account, &[])?;

    Ok("0:".to_string()
        + &output
            .tokens
            .ok_or_else(|| anyhow!("No tokens in output"))?
            .unpack_first::<MsgAddrStd>()?
            .address
            .as_hex_string())
}

pub async fn next_bid_value(auction: MsgAddressInt, jrpc_client: JrpcClient) -> Result<BigDecimal> {
    let contract = jrpc_client
        .get_contract_state(&auction)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let output = get_next_bid_value_function().run_local(&SimpleClock, contract.account, &[])?;

    Ok(BigDecimal::from_str(
        &output
            .tokens
            .ok_or_else(|| anyhow!("No tokens in output"))?
            .unpack_first::<BigUint>()?
            .to_string(),
    )?)
}

pub async fn token_to_usd(token: &str) -> Result<BigDecimal> {
    let client = reqwest::Client::new();
    let response = crate::retrier::Retrier::new(move || {
        let request = client.post(format!("https://api.flatqube.io/v1/currencies/{token}"));
        Box::pin(request.send())
    })
    .attempts(1)
    //.backoff(10)
    //.factor(2)
    .trace_id(format!("usd price for {}", token))
    .run()
    .await?;

    let object: serde_json::Value = serde_json::from_slice(&response.bytes().await?)?;
    let usd = object
        .get("price")
        .ok_or_else(|| anyhow!("No price in response"))?
        .as_str()
        .ok_or_else(|| anyhow!("Can't get price"))?;

    Ok(BigDecimal::from_str(usd)?)
}
