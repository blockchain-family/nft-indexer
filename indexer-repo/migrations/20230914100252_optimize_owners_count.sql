create or replace view nft_collection_details as
select c.address,
       c.owner,
       c.name,
       c.description,
       c.created,
       c.updated,
       c.wallpaper,
       c.logo,
       ( select count(*)
         from ( select distinct owner
                from nft n
                where n.collection = c.address and not n.burned ) owners )            as owners_count,
       c.verified,
       ( select count(*) from nft n where n.collection = c.address and not n.burned ) as nft_count,
       least(direct_sell.usd, auction.usd)                                            as floor_price_usd,
       coalesce(total_volume.usd, (0)::numeric)                                       as total_volume_usd,
       attr.list                                                                      as attributes,
       c.first_mint
from nft_collection c
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
         left join lateral ( select sum(coalesce(tup.usd_price * nph.price, 0)) as usd
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.collection = c.address) total_volume on true;