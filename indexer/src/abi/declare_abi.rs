use once_cell::sync::OnceCell;
use ton_abi::Contract;

macro_rules! declare_abi {
    ($($contract:ident => $source:literal),*$(,)?) => {$(
        pub fn $contract() -> &'static Contract {
            static ABI: OnceCell<Contract> = OnceCell::new();
            ABI.load(include_str!($source))
        }
    )*};
}

declare_abi! {
    auction_root_tip3 => "json/FactoryAuction.abi.json",
    auction_tip3 => "json/Auction.abi.json",
    callbacks => "json/Callbacks.abi.json",
    collection => "json/Collection.abi.json",
    direct_buy => "json/DirectBuy.abi.json",
    direct_sell => "json/DirectSell.abi.json",
    factory_direct_buy => "json/FactoryDirectBuy.abi.json",
    factory_direct_sell => "json/FactoryDirectSell.abi.json",
    mint_and_sell => "json/MintAndSell.abi.json",
    nft => "json/Nft.abi.json"
}

trait OnceCellExt {
    fn load(&self, data: &str) -> &Contract;
}

impl OnceCellExt for OnceCell<Contract> {
    fn load(&self, data: &str) -> &Contract {
        self.get_or_init(|| Contract::load(data).expect("Trust me"))
    }
}
