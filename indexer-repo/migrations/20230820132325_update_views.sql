alter table nft_direct_buy add column root t_address;
alter table nft_direct_sell add column root t_address;
alter table nft_auction add column root t_address;

alter table nft_price_history alter column ts drop not null;

alter table roots drop constraint roots_events_whitelist_address_fk;
alter table roots rename column event_whitelist_address to address;

drop view nft_auction_active_bids cascade;
drop view nft_auction_bids_view cascade;
drop view nft_collection_details cascade;
drop view nft_current_price_usd cascade;
drop view nft_deal_history_usd cascade;
drop view nft_details cascade;
drop view nft_direct_buy_best_offer cascade;
drop view nft_direct_buy_usd cascade;
drop view nft_direct_sell_usd cascade;
drop view nft_price_history_usd cascade;

alter table nft_collection drop column owners_count;

create or replace view nft_direct_buy_usd as
select
    s.address,
    s.root,
    s.nft,
    n.collection,
    s.price_token,
    s.price,
    s.buy_price_usd,
    s.buyer,
    s.finished_at,
    s.expired_at,
    case
        when s.state = 'active'::direct_buy_state and
             to_timestamp(0) < s.finished_at and s.finished_at < now()::timestamp
        then
            'expired'::direct_buy_state
        else
            s.state
    end as state,
    s.created,
    s.updated,
    s.tx_lt,
    s.price * p.usd_price as usd_price,
    ev.fee_numerator,
    ev.fee_denominator
from nft_direct_buy s
         join roots r on r.address::text = s.root::text
         join nft n on n.address::text = s.nft::text and not n.burned
         left join token_usd_prices p on s.price_token::text = p.token::text
         left join lateral (
    select (ne.args -> 'fee' -> 'numerator')::int as fee_numerator,
           (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
    from nft_events ne
    where ne.event_type = 'market_fee_changed' and
                ne.args ->> 'auction' = s.address
    ) as ev on true;

create or replace view nft_direct_buy_best_offer as
select distinct on (s.nft)
    s.address,
    s.root,
    s.nft,
    s.collection,
    s.price_token,
    s.price,
    s.buy_price_usd,
    s.buyer,
    s.finished_at,
    s.expired_at,
    -- NOTE: state check in nft_direct_buy_usd
    s.state,
    s.created,
    s.updated,
    s.tx_lt,
    s.usd_price
from nft_direct_buy_usd s
join roots r on r.address::text = s.root::text
join token_usd_prices p on s.price_token::text = p.token::text
where s.state = 'active'::direct_buy_state and
      s.usd_price is not null
order by s.nft, s.usd_price desc;

create or replace view nft_details as
select
    n.address,
    n.collection,
    n.owner,
    n.manager,
    n.name::text                         as name,
    n.description,
    n.burned,
    n.updated,
    n.owner_update_lt                    as tx_lt,
    m.meta,
    auc.auction,
    auc."auction_status: _",
    sale.forsale,
    sale."forsale_status: _",
    (select s.address as best_offer
     from nft_direct_buy_best_offer s
     where s.nft::text = n.address::text
     limit 1)                            as best_offer,
    least(auc.price_usd, sale.price_usd) as floor_price_usd,
    last_deal.last_price                 as deal_price_usd,
    case
        when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.min_bid
        when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.price
        else null::numeric
    end as floor_price,
    case
        when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.token::character varying
        when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.token::character varying
        else null::character varying
    end as floor_price_token,
    nft_id.id as nft_id
from nft n
    left join lateral (
        select max(ne.args ->> 'id')::text as id from nft_events ne
        where ne.event_type = 'nft_created'
            and ne.nft = n.address
        group by ne.nft
    ) nft_id on true
    left join lateral (
        select ag.price as last_price
        from (
            select s.price * tup.usd_price as price, s.created
            from nft_direct_sell s
            join token_usd_prices tup on tup.token::text = s.price_token::text
            where s.state = 'filled'::direct_sell_state and
                  s.nft::text = n.address::text
            union all
            select s.price * tup.usd_price, s.created
            from nft_direct_buy s
            join token_usd_prices tup on tup.token::text = s.price_token::text
            where s.state = 'filled'::direct_buy_state and
                  s.nft::text = n.address::text
            union all
            select s.max_bid * tup.usd_price, s.created_at
            from nft_auction s
            join token_usd_prices tup on tup.token::text = s.price_token::text
            where s.status = 'completed'::auction_status and
                  s.nft::text = n.address::text
        ) ag
        order by ag.created desc
        limit 1
    ) last_deal on true
    left join lateral (
        select a.address                 as auction,
               a.status                  as "auction_status: _",
               a.min_bid * tup.usd_price as price_usd,
               tup.token,
               a.min_bid
        from nft_auction a
        join roots r
            on r.address::text = a.root::text
        left join token_usd_prices tup
            on tup.token::text = a.price_token::text and
               (a.status = 'active'::auction_status and
                (a.finished_at = to_timestamp(0) or
                 a.finished_at > now()::timestamp))
        where a.nft::text = n.address::text and
              a.status = 'active'::auction_status
        limit 1
    ) auc on true
    left join nft_metadata m on m.nft::text = n.address::text
    left join lateral (
        select s.address               as forsale,
               s.state                 as "forsale_status: _",
               s.price * tup.usd_price as price_usd,
               s.price,
               tup.token
        from nft_direct_sell s
        join roots r on r.address::text = s.root::text
        left join token_usd_prices tup
        on tup.token::text = s.price_token::text and
           (s.state = 'active'::direct_sell_state and
            (s.expired_at = to_timestamp(0) or
             s.expired_at > now()))
        where s.nft::text = n.address::text and
              s.state = 'active'::direct_sell_state
        limit 1
    ) sale on true
where not n.burned;

create or replace view nft_direct_sell_usd as
select
    s.address,
    s.nft,
    n.collection,
    s.price_token,
    s.price,
    s.sell_price_usd,
    s.seller,
    s.finished_at,
    s.expired_at,
    case
        when s.state = 'active'::direct_sell_state and
             to_timestamp(0) < s.finished_at and s.finished_at < now()::timestamp
            then
            'expired'::direct_sell_state
        else
            s.state
    end as state,
    s.created,
    s.updated,
    s.tx_lt,
    s.price * p.usd_price as usd_price,
    ev.fee_numerator,
    ev.fee_denominator
from nft_direct_sell s
join roots r on r.address::text = s.root::text
join nft n
    on n.address::text = s.nft::text and
       not n.burned
left join token_usd_prices p on s.price_token::text = p.token::text
left join lateral (
    select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
           (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
    from nft_events ne
    where ne.event_type = 'market_fee_changed' and
          ne.args ->> 'auction' = s.address
) as ev on true;

create or replace view nft_auction_bids_view as
select
    b.auction,
    b.buyer,
    b.price,
    b.created_at,
    b.next_bid_value,
    b.tx_lt,
    (max(l.created_at) = b.created_at) as active,
    (b.price * p.usd_price) as usd_price,
    (b.next_bid_value * p.usd_price) as next_bid_usd_value,
    a.nft,
    n.collection,
    a.price_token,
    n.owner
from nft_auction_bid b
join nft_auction_bid l
    on (l.auction::text = b.auction::text) and
       (l.declined is null or l.declined is false)
join nft_auction a
    on a.address::text = b.auction::text and
    a.status <> 'completed'::auction_status
join roots r on r.address::text = a.root::text
join nft n
    on n.address::text = a.nft::text and
       not n.burned
left join token_usd_prices p on p.token::text = a.price_token::text
where b.declined is null or
      b.declined is false
group by b.auction, b.buyer, b.price, b.created_at, b.next_bid_value, b.tx_lt,
         p.usd_price,
         a.nft, a.price_token,
         n.collection, n.owner;

create or replace view nft_auction_active_bids as
select distinct on (b.auction)
    b.auction,
    b.buyer,
    b.price,
    b.next_bid_value,
    b.declined,
    b.created_at,
    b.tx_lt
from nft_auction_bid b
where b.declined is null or
      b.declined is false
order by b.auction, b.created_at desc;

create or replace view nft_auction_search as
select
    a.address,
    a.nft,
    a.wallet_for_bids,
    a.price_token,
    a.start_price,
    a.max_bid,
    a.min_bid,
    case
        when a.status = 'active'::auction_status and
             to_timestamp(0) < a.finished_at and a.finished_at < now()::timestamp
            then
            'expired'::auction_status
        else
            a.status
    end as "status: _",
    a.created_at,
    a.finished_at,
    a.tx_lt,
    v.buyer                     as last_bid_from,
    count(b.*)                  as bids_count,
    max(b.price)                as last_bid_value,
    max(b.price) * p.usd_price  as last_bid_usd_value,
    max(b.created_at)           as last_bid_ts,
    a.start_price * p.usd_price as start_usd_price,
    a.max_bid * p.usd_price     as max_usd_bid,
    a.min_bid * p.usd_price     as min_usd_bid,
    ev.fee_numerator,
    ev.fee_denominator
from nft_auction a
join roots r on r.address::text = a.root::text
join nft n on n.address::text = a.nft::text and not n.burned
left join nft_auction_bid b
    on b.auction::text = a.address::text and
       (b.declined is null or b.declined is false)
left join nft_auction_bids_view v
    on v.auction::text = a.address::text and
       v.active is true
left join token_usd_prices p on p.token::text = a.price_token::text
left join lateral (
    select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
           (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
    from nft_events ne
    where ne.event_type = 'market_fee_changed'
      and ne.args ->> 'auction' = a.address
) as ev on true
group by a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price, ev.fee_numerator,  ev.fee_denominator;

create or replace view nft_collection_details as
select
    c.address,
    c.owner,
    c.name,
    c.description,
    c.created,
    c.updated,
    c.wallpaper,
    c.logo,
    c.total_price,
    c.max_price,
    nft_counter.owners_count,
    c.verified,
    nft_counter.cnt as nft_count,
    least(direct_sell.usd, auction.usd) as floor_price_usd,
    coalesce(total_volume.usd, (0)::numeric) as total_volume_usd,
    attr.list as attributes,
    c.first_mint
from nft_collection c
left join lateral (
    select json_agg(res.json) as list
    from (
        select json_build_object('traitType', na.trait_type,
                                 'traitValues', json_agg(distinct trim(both from (na.value #>> '{}'::text[])))) as json
        from nft_attributes na
        where na.collection::text = c.address::text
        group by na.trait_type, na.collection
    ) res
) attr on true
left join lateral (
    select min(na.min_bid * tup.usd_price) as usd
    from nft_auction na
    join roots r on r.address::text = na.root::text
    join nft n
        on n.address::text = na.nft::text and
           n.collection::text = c.address::text and
           not n.burned
    left join token_usd_prices tup on tup.token::text = na.price_token::text
    where na.status = 'active'::auction_status and
          (na.finished_at = to_timestamp(0) or
           na.finished_at > now()::timestamp)
) auction on true
left join lateral (
    select min(ds.price * tup.usd_price) as usd
    from nft_direct_sell ds
    join roots r on r.address::text = ds.root::text
    join nft n
        on n.address::text = ds.nft::text and
           n.collection::text = c.address::text and
           not n.burned
    left join token_usd_prices tup on tup.token::text = ds.price_token::text
    where ds.state = 'active'::direct_sell_state and
          (ds.expired_at = to_timestamp(0) or
           ds.expired_at > now()::timestamp)
) direct_sell on true
left join lateral (
    select
        count(1) as cnt,
        count(distinct owner) as owners_count
    from nft n
    where n.burned is false and
          n.collection::text = c.address::text
) nft_counter on true
left join lateral (
    select sum(ag.price_usd) as usd
    from (
        select coalesce(tup.usd_price * ndb.price, (0)::numeric) as price_usd
        from nft_direct_buy ndb
        join roots r on r.address::text = ndb.root::text
        join nft n
            on n.address::text = ndb.nft::text and
               n.collection::text = c.address::text and
               not n.burned
        join token_usd_prices tup on tup.token::text = ndb.price_token::text
        where ndb.state = 'filled'::direct_buy_state
        union all
        select coalesce(tup.usd_price * nds.price, (0)::numeric) as "coalesce"
        from nft_direct_sell nds
        join roots r on r.address::text = nds.root::text
        join nft n
            on n.address::text = nds.nft::text and
               n.collection::text = c.address::text and
               not n.burned
        join token_usd_prices tup on tup.token::text = nds.price_token::text
        where nds.state = 'filled'::direct_sell_state
        union all
        select coalesce(tup.usd_price * na.max_bid, (0)::numeric) as "coalesce"
        from nft_auction na
        join roots r on r.address::text = na.root::text
        join nft n
            on na.nft::text = n.address::text and
               not n.burned and
               n.collection::text = c.address::text
        join token_usd_prices tup on tup.token::text = na.price_token::text
        where na.status = 'completed'::auction_status
    ) ag
) total_volume on true;

create or replace view nft_current_price_usd as
select
    h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    h.collection,
    h.price * u.usd_price as usd_price
from nft_current_price h
join token_usd_prices u on u.token::text = h.price_token::text;

create or replace view nft_deal_history_usd as
select
    h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    h.collection,
    h.buyer,
    h.seller,
    h.price * u.usd_price as usd_price
from nft_deal_history h
join token_usd_prices u on u.token::text = h.price_token::text;

create or replace view nft_price_history_usd as
select
    h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    n.collection,
    h.price * u.usd_price as usd_price
from nft_price_history h
join nft n
    on n.address::text = h.nft::text and
       not n.burned
join token_usd_prices u on u.token::text = h.price_token::text
union
select
    h.source,
    h.source_type,
    h.ts,
    h.price,
    na.price_token,
    na.nft,
    n.collection,
    h.price * u.usd_price as usd_price
from nft_price_history h
join nft_auction na on h.source = na.address and
    h.nft is null and h.price_token is null
join nft n
    on n.address::text = na.nft::text and
       not n.burned
join token_usd_prices u on u.token::text = na.price_token::text;

alter table nft_direct_buy drop column collection;
alter table nft_direct_sell drop column collection;
alter table nft_price_history drop column collection;

drop table events_whitelist;


