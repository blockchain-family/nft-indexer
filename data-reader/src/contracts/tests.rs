use super::access::*;

use everscale_rpc_client::{ClientOptions, RpcClient};
use nekoton::transport::models::ExistingContract;
use nekoton_utils::SimpleClock;
use reqwest::Url;
use std::str::FromStr;
use ton_block::MsgAddressInt;

async fn get_rpc_client() -> RpcClient {
    RpcClient::new(
        vec![Url::from_str("https://jrpc.venom.foundation/rpc").unwrap()],
        ClientOptions::default(),
    )
    .await
    .unwrap()
}

async fn get_existing_contract(address: &str) -> ExistingContract {
    let rpc_client = get_rpc_client().await;

    let address = MsgAddressInt::from_str(address).unwrap();

    rpc_client
        .get_contract_state(&address, None)
        .await
        .unwrap()
        .unwrap()
}

async fn test_contract_owner(collection: &str, expected_owner: &str) {
    let contract = get_existing_contract(collection).await;
    let ownable_internal_contract = OwnableInternalContract(contract.as_context(&SimpleClock));
    assert_eq!(
        ownable_internal_contract.owner().unwrap().to_string(),
        expected_owner
    );
}

#[tokio::test]
async fn test_contract_owners() {
    test_contract_owner(
        "0:fa200706e21863c1da8f35d334f48faeb39d34ad21efd81178ffab40850c0898",
        "0:02b9ec5c42687b4bfddde4af6205defe32bca8ee75d57b8aa03be5c492dbcd00",
    )
    .await;
    test_contract_owner(
        "0:898de39c6ea5472b7237937d4bdf525ded6eb5b9c967032d2ef7ec527a2a4b1c",
        "0:3bf3b62dd4c2a73d0624d8bfbdb9c13e526369e5b5209b26b5116ddac3a9ef64",
    )
    .await;
}
