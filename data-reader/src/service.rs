use crate::contracts::tip4_1::nft_contract::GetInfoOutputs;
use crate::contracts::{tip4_1, tip4_2, tip4_2_2, tip4_3, tip6};
use anyhow::Result;
use everscale_rpc_client::RpcClient;
use nekoton::transport::models::ExistingContract;
use nekoton_abi::{FunctionBuilder, FunctionExt, UnpackFirst};
use nekoton_utils::{Clock, SimpleClock};
use ton_block::{MsgAddrStd, MsgAddressInt, Serializable};
use ton_types::{BuilderData, Cell, UInt256};

#[derive(Clone)]
pub struct MetadataRpcService {
    rpc_client: RpcClient,
    http_client: reqwest::Client,
}

impl MetadataRpcService {
    pub fn new(rpc_client: RpcClient, http_client: reqwest::Client) -> Self {
        Self {
            rpc_client,
            http_client,
        }
    }

    async fn get_contract_state(&self, address: &MsgAddressInt) -> Result<ExistingContract> {
        self.rpc_client
            .get_contract_state(address, None)
            .await?
            .ok_or_else(|| NftError::ContractNotExist.into())
    }

    async fn read_metadata_from_url(&self, url: &str) -> Result<String> {
        Ok(self.http_client.get(url).send().await?.text().await?)
    }

    async fn json_value_from_json_info(
        &self,
        json_info: Option<JsonInfo>,
    ) -> Result<serde_json::Value> {
        let metadata = match json_info {
            None => return Err(NftError::MissingMetadata.into()),
            Some(JsonInfo::Json(data)) => data,
            Some(JsonInfo::Url(url)) => self.read_metadata_from_url(&url).await?,
        };

        Ok(serde_json::from_str(&metadata)?)
    }

    pub async fn get_nft_meta(&self, address: &MsgAddressInt) -> Result<serde_json::Value> {
        let contract = self.get_contract_state(address).await?;
        let nft_state = NftContractState(&contract);

        let interfaces = match nft_state.check_supported_interfaces(&SimpleClock)? {
            Some(interfaces) => interfaces,
            None => return Err(NftError::InvalidNftContact.into()),
        };

        let info = nft_state.get_info(&SimpleClock)?;

        let collection_contract = self.get_contract_state(&info.collection).await?;

        let collection_state = CollectionContractState(&collection_contract);

        let json_data = interfaces
            .tip4_2
            .then(|| nft_state.get_json(&SimpleClock))
            .transpose()?;

        let json_url = interfaces
            .tip4_2_2
            .then(|| nft_state.get_url_parts(&SimpleClock))
            .transpose()?
            .map(|part| collection_state.get_nft_url(&SimpleClock, part))
            .transpose()?;

        self.json_value_from_json_info(json_data.or(json_url)).await
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
        let contract = self.get_contract_state(&collection).await?;

        let collection_state = CollectionContractState(&contract);

        let interfaces = collection_state.check_collection_supported_interfaces(&SimpleClock)?;
        if !interfaces.tip4_3 {
            return Err(NftError::InvalidCollectionContract.into());
        }

        let ctx = contract.as_context(&SimpleClock);
        let json_data = interfaces
            .tip4_2
            .then(|| tip4_2::MetadataContract(ctx).get_json())
            .transpose()?
            .map(JsonInfo::Json);

        let json_url = interfaces
            .tip4_2_2
            .then(|| tip4_2_2::CollectionContract(ctx).get_collection_url())
            .transpose()?
            .map(JsonInfo::Url);

        let meta = self
            .json_value_from_json_info(json_data.or(json_url))
            .await?;

        let owner_contract =
            MetadataRpcService::owner().run_local(&SimpleClock, contract.account.clone(), &[])?;

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
                MetadataRpcService::get_owner().run_local(&SimpleClock, contract.account, &[])?;

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

const NFT_STAMP: &[u8; 3] = b"nft";

pub enum JsonInfo {
    Json(String),
    Url(String),
}

#[derive(Debug)]
pub struct CollectionContractState<'a>(pub &'a ExistingContract);

impl<'a> CollectionContractState<'a> {
    pub fn check_collection_supported_interfaces(
        &self,
        clock: &dyn Clock,
    ) -> Result<CollectionInterfaces> {
        let ctx = self.0.as_context(clock);
        let tip6_interface = tip6::SidContract(ctx);

        let mut result = CollectionInterfaces::default();

        if tip6_interface.supports_interface(tip4_3::collection_contract::INTERFACE_ID)? {
            result.tip4_3 = true;
            result.tip4_2_2 =
                tip6_interface.supports_interface(tip4_2_2::collection_contract::INTERFACE_ID)?;
            result.tip4_2 =
                tip6_interface.supports_interface(tip4_2::metadata_contract::INTERFACE_ID)?;
        }

        Ok(result)
    }

    pub fn resolve_collection_index_code(&self, clock: &dyn Clock) -> Result<Cell> {
        let ctx = self.0.as_context(clock);
        tip4_3::CollectionContract(ctx).index_code()
    }

    pub fn get_collection_code_hash(
        &self,
        owner: &MsgAddressInt,
        code_index: Cell,
    ) -> Result<UInt256> {
        let mut builder = BuilderData::new();

        let owner_cell = owner.serialize()?;
        let collection_cell = self.0.account.addr.serialize()?;

        builder.append_raw(collection_cell.data(), collection_cell.bit_length())?;
        builder.append_raw(owner_cell.data(), owner_cell.bit_length())?;

        let mut nft = BuilderData::new();
        nft.append_raw(NFT_STAMP, 24)?;

        builder.checked_append_reference(nft.into_cell()?)?;

        let salt = builder.into_cell()?;

        let cell = nekoton_abi::set_code_salt(code_index, salt)?;
        Ok(cell.hash(0))
    }

    pub fn get_nft_url(&self, clock: &dyn Clock, part: Cell) -> Result<JsonInfo> {
        let ctx = self.0.as_context(clock);
        let tip4_2_2_interface = tip4_2_2::CollectionContract(ctx);
        tip4_2_2_interface.get_nft_url(part).map(JsonInfo::Url)
    }
}

#[derive(Copy, Clone, Default)]
pub struct CollectionInterfaces {
    pub tip4_3: bool,
    pub tip4_2_2: bool,
    pub tip4_2: bool,
}

#[derive(Copy, Clone, Default)]
pub struct NftInterfaces {
    pub tip4_3: bool,
    pub tip4_2_2: bool,
    pub tip4_2: bool,
    pub tip4_1: bool,
}

#[derive(Debug)]
pub struct NftContractState<'a>(pub &'a ExistingContract);

impl<'a> NftContractState<'a> {
    pub fn check_supported_interfaces(&self, clock: &dyn Clock) -> Result<Option<NftInterfaces>> {
        let ctx = self.0.as_context(clock);
        let tip6_interface = tip6::SidContract(ctx);

        let result = NftInterfaces {
            tip4_1: tip6_interface.supports_interface(tip4_1::nft_contract::INTERFACE_ID)?,
            tip4_2: tip6_interface.supports_interface(tip4_2::metadata_contract::INTERFACE_ID)?,
            tip4_2_2: tip6_interface
                .supports_interface(tip4_2_2::metadata_contract::INTERFACE_ID)?,
            tip4_3: tip6_interface.supports_interface(tip4_3::nft_contract::INTERFACE_ID)?,
        };

        Ok((result.tip4_1 || result.tip4_3).then_some(result))
    }

    pub fn get_json(&self, clock: &dyn Clock) -> Result<JsonInfo> {
        let ctx = self.0.as_context(clock);
        let tip4_2_interface = tip4_2::MetadataContract(ctx);
        tip4_2_interface.get_json().map(JsonInfo::Json)
    }

    pub fn get_url_parts(&self, clock: &dyn Clock) -> Result<Cell> {
        let ctx = self.0.as_context(clock);
        tip4_2_2::MetadataContract(ctx).get_url_parts()
    }

    pub fn get_info(&self, clock: &dyn Clock) -> Result<GetInfoOutputs> {
        let ctx = self.0.as_context(clock);
        let tip4_1_interface = tip4_1::NftContract(ctx);
        tip4_1_interface.get_info()
    }
}

#[derive(thiserror::Error, Debug)]
enum NftError {
    #[error("Invalid collection contract")]
    InvalidCollectionContract,
    #[error("Invalid nft contract")]
    InvalidNftContact,
    #[error("Missing metadata")]
    MissingMetadata,
    #[error("Contract does not exist")]
    ContractNotExist,
}
