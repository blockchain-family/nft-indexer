use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use serde::{Serialize, Deserialize};


/// GET /collection/{address}/details
pub fn get_collection(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("collection" / Address / "details")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_collection_handler)
}

pub async fn get_collection_handler(owner: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

/// GET /collections/by-owner/{owner}
pub fn get_collections_by_owner(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("collections" / "by-owner" / Address)
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_collections_by_owner_handler)
}

pub async fn get_collections_by_owner_handler(owner: Address, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}