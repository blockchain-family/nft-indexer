use anyhow::{anyhow, Result};
use nekoton_abi::{FunctionBuilder, FunctionExt, UnpackFirst};
use nekoton_utils::SimpleClock;
use ton_block::{MsgAddrStd, MsgAddressInt};
use transaction_consumer::JrpcClient;

#[derive(Clone)]
pub struct MetadataJrpcService {
    jrpc_client: JrpcClient,
}

impl MetadataJrpcService {
    pub fn new(jrpc_client: JrpcClient) -> Self {
        Self { jrpc_client }
    }

    pub async fn get_nft_meta(&self, address: &MsgAddressInt) -> Result<serde_json::Value> {
        let contract = self
            .jrpc_client
            .get_contract_state(address)
            .await?
            .ok_or_else(|| anyhow!("Contract state is none!"))?;

        let metadata =
            nekoton_contracts::tip4_2::MetadataContract(contract.as_context(&SimpleClock));

        Ok(serde_json::from_str::<serde_json::Value>(
            &metadata.get_json()?,
        )?)
    }

    fn owner() -> ton_abi::Function {
        FunctionBuilder::new("owner")
            .abi_version(ton_abi::contract::ABI_VERSION_2_2)
            .default_headers()
            .output("value0", ton_abi::ParamType::Address)
            .build()
    }

    fn get_owner() -> ton_abi::Function {
        FunctionBuilder::new("getOwner")
            .abi_version(ton_abi::contract::ABI_VERSION_2_2)
            .default_headers()
            .output("value0", ton_abi::ParamType::Address)
            .build()
    }

    pub async fn get_collection_meta(
        &self,
        collection: MsgAddressInt,
    ) -> Result<(Option<String>, serde_json::Value)> {
        let contract = self
            .jrpc_client
            .get_contract_state(&collection)
            .await?
            .ok_or_else(|| anyhow!("Contract state is none!"))?;

        let metadata =
            nekoton_contracts::tip4_2::MetadataContract(contract.as_context(&SimpleClock));

        let meta = serde_json::from_str::<serde_json::Value>(&metadata.get_json()?)?;

        let owner_contract =
            MetadataJrpcService::owner().run_local(&SimpleClock, contract.account.clone(), &[])?;

        let owner = owner_contract
            .tokens
            .map(|t| {
                t.unpack_first::<MsgAddrStd>()
                    .map(MsgAddressInt::AddrStd)
                    .map(|a| a.to_string())
            })
            .transpose()
            .map_err(|e| {
                log::error!(
                    "Can't get collection {} owner with 'owner' method: {:#?}",
                    collection.to_string(),
                    e
                );
                e
            })
            .ok();

        if let Some(Some(owner)) = owner {
            Ok((Some(owner), meta))
        } else {
            let get_owner_contract =
                MetadataJrpcService::get_owner().run_local(&SimpleClock, contract.account, &[])?;

            let owner = get_owner_contract
                .tokens
                .map(|t| {
                    t.unpack_first::<MsgAddrStd>()
                        .map(MsgAddressInt::AddrStd)
                        .map(|a| a.to_string())
                })
                .transpose()
                .map_err(|e| {
                    log::error!(
                        "Can't get collection {} owner with 'getOwner' method: {:#?}",
                        collection.to_string(),
                        e
                    );
                    e
                })
                .ok();

            Ok((owner.unwrap_or_default(), meta))
        }
    }
}
