use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};


/// GET /owner/{address}/bids-out
pub fn get_owner_bids_out(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("owner" / Address / "bids-out")
        .and(warp::get())
        .and(warp::query::<OwnerBidsOutQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_owner_bids_out_handler)
}

pub async fn get_owner_bids_out_handler(owner: Address, query: OwnerBidsOutQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerBidsOutQuery {
    pub collections: Vec<Address>,
    pub lastbid: Option<bool>,
}

/// GET /owner/{address}/bids-in
pub fn get_owner_bids_in(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("owner" / Address / "bids-in")
        .and(warp::get())
        .and(warp::query::<OwnerBidsInQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_owner_bids_in_handler)
}

pub async fn get_owner_bids_in_handler(owner: Address, query: OwnerBidsInQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerBidsInQuery {
    pub collections: Vec<Address>,
}

/// GET /owner/{address}/offers-out
pub fn get_owner_offers_out(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("owner" / Address / "offers-out")
        .and(warp::get())
        .and(warp::query::<OwnerOffersQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_owner_offers_out_handler)
}

pub async fn get_owner_offers_out_handler(owner: Address, query: OwnerOffersQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

/// GET /owner/{address}/offers-in
pub fn get_owner_offers_in(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("owner" / Address / "offers-in")
        .and(warp::get())
        .and(warp::query::<OwnerOffersQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_owner_offers_in_handler)
}

pub async fn get_owner_offers_in_handler(owner: Address, query: OwnerOffersQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerOffersQuery {
    pub collections: Vec<Address>,
    pub active: Option<bool>,
}