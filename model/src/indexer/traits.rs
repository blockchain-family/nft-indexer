use anyhow::Result;
use nekoton_abi::transaction_parser::ExtractedOwned;

pub trait ContractEvent {
    fn build_from(event: &ExtractedOwned) -> Result<Self>
    where
        Self: Sized;
}
