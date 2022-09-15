use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};


/// GET /nft/{address}/details
pub fn get_nft(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("nft" / Address / "details")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_nft_handler)
}

pub async fn get_nft_handler(address: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
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

pub async fn get_nft_list_handler(_params: NFTListQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NFTListQuery {
    pub owners: Vec<Address>,

    pub collections: Vec<Address>,

    #[serde(rename = "priceFrom")]
    pub price_from: Option<u64>,

    #[serde(rename = "priceTo")]
    pub price_to: Option<u64>,

    #[serde(rename = "priceToken")]
    pub price_token: Option<Address>,

    pub forsale: Option<bool>,
    pub auction: Option<bool>,
}