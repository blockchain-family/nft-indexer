drop view if exists nft_auction_active_bids cascade;
drop view if exists nft_auction_bids_view cascade;
drop view if exists nft_collection_details cascade;
drop view if exists nft_auction_search cascade;
drop view if exists nft_current_price_usd cascade;
drop view if exists nft_deal_history_usd cascade;
drop view if exists nft_details cascade;
drop view if exists nft_direct_buy_best_offer cascade;
drop view if exists nft_direct_buy_usd cascade;
drop view if exists nft_direct_sell_usd cascade;
drop view if exists nft_price_history_usd cascade;

alter table nft_auction
    add column collection t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction
    alter column collection drop default;

alter table nft_direct_buy
    add column collection t_address;

alter table nft_direct_sell
    add column collection t_address;

alter table nft_auction_bid
    add column nft t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction_bid
    alter column nft drop default;

alter table nft_auction_bid
    add column nft_owner t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction_bid
    alter column nft_owner drop default;

alter table nft_auction_bid
    add column collection t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction_bid
    alter column collection drop default;

alter table nft_auction_bid
    add column price_token t_address not null default '0:0000000000000000000000000000000000000000000000000000000000000000';
alter table nft_auction_bid
    alter column price_token drop default;

alter table nft_auction_bid
    add primary key (auction, buyer, price, created_at);

alter table nft
    add column id numeric not null default 0;
alter table nft
    alter column id drop default;

alter table nft_auction
    drop column closing_price_usd;

alter table nft_collection
    drop total_price;
alter table nft_collection
    drop column max_price;

drop table nft_current_price;
drop table nft_deal_history;
drop table search_index;

alter table nft_direct_buy
    drop column buy_price_usd;
alter table nft_direct_sell
    drop column sell_price_usd;

update nft_price_history
set price_token = '0:0000000000000000000000000000000000000000000000000000000000000000'
where price_token is null;

update nft_price_history
set nft = '0:0000000000000000000000000000000000000000000000000000000000000000'
where nft is null;

update nft_price_history
set ts = to_timestamp(0)
where ts is null;

alter table nft_price_history
    alter column price_token set not null;
alter table nft_price_history
    alter column nft set not null;
alter table nft_price_history
    alter column ts set not null;

alter table nft_price_history
    add column usd_price numeric;

alter table roots
    add column expiry_date timestamp;
alter table roots
    alter code set not null;
alter table deployed_offers
    add column created timestamp not null default to_timestamp(0);
alter table deployed_offers
    alter column created drop default;

create or replace view offers_whitelist as
select of.address
from deployed_offers of
         inner join roots r on r.address = of.root and (r.expiry_date is null or r.expiry_date >= of.created);



create or replace view nft_direct_buy_usd as
select s.address,
       s.root,
       s.nft,
       n.collection,
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
         inner join roots r on r.address = s.root
         left join nft n on n.address = s.nft and not n.burned
         left join token_usd_prices p on s.price_token = p.token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_direct_buy_best_offer as
select distinct on (s.nft) s.address,
                           s.root,
                           s.nft,
                           s.collection,
                           s.price_token,
                           s.price,
                           s.buyer,
                           s.finished_at,
                           s.expired_at,
                           s.state,
                           s.created,
                           s.updated,
                           s.tx_lt
from nft_direct_buy_usd s
         inner join roots r on r.address = s.root
         left join token_usd_prices p on s.price_token = p.token
where s.state = 'active'::direct_buy_state
order by s.nft desc;

create or replace view nft_details as
select n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name::text                                                                                        as name,
       n.description,
       n.burned,
       n.updated,
       n.owner_update_lt                                                                                   as tx_lt,
       m.meta,
       auc.auction,
       auc."auction_status: _",
       sale.forsale,
       sale."forsale_status: _",
       ( select s.address as best_offer
         from nft_direct_buy_best_offer s
         where s.nft = n.address
         limit 1 )                                                                                         as best_offer,
       least(auc.price_usd, sale.price_usd)                                                                as floor_price_usd,
       last_deal.last_price                                                                                as deal_price_usd,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.min_bid
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.price
            else null::numeric end                                                                         as floor_price,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.token::character varying
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.token::character varying
            else null::character varying end                                                               as floor_price_token,
       nft_id.id                                                                                           as nft_id
from nft n
         left join lateral ( select max(ne.args ->> 'id')::text as id
                             from nft_events ne
                             where ne.event_type = 'nft_created'
                               and ne.nft = n.address
                             group by ne.nft ) nft_id on true
         left join lateral ( select ag.price as last_price
                             from ( select s.price * tup.usd_price as price, s.created
                                    from nft_direct_sell s
                                             inner join roots r on r.address = s.root
                                             left join token_usd_prices tup on tup.token = s.price_token
                                    where s.state = 'filled'::direct_sell_state
                                      and s.nft = n.address
                                    union all
                                    select s.price * tup.usd_price, s.created
                                    from nft_direct_buy s
                                             inner join roots r on r.address = s.root
                                             left join token_usd_prices tup on tup.token = s.price_token
                                    where s.state = 'filled'::direct_buy_state
                                      and s.nft = n.address
                                    union all
                                    select s.max_bid * tup.usd_price, s.created_at
                                    from nft_auction s
                                             inner join roots r on r.address = s.root
                                             left join token_usd_prices tup on tup.token = s.price_token
                                    where s.status = 'completed'::auction_status
                                      and s.nft = n.address ) ag
                             order by ag.created desc
                             limit 1 ) last_deal on true
         left join lateral ( select a.address                 as auction,
                                    a.status                  as "auction_status: _",
                                    a.min_bid * tup.usd_price as price_usd,
                                    tup.token,
                                    a.min_bid
                             from nft_auction a
                                      inner join roots r on r.address = a.root
                                      left join token_usd_prices tup on tup.token = a.price_token and
                                                                        (a.status = 'active'::auction_status and
                                                                         (a.finished_at = to_timestamp(0) or a.finished_at > now()::timestamp))
                             where a.nft = n.address
                               and a.status = 'active'::auction_status
                             limit 1 ) auc on true
         left join nft_metadata m on m.nft = n.address
         left join lateral ( select s.address               as forsale,
                                    s.state                 as "forsale_status: _",
                                    s.price * tup.usd_price as price_usd,
                                    s.price,
                                    tup.token
                             from nft_direct_sell s
                                      inner join roots r on r.address = s.root
                                      left join token_usd_prices tup on tup.token = s.price_token and
                                                                        s.state = 'active'::direct_sell_state and
                                                                        (s.expired_at = to_timestamp(0) or s.expired_at > now())
                             where s.nft = n.address
                               and s.state = 'active'::direct_sell_state
                             limit 1 ) sale on true
where not n.burned;

create or replace view nft_direct_sell_usd as
select s.address,
       s.nft,
       n.collection,
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
         inner join roots r on r.address = s.root
         join nft n on n.address = s.nft and not n.burned
         left join token_usd_prices p on s.price_token = p.token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_auction_bids_view as
select b.auction,
       b.buyer,
       b.price,
       b.created_at,
       b.next_bid_value,
       b.tx_lt,
       (max(l.created_at) = b.created_at) as active,
       (b.price * p.usd_price)            as usd_price,
       (b.next_bid_value * p.usd_price)   as next_bid_usd_value,
       a.nft,
       n.collection,
       a.price_token,
       n.owner
from nft_auction_bid b
         inner join nft_auction_bid l on (l.auction = b.auction) and (l.declined is null or l.declined is false)
         inner join nft_auction a on a.address = b.auction and a.status <> 'completed'::auction_status
         inner join roots r on r.address = a.root
         inner join nft n on n.address = a.nft and not n.burned
         left join token_usd_prices p on p.token = a.price_token
where b.declined is null
   or b.declined is false
group by b.auction, b.buyer, b.price, b.created_at, b.next_bid_value, b.tx_lt, p.usd_price, a.nft, a.price_token,
         n.collection, n.owner;

create or replace view nft_auction_active_bids as
select distinct on (b.auction) b.auction, b.buyer, b.price, b.next_bid_value, b.declined, b.created_at, b.tx_lt
from nft_auction_bid b
where b.declined is null
   or b.declined is false
order by b.auction, b.created_at desc;

create or replace view nft_auction_search as
select a.address,
       a.nft,
       a.wallet_for_bids,
       a.price_token,
       a.start_price,
       a.max_bid,
       a.min_bid,
       case when a.status = 'active'::auction_status and to_timestamp(0) < a.finished_at and a.finished_at < now()::timestamp
                then 'expired'::auction_status
            else a.status end      as "status: _",
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
         inner join roots r on r.address = a.root
         left join nft n on n.address = a.nft and not n.burned
         left join nft_auction_bid b on b.auction = a.address and (b.declined is null or b.declined is false)
         left join nft_auction_bids_view v on v.auction = a.address and v.active is true
         left join token_usd_prices p on p.token = a.price_token
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = a.address ) as ev on true
group by a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price, ev.fee_numerator, ev.fee_denominator;

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
                                      inner join roots r on r.address = na.root
                                      inner join nft n
                                                 on n.address = na.nft and n.collection = c.address and not n.burned
                                      left join token_usd_prices tup on tup.token = na.price_token
                             where na.status = 'active'::auction_status
                               and (na.finished_at = to_timestamp(0) or na.finished_at > now()::timestamp) ) auction
                   on true
         left join lateral ( select min(ds.price * tup.usd_price) as usd
                             from nft_direct_sell ds
                                      inner join roots r on r.address = ds.root
                                      inner join nft n
                                                 on n.address = ds.nft and n.collection = c.address and not n.burned
                                      left join token_usd_prices tup on tup.token = ds.price_token
                             where ds.state = 'active'::direct_sell_state
                               and (ds.expired_at = to_timestamp(0) or ds.expired_at > now()::timestamp) ) direct_sell
                   on true
         left join lateral ( select count(1) as cnt, count(distinct owner) as owners_count
                             from nft n
                             where n.burned is false
                               and n.collection = c.address ) nft_counter on true
         left join lateral ( select sum(ag.price_usd) as usd
                             from ( select coalesce(tup.usd_price * ndb.price, (0)::numeric) as price_usd
                                    from nft_direct_buy ndb
                                             inner join roots r on r.address = ndb.root
                                             inner join nft n
                                                        on n.address = ndb.nft and n.collection = c.address and not n.burned
                                             left join token_usd_prices tup on tup.token = ndb.price_token
                                    where ndb.state = 'filled'::direct_buy_state
                                    union all
                                    select coalesce(tup.usd_price * nds.price, (0)::numeric) as "coalesce"
                                    from nft_direct_sell nds
                                             inner join roots r on r.address = nds.root
                                             inner join nft n
                                                        on n.address = nds.nft and n.collection = c.address and not n.burned
                                             left join token_usd_prices tup on tup.token = nds.price_token
                                    where nds.state = 'filled'::direct_sell_state
                                    union all
                                    select coalesce(tup.usd_price * na.max_bid, (0)::numeric) as "coalesce"
                                    from nft_auction na
                                             inner join roots r on r.address = na.root
                                             inner join nft n
                                                        on na.nft = n.address and not n.burned and n.collection = c.address
                                             left join token_usd_prices tup on tup.token = na.price_token
                                    where na.status = 'completed'::auction_status ) ag ) total_volume on true;
