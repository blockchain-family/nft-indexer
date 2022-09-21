use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};
use crate::model::{NFT, Contract};


/// GET /nft/{address}/details
pub fn get_nft(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nft" / Address / "details")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_nft_handler)
}

pub async fn get_nft_handler(address: Address, db: ApiService) -> Result<Box<dyn warp::Reply>, Infallible> {
    match db.get_nft_details((&address).into()).await {
        Err(e) => Ok(Box::from(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))),
        Ok(None) => Ok(Box::from(warp::reply::with_status(String::default(), StatusCode::BAD_REQUEST))),
        Ok(Some(nft)) => {
            let ret = NFT {
                contract: Contract { 
                    address: Address::from(nft.address.expect("null nft address")),
                    name: nft.name,
                    description: nft.description,
                    owner: nft.owner.map(Address::from),
                },
                collection: Contract { 
                    address: Address::from(nft.collection.unwrap_or_default()),
                    name: nft.collection_name,
                    description: nft.collection_description,
                    owner: nft.collection_owner.map(Address::from),
                },
                manager: nft.manager.map(Address::from),
                image: "".to_string(),
                attributes: None,
                auction: None,
                forsale: None,
                current_price: None,
                last_price: None,

            };
            Ok(Box::from(warp::reply::with_status(warp::reply::json(&ret), StatusCode::OK)))
        }
    }

}

/// GET /nft/{address}/offers
pub fn get_nft_offers(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nft" / Address / "offers")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_nft_offers_handler)
}

pub async fn get_nft_offers_handler(address: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}


/// GET /nft/{address}/price-history
pub fn get_nft_price_history(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nft" / Address / "price-history")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_nft_price_history_handler)
}

pub async fn get_nft_price_history_handler(address: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

/// POST /nft/{address}/reload-meta
pub fn post_nft_reload_meta(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nft" / Address / "reload-meta")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and_then(post_nft_reload_meta_handler)
}

pub async fn post_nft_reload_meta_handler(address: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

/// GET /nfts/
pub fn get_nft_list(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nfts")
        .and(warp::get())
        .and(warp::query::<NFTListQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_nft_list_handler)
}

pub async fn get_nft_list_handler(params: NFTListQuery, db: ApiService) -> Result<Box<dyn warp::Reply>, Infallible> {
    let owners = params.owners.as_ref().map(|x| x.as_slice()).unwrap_or(&[]);
    let collections = params.collections.as_ref().map(|x| x.as_slice()).unwrap_or(&[]);
    match db.nft_search(
        owners,
        collections,
        params.price_from,
        params.price_to,
        params.price_token.into(),
        params.forsale,
        params.auction,
    ).await {
        Err(e) => Ok(Box::from(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))),
        Ok(_list) => {
            Ok(Box::from(warp::reply::with_status(String::default(), StatusCode::BAD_REQUEST)))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NFTListQuery {
    pub owners: Option<Vec<String>>,

    pub collections: Option<Vec<String>>,

    #[serde(rename = "priceFrom")]
    pub price_from: Option<u64>,

    #[serde(rename = "priceTo")]
    pub price_to: Option<u64>,

    #[serde(rename = "priceToken")]
    pub price_token: Option<String>,

    pub forsale: Option<bool>,
    pub auction: Option<bool>,
}