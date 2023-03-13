use std::sync::Arc;
use actix_web::{HttpResponse, post, web};
use actix_web::web::Json;
use serde::Deserialize;
use crate::indexer::metadata_service::MetadataService;

#[derive(Deserialize)]
pub struct RefreshMetadataParams {
    nft: Option<String>,
    collection: String
}

#[post("/metadata/refresh/")]
pub async fn refresh_metadata_by_nft(
    path: Json<RefreshMetadataParams>,
    metadata_service: web::Data<Arc<MetadataService>>,
) -> HttpResponse {
    match path.0.nft {
        None => HttpResponse::BadRequest().finish(),
        Some(nft) => {
            let res = metadata_service.refresh_metadata(nft.as_str(), path.0.collection.as_str()).await;
            match res {
                Ok(_) => {
                     HttpResponse::Ok().finish()
                }
                Err(err) => {
                    log::error!("calc metadata error {err}");
                     HttpResponse::InternalServerError().finish()
                }
            }

        }
    }

}