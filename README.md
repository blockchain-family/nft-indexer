<p align="center">
  <a href="https://github.com/venom-blockchain/developer-program">
    <img src="https://raw.githubusercontent.com/venom-blockchain/developer-program/main/vf-dev-program.png" alt="Logo" width="366.8" height="146.4">
  </a>
</p>

<hr>

Force refresh metadata


```
/* Run after migrations */

delete from cron.job where jobname = 'refresh nft_verified_mv'

select refresh_nft_verified_extended(true);
SELECT cron.schedule('execute safe_refresh_nft_verified_extended every 4 hours', '8 */4 * * *',
                     'select safe_refresh_nft_verified_extended(true);');

SELECT cron.schedule('execute safe_refresh_nft_verified_extended every 7 minutes', '*/7 * * * *',
                     'select safe_refresh_nft_verified_extended(false);');               
                     
SELECT cron.schedule('refresh materialized view concurrently nft_events_verified_mv', '*/11 * * * *',
                     'refresh materialized view concurrently nft_events_verified_mv;');                          
```