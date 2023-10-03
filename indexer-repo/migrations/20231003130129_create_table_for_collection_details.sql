drop materialized view nft_collection_details;

create table nft_collection_details as
select coalesce(ncc.address, c.address)                                                                       as address,
       c.owner,
       coalesce(ncc.name, c.name)                                                                             as name,
       coalesce(ncc.description, c.description)                                                               as description,
       c.created,
       coalesce(ncc.updated, c.updated)                                                                       as updated,
       coalesce(ncc.wallpaper, c.wallpaper)                                                                   as wallpaper,
       coalesce(ncc.logo, c.logo)                                                                             as logo,
       ncc.social,
       ( select count(*) as count
         from ( select distinct n.owner
                from nft n
                where n.collection = c.address
                  and not n.burned ) owners )                                                                 as owners_count,
       c.verified,
       ( select count(*) as count
         from nft n
         where n.collection = c.address
           and not n.burned )                                                                                 as nft_count,
       least(direct_sell.usd, auction.usd)                                                                    as floor_price_usd,
       coalesce(total_volume.usd, 0)                                                                          as total_volume_usd,
       attr.list                                                                                              as attributes,
       count(1) over ()                                                                                       as total_count,
       count(1) filter (where c.verified) over ()                                                             as verified_count,
       c.first_mint,
       least(direct_sell.token_price, auction.token_price)                                                    as floor_price_token
from nft_collection c
         left join nft_collection_custom ncc on c.address = ncc.address
         left join lateral ( select json_agg(res.json) as list
                             from ( select json_build_object('traitType', na.trait_type, 'traitValues',
                                                             json_agg(distinct trim(both from na.value #>> '{}'::text[]))) as json
                                    from nft_attributes na
                                    where na.collection = c.address
                                    group by na.trait_type, na.collection ) res) attr on true
         left join lateral ( select min(na.min_bid * tup.usd_price) as usd, min(na.min_bid) as token_price
                             from nft_auction na
                                      join offers_whitelist ow on ow.address = na.address
                                      left join token_usd_prices tup on tup.token = na.price_token
                             where na.collection = c.address
                               and na.status = 'active'::auction_status
                               and (na.finished_at = to_timestamp(0) or na.finished_at > now())) auction on true
         left join lateral ( select min(ds.price * tup.usd_price) as usd, min(ds.price) as token_price
                             from nft_direct_sell ds
                                      join offers_whitelist ow on ow.address = ds.address
                                      left join token_usd_prices tup on tup.token = ds.price_token
                             where ds.collection = c.address
                               and ds.state = 'active'::direct_sell_state
                               and (ds.expired_at = to_timestamp(0) or ds.expired_at > now())) direct_sell on true
         left join lateral ( select sum(coalesce(tup.usd_price * nph.price, 0)) as usd
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.collection = c.address) total_volume on true;

alter table nft_collection_details
    add constraint nft_collection_details_temp_fk foreign key (address) references nft_collection (address);

create index nft_collection_details_owner_idx on nft_collection_details (owner);
create index nft_collection_details_verified_idx on nft_collection_details (verified);
create index nft_collection_details_name_idx on nft_collection_details (name);
create index nft_collection_details_owners_count_idx on nft_collection_details (owners_count desc);


alter table nft_events
    add column local_created_at timestamp not null default now();

create procedure update_collections_details(collections t_address[], with_counts bool default false)
    language plpgsql as
$$
declare
    collection_for_update t_address;
begin
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
                                     least(direct_sell.token_price, auction.token_price)                   as floor_price_token
                              from nft_collection c
                                       left join nft_collection_custom ncc on c.address = ncc.address
                                       left join lateral ( select json_agg(res.json) as list
                                                           from ( select json_build_object('traitType', na.trait_type,
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
            update nft_collection_details nc
            set owner             = d.owner,
                name              = d.name,
                description       = d.description,
                updated           = d.updated,
                wallpaper         = d.wallpaper,
                logo              = d.logo,
                social            = d.social,
                owners_count      = coalesce(nc.owners_count, d.owners_count),
                verified          = d.verified,
                nft_count         = coalesce(nc.nft_count, d.nft_count),
                floor_price_usd   = d.floor_price_usd,
                total_volume_usd  = d.total_volume_usd,
                attributes        = d.attributes,
                floor_price_token = d.floor_price_token
            from details d
            where nc.address = collection_for_update;
        end loop;

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

-- ex.: `call update_latest_collections(interval '1 hour');`

create procedure update_latest_collections("interval" interval)
    language plpgsql as
$$
declare
    collections_for_update t_address[];
begin
    collections_for_update := ( select array_agg(distinct ne.collection)
                                from nft_events ne
                                where ne.local_created_at > (extract(epoch from (now() - "interval")))::bigint
                                  and ne.collection is not null
                                  and ne.computed_event_kind in ('mint'::event_kind, 'transfer'::event_kind) );

    call update_collections_details(collections_for_update, true);
end;
$$;

create procedure update_all_collections()
    language plpgsql as
$$
declare
    collections_for_update t_address[];
begin
    collections_for_update := ( select array_agg(distinct c.address) from nft_collection c );
    call update_collections_details(collections_for_update, true);
end;
$$;