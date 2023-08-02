use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer};
use data_reader::{MetaReaderContext, MetadataJrpcService};
use indexer_repo::meta::MetadataModelService;
use std::net::SocketAddr;

use crate::api;

pub async fn run_api(address: &SocketAddr, context: MetaReaderContext) -> std::io::Result<()> {
    let meta_jrpc_service = MetadataJrpcService::new(context.jrpc_client);
    let meta_model_service = MetadataModelService::new(context.pool);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(api::metadata::refresh_metadata_by_nft)
            .service(health)
            .app_data(Data::new(meta_jrpc_service.clone()))
            .app_data(Data::new(meta_model_service.clone()))
    })
    .bind(address)?
    .run()
    .await
}

#[get("/healthz")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
