#![deny(clippy::dbg_macro)]
use std::time::Duration;

mod jrpc_client;

use anyhow::{Context, Result};
pub use self::jrpc_client::LoadBalancedRpcOptions;
use self::jrpc_client::{JrpcRequest, LoadBalancedRpc};
use nekoton::transport::models::{ExistingContract, RawContractState};
use nekoton_utils::SimpleClock;
use serde::Serialize;
use ton_block::MsgAddressInt;
use url::Url;

macro_rules! try_res {
    ($some:expr, $msg:literal) => {
        match $some {
            Ok(a) => a,
            Err(e) => {
                ::log::error!("{}:{:?}", $msg, e);
                continue;
            }
        }
    };
}

macro_rules! try_opt {
    ($some:expr, $msg:literal) => {
        match $some {
            Some(a) => a,
            None => {
                ::log::error!("{}", $msg);
                continue;
            }
        }
    };
}

#[derive(Clone)]
pub struct StatesClient {
    client: LoadBalancedRpc,
}

impl StatesClient {
    pub async fn new<I: std::fmt::Debug, U>(
        states_rpc_endpoint: I,
        options: Option<LoadBalancedRpcOptions>,
    ) -> Result<StatesClient>
    where
        I: IntoIterator<Item = U>,
        U: AsRef<str>,
    {
        let endpoints: Result<Vec<_>, _> = states_rpc_endpoint
            .into_iter()
            .map(|x| 
                Url::parse(x.as_ref()).and_then(|x| x.join("/rpc"))
            )
            .collect();
        let options = options.unwrap_or(LoadBalancedRpcOptions {
            prove_interval: Duration::from_secs(10),
        });
        let client = LoadBalancedRpc::new(endpoints.context("Bad endpoints")?, options).await?;

        Ok(Self { client })
    }

    pub async fn get_contract_state(
        &self,
        contract_address: &MsgAddressInt,
    ) -> Result<Option<ExistingContract>> {
        #[derive(Serialize)]
        struct Request {
            address: String,
        }

        let req = Request {
            address: contract_address.to_string(),
        };

        let req = JrpcRequest {
            id: 13,
            method: "getContractState",
            params: req,
        };

        let response = self.client.request(req).await;
        let parsed: RawContractState = response.unwrap()?;
        let response = match parsed {
            RawContractState::NotExists => None,
            RawContractState::Exists(c) => Some(c),
        };
        Ok(response)
    }

    pub async fn run_local(
        &self,
        contract_address: &MsgAddressInt,
        function: &ton_abi::Function,
        input: &[ton_abi::Token],
    ) -> Result<Option<nekoton_abi::ExecutionOutput>> {
        use nekoton_abi::FunctionExt;

        let state = match self.get_contract_state(contract_address).await? {
            Some(a) => a,
            None => return Ok(None),
        };
        function
            .clone()
            .run_local(&SimpleClock, state.account, input)
            .map(Some)
    }
}