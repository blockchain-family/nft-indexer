select cron.schedule('refresh nft_verified_mv', '*/8 * * * *',
                     'refresh materialized view concurrently nft_verified_mv;');

create index nft_verified_mv_updated_idx
	on nft_verified_mv (updated);
