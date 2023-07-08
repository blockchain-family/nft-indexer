use sqlx::PgPool;

pub async fn run_updater(pool: PgPool) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        if let Err(e) = indexer_repo::actions::update_offers_status(&pool).await {
            log::error!("Error updating offers: {:#?}", e);
        }
    }
}
