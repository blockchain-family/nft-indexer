use crate::api;
use crate::indexer::metadata_service::MetadataService;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use transaction_consumer::TransactionConsumer;

pub async fn run_api(
    address: &SocketAddr,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let metadata_service = Arc::new(MetadataService {
            consumer: consumer.clone(),
            pool: pool.clone(),
        });
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(api::metadata::refresh_metadata_by_nft)
            .service(health)
            .app_data(Data::new(metadata_service))
    })
    .bind(address)?
    .run()
    .await
}

#[get("/healthz")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
