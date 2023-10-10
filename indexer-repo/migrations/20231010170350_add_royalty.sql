alter table nft_collection
    add royalty jsonb;
alter table nft_collection_details
    add royalty jsonb;

create unique index nft_collection_details_address_index on nft_collection_details (address);

create or replace procedure update_collections_details(collections t_address[], with_counts bool default false)
    language plpgsql as
$$
declare
    collection_for_update t_address;
begin
    if array_length(collections, 1) > 0 then
        foreach collection_for_update in array collections
            loop
                with details as ( select coalesce(ncc.address, c.address)                                      as address,
                                         c.owner,
                                         coalesce(ncc.name, c.name)                                            as name,
                                         coalesce(ncc.description, c.description)                              as description,
                                         greatest(ncc.updated, c.updated)                                      as updated,
                                         coalesce(ncc.wallpaper, c.wallpaper)                                  as wallpaper,
                                         coalesce(ncc.logo, c.logo)                                            as logo,
                                         ncc.social,
                                         case when with_counts then ( select count(*) as count
                                                                      from ( select distinct n.owner
                                                                             from nft n
                                                                             where n.collection = c.address
                                                                               and not n.burned ) owners ) end as owners_count,
                                         c.verified,
                                         case when with_counts then ( select count(*) as count
                                                                      from nft n
                                                                      where n.collection = c.address
                                                                        and not n.burned ) end                 as nft_count,
                                         least(direct_sell.usd, auction.usd)                                   as floor_price_usd,
                                         coalesce(total_volume.usd, 0)                                         as total_volume_usd,
                                         attr.list                                                             as attributes,
                                         least(direct_sell.token_price, auction.token_price)                   as floor_price_token,
                                         c.first_mint,
                                         c.created
                                  from nft_collection c
                                           left join nft_collection_custom ncc on c.address = ncc.address
                                           left join lateral ( select json_agg(res.json) as list
                                                               from ( select json_build_object('traitType',
                                                                                               na.trait_type,
                                                                                               'traitValues',
                                                                                               json_agg(distinct trim(both from na.value #>> '{}'::text[]))) as json
                                                                      from nft_attributes na
                                                                      where na.collection = c.address
                                                                      group by na.trait_type, na.collection ) res) attr
                                                     on true
                                           left join lateral ( select min(na.min_bid * tup.usd_price) as usd,
                                                                      min(na.min_bid)                 as token_price
                                                               from nft_auction na
                                                                        join offers_whitelist ow on ow.address = na.address
                                                                        left join token_usd_prices tup on tup.token = na.price_token
                                                               where na.collection = c.address
                                                                 and na.status = 'active'::auction_status
                                                                 and (na.finished_at = to_timestamp(0) or na.finished_at > now())) auction
                                                     on true
                                           left join lateral ( select min(ds.price * tup.usd_price) as usd,
                                                                      min(ds.price)                 as token_price
                                                               from nft_direct_sell ds
                                                                        join offers_whitelist ow on ow.address = ds.address
                                                                        left join token_usd_prices tup on tup.token = ds.price_token
                                                               where ds.collection = c.address
                                                                 and ds.state = 'active'::direct_sell_state
                                                                 and (ds.expired_at = to_timestamp(0) or ds.expired_at > now())) direct_sell
                                                     on true
                                           left join lateral ( select sum(coalesce(tup.usd_price * nph.price, 0)) as usd
                                                               from nft_price_history nph
                                                                        join offers_whitelist ow on ow.address = nph.source
                                                                        left join token_usd_prices tup on tup.token = nph.price_token
                                                               where nph.collection = c.address) total_volume on true
                                  where c.address = collection_for_update )
                insert
                into nft_collection_details (address, owner, name, description, created, updated, wallpaper, logo,
                                             social,
                                             owners_count,
                                             verified, nft_count, floor_price_usd, total_volume_usd, attributes,
                                             total_count,
                                             verified_count, first_mint, floor_price_token)
                select d.address,
                       d.owner,
                       d.name,
                       d.description,
                       d.created,
                       d.updated,
                       d.wallpaper,
                       d.logo,
                       d.social,
                       d.owners_count,
                       d.verified,
                       d.nft_count,
                       d.floor_price_usd,
                       d.total_volume_usd,
                       d.attributes,
                       0,
                       0,
                       d.first_mint,
                       d.floor_price_token
                from details d
                on conflict (address) do update set owner             = excluded.owner,
                                                    name              = excluded.name,
                                                    description       = excluded.description,
                                                    updated           = excluded.updated,
                                                    wallpaper         = excluded.wallpaper,
                                                    logo              = excluded.logo,
                                                    social            = excluded.social,
                                                    owners_count      = coalesce(nft_collection_details.owners_count, excluded.owners_count),
                                                    verified          = excluded.verified,
                                                    nft_count         = coalesce(nft_collection_details.nft_count, excluded.nft_count),
                                                    floor_price_usd   = excluded.floor_price_usd,
                                                    total_volume_usd  = excluded.total_volume_usd,
                                                    attributes        = excluded.attributes,
                                                    floor_price_token = excluded.floor_price_token
                where nft_collection_details.address = collection_for_update;
            end loop;
    end if;

    if with_counts then
        with counts as ( select count(1) over ()                           as total_count,
                                count(1) filter (where c.verified) over () as verified_count
                         from nft_collection c )
        update nft_collection_details c
        set total_count    = counts.total_count,
            verified_count = counts.verified_count
        from counts
        where true;
    end if;
end;
$$;
