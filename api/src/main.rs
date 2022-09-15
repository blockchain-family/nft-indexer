use api::cfg::ApiConfig;
use warp::Filter;
use storage::api_service::ApiService;
use std::sync::Arc;
use api::handlers::*;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();
    let cfg = ApiConfig::new().unwrap();

    let db_pool = cfg.database.init().await.expect("err init database");
    let service = ApiService::new(Arc::new(db_pool));

    let api = get_nft(service.clone())
        .or(get_nft_list(service.clone()))
        .or(get_nft_offers(service.clone()))
        .or(get_nft_price_history(service.clone()))
        .or(post_nft_reload_meta(service.clone()))
        .or(get_collection(service.clone()))
        .or(get_collections_by_owner(service.clone()))
        .or(get_owner_bids_out(service.clone()))
        .or(get_owner_bids_in(service.clone()))
        .or(get_owner_offers_out(service.clone()))
        .or(get_owner_offers_in(service.clone()))
        .or(get_auctions(service.clone()))
        .or(get_auction_bids(service.clone()))
        .or(get_events(service.clone()));


    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("api"));
    // Start up the server...
    warp::serve(routes).run(cfg.http_address).await;
}


