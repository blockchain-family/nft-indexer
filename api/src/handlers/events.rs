use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use storage::types::Address;
use storage::tables::EventType;
use serde::{Serialize, Deserialize};


/// GET /events
pub fn get_events(
    db: ApiService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("events")
        .and(warp::get())
        .and(warp::query::<EventsQuery>())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_events_handler)
}

pub async fn get_events_handler(query: EventsQuery, _db: ApiService) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventsQuery {
    pub owner: Option<Address>,
    pub collection: Option<Address>,
    pub nft: Option<Address>,
    pub initiator: Option<Address>,
    #[serde(rename = "type")]
    pub typ: Option<EventType>,
    #[serde(rename = "page-size")]
    pub page_size: Option<usize>,
    pub offset: Option<usize>,
}
