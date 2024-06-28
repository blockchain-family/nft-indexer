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
