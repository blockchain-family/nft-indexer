use once_cell::sync::OnceCell;
use serde_json::Value;
use ton_abi::Contract;

macro_rules! declare_abi {
    ($($contract:ident => $source:literal $( only events($($only_event:literal),+))? $( exclude events($($exclude_event:literal),+))?),*$(,)?) => {$(
        pub fn $contract() -> &'static Contract {
            static ABI: OnceCell<Contract> = OnceCell::new();
            ABI.load(include_str!($source), &[$($($only_event),*),*], &[$($($exclude_event),*),*])
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
    nft => "json/Nft.abi.json" exclude events("NftCreated"),
    collection4_2_2 => "json/Collection4_2_2.abi.json" only events("NftMetadataUpdated", "CollectionMetadataUpdated"),
    nft4_2_2 => "json/Nft4_2_2.abi.json" only events("MetadataUpdated")
}

trait OnceCellExt {
    fn load(&self, data: &str, only_events: &[&str], excluded_events: &[&str]) -> &Contract;
}

impl OnceCellExt for OnceCell<Contract> {
    fn load(&self, data: &str, only_events: &[&str], excluded_events: &[&str]) -> &Contract {
        let mut data: Value = serde_json::from_str(data).unwrap();

        if let Some(events) = data.get_mut("events").and_then(|e| e.as_array_mut()) {
            if !only_events.is_empty() {
                let only_events = only_events
                    .iter()
                    .map(|&e| Some(Value::String(e.to_string())))
                    .collect::<Vec<_>>();
                events.retain(|event| {
                    let name = &event.get("name").cloned();
                    only_events.contains(name)
                });
            }

            for &excluded_event in excluded_events {
                events.retain(|event| {
                    event.get("name") != Some(&Value::String(excluded_event.to_string()))
                });
            }
        }

        self.get_or_init(|| Contract::load_from_json(data).expect("Trust me"))
    }
}

#[cfg(test)]
mod tests {
    use super::{collection, collection4_2_2, nft, nft4_2_2};

    #[test]
    fn ensure_excluded_event() {
        let col = collection();
        let nft = nft();
        let col422 = collection4_2_2();
        let nft422 = nft4_2_2();

        assert!(col.events.contains_key("NftCreated"));
        assert!(col.events.contains_key("NftBurned"));
        assert!(nft.events.contains_key("NftBurned"));
        assert!(col422.events.contains_key("NftMetadataUpdated"));
        assert!(col422.events.contains_key("CollectionMetadataUpdated"));
        assert!(nft422.events.contains_key("MetadataUpdated"));

        assert!(!nft.events.contains_key("NftCreated"));
        assert!(!col422.events.contains_key("NftCreated"));
        assert!(!nft422.events.contains_key("NftCreated"));
    }
}
