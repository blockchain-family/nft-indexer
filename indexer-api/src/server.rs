use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use data_reader::MetaUpdater;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::api;

#[derive(Clone)]
pub struct AppState {
    pub meta_updater: MetaUpdater,
    pub address: String,
}

pub async fn run_api(address: &SocketAddr, meta_updater: MetaUpdater) -> std::io::Result<()> {
    let state = AppState {
        meta_updater,
        address: address.to_string(),
    };

    // TODO: split /healthz and /metadata/refresh/"

    let app = Router::new()
        .route("/healthz", get(health))
        .route(
            "/metadata/refresh/",
            post(api::metadata::refresh_metadata_by_nft),
        )
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await
}

async fn health() -> StatusCode {
    StatusCode::OK
}
