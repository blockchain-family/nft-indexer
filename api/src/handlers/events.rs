use std::convert::Infallible;
use warp::http::StatusCode;
use storage::api_service::ApiService;
use warp::Filter;
use crate::model::Event;
use storage::tables::EventType;
use serde::{Serialize, Deserialize};
use std::ops::Sub;


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

pub async fn get_events_handler(query: EventsQuery, db: ApiService) -> Result<Box<dyn warp::Reply>, Infallible> {
    match db.list_events(query.nft, query.collection, query.typ, query.page_size.unwrap_or(100), query.offset.unwrap_or_default()).await {
        Err(e) => Ok(Box::from(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))),
        Ok(mut list) => {
            let ret: Vec<Event> = list.drain(..).map(|ev| Event {
                id: ev.id,
                address: ev.address,
                typ: ev.event_type,
                cat: ev.event_cat,
                args: ev.args,
                ts: ev.created_at as usize,
            }).collect();
            Ok(Box::from(warp::reply::with_status(warp::reply::json(&ret), StatusCode::OK)))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventsQuery {
    pub owner: Option<String>,
    pub collection: Option<String>,
    pub nft: Option<String>,
    pub initiator: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<EventType>,
    #[serde(rename = "page-size")]
    pub page_size: Option<usize>,
    pub offset: Option<usize>,
}
