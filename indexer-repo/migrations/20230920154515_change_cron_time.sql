select cron.schedule('refresh nft_collection_details', '*/7 * * * *',
                     'refresh materialized view concurrently nft_collection_details;');

select cron.schedule('refresh nft_verified_mv', '*/17 * * * *',
                     'refresh materialized view concurrently nft_verified_mv;');