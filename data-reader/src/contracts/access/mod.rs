use anyhow::Result;
use nekoton_abi::*;

use super::RunLocalSimple;

pub mod ownable_contract;

#[derive(Copy, Clone)]
pub struct OwnableContract<'a>(pub ExecutionContext<'a>);

impl OwnableContract<'_> {
    pub fn owner(&self) -> Result<ton_block::MsgAddressInt> {
        let result = self
            .0
            .run_local_simple(ownable_contract::owner(), &[])?
            .unpack_first()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::OwnableContract;
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

    #[tokio::test]
    async fn test_contract_owner() {
        let contract = get_existing_contract(
            "0:fa200706e21863c1da8f35d334f48faeb39d34ad21efd81178ffab40850c0898",
        )
        .await;
        let ownable_contract = OwnableContract(contract.as_context(&SimpleClock));
        assert_eq!(
            ownable_contract.owner().unwrap().to_string(),
            "0:02b9ec5c42687b4bfddde4af6205defe32bca8ee75d57b8aa03be5c492dbcd00"
        );
    }
}
