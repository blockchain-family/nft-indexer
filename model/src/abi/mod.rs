use ton_abi::Contract;
pub const DEX_PAIR_ABI: &str = include_str!("AuctionRootTip3.abi.json");
pub const DEX_PAIR_ABI2: &str = include_str!("FactoryDirectBuy.abi.json");
pub const DEX_PAIR_ABI4: &str = include_str!("FactoryDirectSell.abi.json");
pub const _DEX_ROOT_ADDRESS: &str =
    "0:943bad2e74894aa28ae8ddbe673be09a0f3818fd170d12b4ea8ef1ea8051e940";

lazy_static::lazy_static! {
    /// This is an example for using doc comment attributes
    static ref EXAMPLE: Contract = Contract::load(include_str!("AuctionRootTip3.abi.json"))
        .expect("err AuctionRootTip3.abi.json");
}
