drop extension if exists pg_cron;
create extension pg_cron;
select cron.schedule('refresh nft_collection_details', '*/10 * * * *',
                     'refresh materialized view concurrently nft_collection_details;');