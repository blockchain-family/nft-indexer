select cron.schedule('refresh nft_collection_details', '*/120 * * * *',
                     'refresh materialized view concurrently nft_collection_details;');