alter table nft_price_history
    add column collection t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_price_history
    alter column collection drop default;

alter table nft_auction
    add column nft_owner t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction
    alter column nft_owner drop default;

create or replace view nft_direct_sell_usd as
select s.address,
       s.nft,
       s.collection,
       s.price_token,
       s.price,
       s.seller,
       s.finished_at,
       s.expired_at,
       case when s.state = 'active'::direct_sell_state and to_timestamp(0) < s.finished_at and s.finished_at < now()::timestamp
                then 'expired'::direct_sell_state
            else s.state end as state,
       s.created,
       s.updated,
       s.tx_lt,
       s.price * p.usd_price as usd_price,
       ev.fee_numerator,
       ev.fee_denominator
from nft_direct_sell s
         join offers_whitelist ow on ow.address = s.address
         left join token_usd_prices p on s.price_token = p.token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_direct_buy_usd as
select s.address,
       s.root,
       s.nft,
       s.collection,
       s.price_token,
       s.price,
       s.buyer,
       s.finished_at,
       s.expired_at,
       case when s.state = 'active'::direct_buy_state and to_timestamp(0) < s.finished_at and s.finished_at < now()::timestamp
                then 'expired'::direct_buy_state
            else s.state end as state,
       s.created,
       s.updated,
       s.tx_lt,
       s.price * p.usd_price as usd_price,
       ev.fee_numerator,
       ev.fee_denominator
from nft_direct_buy s
         join offers_whitelist ow on ow.address = s.address
         left join token_usd_prices p on s.price_token = p.token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_collection_details as
select c.address,
       c.owner,
       c.name,
       c.description,
       c.created,
       c.updated,
       c.wallpaper,
       c.logo,
       nft_counter.owners_count,
       c.verified,
       nft_counter.cnt                          as nft_count,
       least(direct_sell.usd, auction.usd)      as floor_price_usd,
       coalesce(total_volume.usd, (0)::numeric) as total_volume_usd,
       attr.list                                as attributes,
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
         left join lateral ( select count(1) as cnt, count(distinct owner) as owners_count
                             from nft n
                             where n.burned is false
                               and n.collection = c.address ) nft_counter on true
         left join lateral ( select sum(coalesce(tup.usd_price * nph.price, 0)) as usd
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.collection = c.address) total_volume on true;

create or replace view nft_details as
select n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name::text                          as name,
       n.description,
       n.burned,
       n.updated,
       n.owner_update_lt                     as tx_lt,
       m.meta,
       auc.auction,
       auc."auction_status: _",
       sale.forsale,
       sale."forsale_status: _",
       ( select distinct on (s.address) first_value(s.address) over w
         from nft_direct_buy_usd s
         where state = 'active'
           and nft = n.address
         window w as (partition by nft order by usd_price desc)
         limit 1 )                           as best_offer,
       least(auc.price_usd, sale.price_usd)  as floor_price_usd,
       last_deal.last_price                  as deal_price_usd,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.min_bid
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.price
            else null::numeric end           as floor_price,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.token::character varying
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.token::character varying
            else null::character varying end as floor_price_token,
       n.id::text                            as nft_id
from nft n
         left join lateral ( select nph.price * tup.usd_price as last_price
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.nft = n.address
                             order by nph.ts desc
                             limit 1 ) last_deal on true
         left join lateral ( select a.address                 as auction,
                                    a.status                  as "auction_status: _",
                                    a.min_bid * tup.usd_price as price_usd,
                                    tup.token,
                                    a.min_bid
                             from nft_auction a
                                      join offers_whitelist ow on ow.address = a.address
                                      left join token_usd_prices tup on tup.token = a.price_token and
                                                                        (a.status = 'active'::auction_status and
                                                                         (a.finished_at = to_timestamp(0) or a.finished_at > now()::timestamp))
                             where a.nft = n.address
                               and a.status = 'active'::auction_status
                               and (a.finished_at = to_timestamp(0) or a.finished_at > now()::timestamp)
                             limit 1 ) auc on true
         left join nft_metadata m on m.nft = n.address
         left join lateral ( select s.address               as forsale,
                                    s.state                 as "forsale_status: _",
                                    s.price * tup.usd_price as price_usd,
                                    s.price,
                                    tup.token
                             from nft_direct_sell s
                                      join offers_whitelist ow on ow.address = s.address
                                      left join token_usd_prices tup on tup.token = s.price_token and
                                                                        s.state = 'active'::direct_sell_state and
                                                                        (s.expired_at = to_timestamp(0) or s.expired_at > now())
                             where s.nft = n.address
                               and s.state = 'active'::direct_sell_state
                               and (s.expired_at = to_timestamp(0) or s.expired_at > now())
                             limit 1 ) sale on true
where not n.burned;

drop view nft_auction_search;

create or replace view nft_auction_search as
select distinct on (a.address) a.address,
                               a.nft,
                               a.collection,
                               a.nft_owner,
                               a.wallet_for_bids,
                               a.price_token,
                               a.start_price,
                               a.max_bid,
                               a.min_bid,
                               case when a.status = 'active'::auction_status and to_timestamp(0) < a.finished_at and
                                         a.finished_at < now()::timestamp then 'expired'::auction_status
                                    else a.status end                                                           as "status: _",
                               a.created_at,
                               a.finished_at,
                               a.tx_lt,
                               sum(case when b.auction is null then 0 else 1 end)
                               over (partition by a.address)                                                    as bids_count,
                               first_value(b.buyer) over bids_w                                                 as last_bid_from,
                               first_value(b.price) over bids_w                                                 as last_bid_value,
                               first_value(b.price * p.usd_price) over bids_w                                   as last_bid_usd_value,
                               first_value(b.created_at) over bids_w                                            as last_bid_ts,
                               a.start_price * p.usd_price                                                      as start_usd_price,
                               a.max_bid * p.usd_price                                                          as max_usd_bid,
                               a.min_bid * p.usd_price                                                          as min_usd_bid,
                               ev.fee_numerator,
                               ev.fee_denominator
from nft_auction a
         join offers_whitelist ow on ow.address = a.address
         left join nft_auction_bid b on b.auction = a.address and b.declined is false
         left join token_usd_prices p on p.token = a.price_token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = a.address ) as ev on true
where b.declined is false
   or b.declined is null
window bids_w as (partition by b.auction order by b.created_at desc);

drop view if exists nft_direct_buy_best_offer;
drop view if exists nft_auction_active_bids;
