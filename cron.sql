-- after migration 20231004205345_nft_verified_extended_add_coalesce_index.sql
select cron.schedule('execute refresh_nft_verified_extended', '*/6 * * * *',
                     'select refresh_nft_verified_extended(false);');
