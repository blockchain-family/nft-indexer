use sqlx::PgPool;

pub async fn run_updater(pool: PgPool) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Err(e) = storage::actions::update_offers_status(&pool).await {
            log::error!("Error updating offers: {:#?}", e);
        }
    }
}
