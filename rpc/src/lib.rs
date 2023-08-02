use anyhow::{anyhow, Result};
use nekoton_abi::{FunctionBuilder, FunctionExt, UnpackFirst};
use nekoton_contracts::tip4_1::nft_contract::GetInfoOutputs;
use nekoton_utils::SimpleClock;
use ton_block::{MsgAddrStd, MsgAddressInt};
use transaction_consumer::JrpcClient;

pub mod retrier;

pub async fn get_json(
    address: MsgAddressInt,
    jrpc_client: JrpcClient,
) -> Result<serde_json::Value> {
    let contract = jrpc_client
        .get_contract_state(&address)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let metadata = nekoton_contracts::tip4_2::MetadataContract(
        contract.as_context(&nekoton_utils::SimpleClock),
    );

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

pub async fn owner(collection: MsgAddressInt, jrpc_client: JrpcClient) -> Result<String> {
    let contract = jrpc_client
        .get_contract_state(&collection)
        .await?
        .ok_or_else(|| anyhow!("Contract state is none!"))?;

    let output = get_owner_function().run_local(&SimpleClock, contract.account, &[])?;

    Ok(output
        .tokens
        .ok_or_else(|| anyhow!("No tokens in output"))?
        .unpack_first::<MsgAddrStd>()
        .map(MsgAddressInt::AddrStd)?
        .to_string())
}
