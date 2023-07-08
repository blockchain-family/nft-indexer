use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use serde::Serialize;

use crate::models::types::*;

#[derive(Clone, UnpackAbiPlain, PackAbiPlain, PartialEq, Eq, Debug, Serialize)]
pub struct DirectSellStateChanged {
    #[abi]
    pub from: u8,
    #[abi]
    pub to: u8,
    #[abi]
    pub value2: DirectSellInfo,
}
