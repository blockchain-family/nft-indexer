use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};


/// GET /auctions
pub fn get_auctions(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auctions")
        .and(warp::get())
        .and(warp::query::<AuctionsQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_auctions_handler)
}

pub async fn get_auctions_handler(query: AuctionsQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuctionsQuery {
    pub owners: Vec<Address>,
    pub collections: Vec<Address>,
    pub tokens: Vec<Address>,
    pub sort: Option<AuctionsSortOrder>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuctionsSortOrder {
    #[serde(rename = "start-date")]
    StartDate,
    #[serde(rename = "bids-count")]
    BidsCount,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "average-in-hour")]
    AverageInHour,
    #[serde(rename = "average-in-day")]
    AverageInDay,
}

/// GET /auction/{address}/bids
pub fn get_auction_bids(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auction" / Address / "bids")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_auction_bids_handler)
}

pub async fn get_auction_bids_handler(auction: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}