create or replace view nft_collection_details
            (address, owner, name, description, created, updated, wallpaper, logo, total_price, max_price, owners_count,
             verified, nft_count, floor_price_usd, total_volume_usd, attributes)
as
       SELECT c.address,
       c.owner,
       c.name,
       c.description,
       c.created,
       c.updated,
       c.wallpaper,
       c.logo,
       c.total_price,
       c.max_price,
       c.owners_count,
       c.verified,
       nft_counter.cnt                        as nft_count,
       coalesce(direct_sell.usd, auction.usd) as floor_price_usd,
       coalesce(total_volume.usd, 0)          as total_volume_usd,
       attr.list                              as attributes

FROM nft_collection c
         left join lateral (
    select json_agg(res.json) as list
    from (
             select json_build_object(
                            'traitType', na.trait_Type,
                            'traitValues',
                            json_agg(distinct
                                     trim(na.value #>> '{}'))) as json

             from nft_attributes na
             where na.collection = c.address
             group by na.trait_type, na.collection) res

    ) attr on true
         left join lateral (select min(na.min_bid * tup.usd_price) as usd
                            from nft_auction na
                                     join nft n
                                          on n.address = na.address
                                              and n.collection = c.address
                                     left join token_usd_prices tup
                                               on tup.token = na.price_token
                            where na.status = 'active'
    ) as auction on true
         left join lateral (select min(ds.price * tup.usd_price) as usd
                            from nft_direct_sell ds
                                     left join token_usd_prices tup
                                               on tup.token = ds.price_token
                            where ds.state = 'active'
                              and ds.collection = c.address
    ) as direct_sell on true
         left join lateral (
    select count(1) cnt
    from nft n
    where n.burned = false
      and n.collection = c.address ) as nft_counter on true
         left join lateral (
    select sum(ag.price_usd) usd
    from (
             select coalesce(tup.usd_price * ndb.price, 0) as price_usd
             from nft_direct_buy ndb
                      join token_usd_prices tup
                           on tup.token = ndb.price_token
                      join nft n on ndb.nft = n.address and n.collection = c.address
             where ndb.state = 'filled'


             union all

             select coalesce(tup.usd_price * nds.price, 0)
             from nft_direct_sell nds
                      join token_usd_prices tup on tup.token = nds.price_token
             where nds.state = 'filled'
               and nds.collection = c.address
             union all
             select coalesce(tup.usd_price * na.max_bid, 0)
             from nft_auction na
                      join nft n on na.nft = n.address
                 and n.collection = c.address
                      join token_usd_prices tup
                           on tup.token = na.price_token
             where na.status = 'completed'
         ) as ag
    ) as total_volume on true
