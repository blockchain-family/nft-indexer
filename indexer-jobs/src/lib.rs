use sqlx::PgPool;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn schedule_database_jobs(pool: PgPool) {
    let scheduler = JobScheduler::new()
        .await
        .expect("failed to create db job scheduler");

    schedule_safe_full_nft_verified_extended_refresh(&pool, &scheduler).await;

    schedule_safe_partial_nft_verified_extended_refresh(&pool, &scheduler).await;

    schedule_nft_events_verified_materialized_view_refresh(&pool, &scheduler).await;

    schedule_nft_types_materialized_view_refresh(&pool, &scheduler).await;

    schedule_collection_types_materialized_view_refresh(&pool, &scheduler).await;

    schedule_latest_collections_update(&pool, &scheduler).await;

    schedule_all_collections_update(pool, &scheduler).await;

    scheduler.start().await.expect("failed to schedule db jobs");

    std::future::pending().await
}

async fn schedule_safe_full_nft_verified_extended_refresh(pool: &PgPool, scheduler: &JobScheduler) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 8 */4 * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::safe_refresh_nft_verified_extended(&pool, true)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to fully refresh verified nfts (expended)");
                    } else {
                        log::info!("fully refreshed verified nfts (expended)");
                    };
                })
            })
            .expect("failed to schedule verified nfts (expended) full refresh")
        })
        .await
        .expect("failed to add verified nfts (expended) full refresh schedule");
}

async fn schedule_safe_partial_nft_verified_extended_refresh(
    pool: &PgPool,
    scheduler: &JobScheduler,
) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 */7 * * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::safe_refresh_nft_verified_extended(&pool, false)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to partially refresh verified nfts (expended)");
                    } else {
                        log::info!("partially refreshed verified nfts (expended)");
                    };
                })
            })
            .expect("failed to schedule verified nfts (expended) partial refresh")
        })
        .await
        .expect("failed to add verified nfts (expended) partial refresh schedule");
}

async fn schedule_nft_events_verified_materialized_view_refresh(
    pool: &PgPool,
    scheduler: &JobScheduler,
) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 */11 * * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::refresh_nft_events_verified_materialized_view(&pool)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to refresh nft events verified materialized view");
                    } else {
                        log::info!("successfully refreshed nft events verified materialized view");
                    };
                })
            })
            .expect("failed to schedule nft events verified materialized view refresh")
        })
        .await
        .expect("failed to add nft events verified materialized view refresh schedule");
}

async fn schedule_nft_types_materialized_view_refresh(pool: &PgPool, scheduler: &JobScheduler) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 */33 * * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::refresh_nft_types_materialized_view(&pool)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to refresh nft types materialized view");
                    } else {
                        log::info!("successfully refreshed nft types materialized view");
                    };
                })
            })
            .expect("failed to schedule nft types materialized view refresh")
        })
        .await
        .expect("failed to add nft types materialized view refresh schedule");
}

async fn schedule_collection_types_materialized_view_refresh(
    pool: &PgPool,
    scheduler: &JobScheduler,
) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 */29 * * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::refresh_collection_types_materialized_view(&pool)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to refresh collection types materialized view");
                    } else {
                        log::info!("successfully refreshed collection types materialized view");
                    };
                })
            })
            .expect("failed to schedule collection types materialized view refresh")
        })
        .await
        .expect("failed to add collection types materialized view refresh schedule");
}

async fn schedule_latest_collections_update(pool: &PgPool, scheduler: &JobScheduler) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 */25 * * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::update_latest_collections(&pool)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to update latest collections");
                    } else {
                        log::info!("successfully updated latest collections");
                    };
                })
            })
            .expect("failed to schedule latest collections update")
        })
        .await
        .expect("failed to add latest collections update schedule");
}

async fn schedule_all_collections_update(pool: PgPool, scheduler: &JobScheduler) {
    scheduler
        .add({
            let pool = pool.clone();
            Job::new_async("0 0 0 * * *", move |_uuid, _l| {
                let pool = pool.clone();
                Box::pin(async move {
                    if indexer_repo::jobs::update_all_collections(&pool)
                        .await
                        .is_err()
                    {
                        log::warn!("failed to update all collections");
                    } else {
                        log::info!("successfully updated all collections");
                    };
                })
            })
            .expect("failed to schedule all collections update")
        })
        .await
        .expect("failed to add all collections update schedule");
}
