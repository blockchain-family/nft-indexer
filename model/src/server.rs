use std::net::SocketAddr;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use transaction_consumer::TransactionConsumer;
use crate::api;
use crate::indexer::metadata_service::MetadataService;

pub async fn run_api(
    address: &SocketAddr,
    pool: PgPool,
    consumer: Arc<TransactionConsumer>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let metadata_service = Arc::new(MetadataService{ consumer: consumer.clone(), pool: pool.clone() });
        let cors = Cors::permissive();
        App::new()
            // .service(health_checker_handler)
            .wrap(Logger::default())
            .wrap(cors)
            .service(api::metadata::refresh_metadata_by_nft)
            .app_data(metadata_service)
    })
    .bind(address)?
    .run()
    .await
}