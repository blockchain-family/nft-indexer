use crate::contracts::utils::declare_function;
use ton_abi::{Param, ParamType};

/// Returns collection owner address
///
/// # Type
/// Simple getter method
///
/// # Inputs
/// No inputs
///
/// # Outputs
/// * `value0: address` - Address of NFT contract
///
pub fn owner() -> &'static ton_abi::Function {
    declare_function! {
        header: [pubkey, time, expire],
        name: "owner",
        inputs: vec![],
        outputs: vec![Param::new("value0", ParamType::Address)],
    }
}
