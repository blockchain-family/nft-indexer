use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use crate::model::{Collection, Contract};
use std::ops::Sub;


/// GET /collection/{address}/details
pub fn get_collection(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("collection" / Address / "details")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_collection_handler)
}

pub async fn get_collection_handler(address: Address, db: ApiService) -> Result<Box<dyn warp::Reply>, Infallible> {
    match db.get_collection((&address).into()).await {
        Err(e) => Ok(Box::from(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))),
        Ok(None) => Ok(Box::from(warp::reply::with_status(String::default(), StatusCode::BAD_REQUEST))),
        Ok(Some(col)) => {
            let ret = Collection {
                contract: Contract { 
                    address: Address::from(col.address),
                    name: Some(col.name),
                    description: Some(col.description),
                    owner: Some(Address::from(col.owner)),
                },
                verified: Some(col.verified),
                created_at: col.created.timestamp() as usize,
                logo: col.logo,
                wallpaper: col.wallpaper,
                owners_count: col.owners_count.unwrap_or_default() as usize,
                total_price: None,
                lowest_price: None,
            };
            Ok(Box::from(warp::reply::with_status(warp::reply::json(&ret), StatusCode::OK)))
        }
    }
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

pub async fn get_collections_by_owner_handler(owner: Address, db: ApiService) -> Result<Box<dyn warp::Reply>, Infallible> {
    match db.list_collections_by_owner((&owner).into()).await {
        Err(e) => Ok(Box::from(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))),
        Ok(mut list) => {
            let ret: Vec<Collection> = list.drain(..).map(|col| Collection {
                contract: Contract { 
                    address: Address::from(col.address),
                    name: Some(col.name),
                    description: Some(col.description),
                    owner: Some(Address::from(col.owner)),
                },
                verified: Some(col.verified),
                created_at: col.created.timestamp() as usize,
                logo: col.logo,
                wallpaper: col.wallpaper,
                owners_count: col.owners_count.unwrap_or_default() as usize,
                total_price: None,
                lowest_price: None,
            }).collect();
            Ok(Box::from(warp::reply::with_status(warp::reply::json(&ret), StatusCode::OK)))
        }
    }
}