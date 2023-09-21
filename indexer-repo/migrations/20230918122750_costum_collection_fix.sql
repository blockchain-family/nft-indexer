drop materialized view nft_collection_details;

create materialized view nft_collection_details as
select coalesce(ncc.address, c.address)         as "address",
       c.owner,
       coalesce(ncc.name, c.name)               as "name",
       coalesce(ncc.description, c.description) as "description",
       c.created,
       coalesce(ncc.updated, c.updated)         as "updated",
       coalesce(ncc.wallpaper, c.wallpaper)     as "wallpaper",
       coalesce(ncc.logo, c.logo)               as "logo",
       nft_counter.owners_count,
       c.verified,
       nft_counter.cnt                          as nft_count,
       least(direct_sell.usd, auction.usd)      as floor_price_usd,
       coalesce(total_volume.usd, (0)::numeric) as total_volume_usd,
       attr.list                                as attributes,
       count(1) over ()                         as total_count,
       c.first_mint
from nft_collection c
         left join nft_collection_custom ncc on c.address = ncc.address
         left join lateral ( select json_agg(res.json) as list
                             from ( select json_build_object('traitType', na.trait_type, 'traitValues',
                                                             json_agg(distinct trim(both from (na.value #>> '{}'::text[])))) as json
                                    from nft_attributes na
                                    where na.collection = c.address
                                    group by na.trait_type, na.collection ) res ) attr on true
         left join lateral ( select min(na.min_bid * tup.usd_price) as usd
                             from nft_auction na
                                      join offers_whitelist ow on ow.address = na.address
                                      join nft n on n.address = na.nft and n.collection = c.address and not n.burned
                                      left join token_usd_prices tup on tup.token = na.price_token
                             where na.status = 'active'::auction_status
                               and (na.finished_at = to_timestamp(0) or na.finished_at > now()::timestamp) ) auction
                   on true
         left join lateral ( select min(ds.price * tup.usd_price) as usd
                             from nft_direct_sell ds
                                      join offers_whitelist ow on ow.address = ds.address
                                      join nft n on n.address = ds.nft and n.collection = c.address and not n.burned
                                      left join token_usd_prices tup on tup.token = ds.price_token
                             where ds.state = 'active'::direct_sell_state
                               and (ds.expired_at = to_timestamp(0) or ds.expired_at > now()::timestamp) ) direct_sell
                   on true
         left join lateral ( select count(1) as cnt, count(distinct owner) as owners_count
                             from nft n
                             where n.burned is false
                               and n.collection = c.address) nft_counter on true
         left join lateral ( select sum(coalesce(tup.usd_price * nph.price, 0)) as usd
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.collection = c.address) total_volume on true;