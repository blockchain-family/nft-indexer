use crate::indexer::metadata_service::MetadataService;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RefreshMetadataParams {
    nft: Option<String>,
    collection: String,
}

#[post("/metadata/refresh/")]
pub async fn refresh_metadata_by_nft(
    path: Json<RefreshMetadataParams>,
    metadata_service: web::Data<Arc<MetadataService>>,
) -> HttpResponse {
    let result = match path.0.nft {
        None => {
            metadata_service
                .refresh_metadata_by_collection(path.0.collection.as_str())
                .await
        }
        Some(nft) => {
            metadata_service
                .refresh_metadata(nft.as_str(), path.0.collection.as_str())
                .await
        }
    };
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log::error!("calc metadata error {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
