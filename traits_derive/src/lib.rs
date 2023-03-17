#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::DeriveInput;

#[proc_macro_derive(EventRecord)]
pub fn event_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (event_cat, event_type) = get_event_cat_type(&name);

    let expanded = quote! {
        impl EventRecord for #name {
            fn get_address(&self) -> Address {
                self.address.clone()
            }

            fn get_nft(&self) -> Option<Address> {
                self.event_nft.clone()
            }

            fn get_collection(&self) -> Option<Address> {
                self.event_collection.clone()
            }

            fn get_created_at(&self) -> i64 {
                self.created_at
            }

            fn get_created_lt(&self) -> i64 {
                self.created_lt
            }

            fn get_message_hash(&self) -> String {
                self.message_hash.clone()
            }

            fn get_event_category(&self) -> EventCategory {
                EventCategory::#event_cat
            }

            fn get_event_type(&self) -> EventType {
                EventType::#event_type
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_event_cat_type(name: &Ident) -> (Ident, Ident) {
    let auction_cat = syn::parse_str::<Ident>("Auction").unwrap();
    let direct_sell_cat = syn::parse_str::<Ident>("DirectSell").unwrap();
    let direct_buy_cat = syn::parse_str::<Ident>("DirectBuy").unwrap();
    let nft_cat = syn::parse_str::<Ident>("Nft").unwrap();
    let collection_cat = syn::parse_str::<Ident>("Collection").unwrap();

    match name.to_string().as_str() {
        type_name @ "AuctionDeployed" => (auction_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "AuctionDeclined" => (auction_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "AuctionRootOwnershipTransferred" => {
            (auction_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "AuctionCreated" => (auction_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "AuctionActive" => (auction_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "AuctionBidPlaced" => {
            (auction_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "AuctionBidDeclined" => {
            (auction_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "AuctionComplete" => (auction_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "AuctionCancelled" => {
            (auction_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectBuyDeployed" => {
            (direct_buy_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectBuyDeclined" => {
            (direct_buy_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "FactoryDirectBuyOwnershipTransferred" => {
            (direct_buy_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectSellDeployed" => {
            (direct_sell_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectSellDeclined" => {
            (direct_sell_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "FactoryDirectSellOwnershipTransferred" => {
            (direct_sell_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectBuyStateChanged" => {
            (direct_buy_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "DirectSellStateChanged" => {
            (direct_sell_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "NftOwnerChanged" => (nft_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "NftManagerChanged" => (nft_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "CollectionOwnershipTransferred" => {
            (collection_cat, syn::parse_str::<Ident>(type_name).unwrap())
        }
        type_name @ "NftCreated" => (collection_cat, syn::parse_str::<Ident>(type_name).unwrap()),
        type_name @ "NftBurned" => (collection_cat, syn::parse_str::<Ident>(type_name).unwrap()),

        _ => panic!("Unknow struct!"),
    }
}
