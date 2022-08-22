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
/*
Задеплоил в devnet:

✔️ AuctionRootTip3 owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
AuctionRootTip3: 0:7fb89b9013c728bafa0e67ef34769346c6b6ecdbe9cb76800bab15fa4bd47418
✔️ FactoryDirectBuy owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
FactoryDirectBuy: 0:6411580f5476efc89d2d80242570003c919053edee988eb549366a85b3ae905d
✔️ FactoryDirectSell owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
FactoryDirectSell: 0:94650d5a07e302d4588ddd40b6365e7892b98c077cb9db507a543451567d9b34
*/