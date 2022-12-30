drop table if exists search_index;
create table search_index(
    address t_address not null primary key,
    typ event_category not null,
    ts timestamp not null,
    name varchar(400) null,
    nft t_address null,
    collection t_address null,
    search tsvector generated ALWAYS as (to_tsvector('english', coalesce(name, ''))) stored
);

create index search_index_idx on search_index using GIN (search);


create table nft_deal_history
(
	source t_address not null,
	source_type nft_price_source not null,
	ts timestamp not null,
	price numeric(40) not null,
	price_token t_address,
	nft t_address,
	collection t_address,
	buyer t_address,
	seller t_address
);


create index idx_nft_deal_history_nft
	on nft_deal_history (nft);

create index idx_nft_deal_history_collection
	on nft_deal_history (collection);

create index idx_nft_deal_history_ts
	on nft_deal_history (ts);

create index idx_nft_deal_history_buyer
	on nft_deal_history (buyer);

create index idx_nft_deal_history_seller
	on nft_deal_history (seller);

create table nft_current_price
(
	nft t_address not null
		constraint nft_current_price_pkey
			primary key,
	source t_address not null,
	source_type nft_price_source not null,
	ts timestamp not null,
	price numeric(40) not null,
	price_token t_address,
	collection t_address
);


create index idx_nft_current_price_collection
	on nft_current_price (collection);

create index idx_nft_current_price_ts
	on nft_current_price (ts);

create or replace view nft_auction_active_bids(auction, buyer, price, next_bid_value, declined, created_at, tx_lt) as
	SELECT DISTINCT ON (b.auction) b.auction,
                               b.buyer,
                               b.price,
                               b.next_bid_value,
                               b.declined,
                               b.created_at,
                               b.tx_lt
FROM nft_auction_bid b
WHERE b.declined IS NULL
   OR b.declined IS FALSE
ORDER BY b.auction, b.created_at DESC;


create or replace view nft_deal_history_usd(source, source_type, ts, price, price_token, nft, collection, buyer, seller, usd_price) as
	SELECT h.source,
       h.source_type,
       h.ts,
       h.price,
       h.price_token,
       h.nft,
       h.collection,
       h.buyer,
       h.seller,
       h.price * u.usd_price AS usd_price
FROM nft_deal_history h
         JOIN token_usd_prices u ON u.token::text = h.price_token::text;


create or replace view nft_current_price_usd(source, source_type, ts, price, price_token, nft, collection, usd_price) as
	SELECT h.source,
       h.source_type,
       h.ts,
       h.price,
       h.price_token,
       h.nft,
       h.collection,
       h.price * u.usd_price AS usd_price
FROM nft_current_price h
         JOIN token_usd_prices u ON u.token::text = h.price_token::text;


create or replace view nft_direct_buy_usd(address, nft, collection, price_token, price, buy_price_usd, buyer, finished_at, expired_at, state, created, updated, tx_lt, usd_price) as
	SELECT s.address,
       s.nft,
       s.collection,
       s.price_token,
       s.price,
       s.buy_price_usd,
       s.buyer,
       s.finished_at,
       s.expired_at,
       s.state,
       s.created,
       s.updated,
       s.tx_lt,
       s.price * p.usd_price AS usd_price
FROM nft_direct_buy s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text;


create or replace view nft_direct_sell_usd(address, nft, collection, price_token, price, sell_price_usd, seller, finished_at, expired_at, state, created, updated, tx_lt, usd_price) as
	SELECT s.address,
       s.nft,
       s.collection,
       s.price_token,
       s.price,
       s.sell_price_usd,
       s.seller,
       s.finished_at,
       s.expired_at,
       s.state,
       s.created,
       s.updated,
       s.tx_lt,
       s.price * p.usd_price AS usd_price
FROM nft_direct_sell s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text;


create or replace view nft_direct_buy_best_offer(address, nft, collection, price_token, price, buy_price_usd, buyer, finished_at, expired_at, state, created, updated, tx_lt, usd_price) as
	SELECT DISTINCT ON (s.nft) s.address,
                           s.nft,
                           s.collection,
                           s.price_token,
                           s.price,
                           s.buy_price_usd,
                           s.buyer,
                           s.finished_at,
                           s.expired_at,
                           s.state,
                           s.created,
                           s.updated,
                           s.tx_lt,
                           s.usd_price
FROM nft_direct_buy_usd s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN token_usd_prices p ON s.price_token::text = p.token::text
WHERE s.state = 'active'::direct_buy_state
  AND s.usd_price IS NOT NULL
ORDER BY s.nft, s.usd_price DESC;


create or replace view nft_details(address, collection, owner, manager, name, description, burned, updated, tx_lt, meta, auction, "auction_status: _", forsale, "forsale_status: _", best_offer, floor_price_usd, deal_price_usd) as
	SELECT n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name,
       n.description,
       n.burned,
       n.updated,
       n.owner_update_lt AS tx_lt,
       m.meta,
       (SELECT a.address AS auction
        FROM nft_auction a
                 JOIN events_whitelist wl ON wl.address::text = a.address::text
        WHERE a.nft::text = n.address::text
          AND (a.status = ANY (ARRAY ['active'::auction_status, 'expired'::auction_status]))
        LIMIT 1)         AS auction,
       (SELECT a.status AS "auction_status: _"
        FROM nft_auction a
                 JOIN events_whitelist wl ON wl.address::text = a.address::text
        WHERE a.nft::text = n.address::text
          AND (a.status = ANY (ARRAY ['active'::auction_status, 'expired'::auction_status]))
        LIMIT 1)         AS "auction_status: _",
       (SELECT s.address AS forsale
        FROM nft_direct_sell s
                 JOIN events_whitelist wl ON wl.address::text = s.address::text
        WHERE s.nft::text = n.address::text
          AND (s.state = ANY (ARRAY ['active'::direct_sell_state, 'expired'::direct_sell_state]))
        LIMIT 1)         AS forsale,
       (SELECT s.state AS "forsale_status: _"
        FROM nft_direct_sell s
                 JOIN events_whitelist wl ON wl.address::text = s.address::text
        WHERE s.nft::text = n.address::text
          AND (s.state = ANY (ARRAY ['active'::direct_sell_state, 'expired'::direct_sell_state]))
        LIMIT 1)         AS "forsale_status: _",
       (SELECT s.address AS best_offer
        FROM nft_direct_buy_best_offer s
                 JOIN events_whitelist wl ON wl.address::text = s.address::text
        WHERE s.nft::text = n.address::text
        LIMIT 1)         AS best_offer,
       (SELECT COALESCE(s.usd_price, 0::numeric) AS floor_price_usd
        FROM nft_current_price_usd s
        WHERE s.nft::text = n.address::text
        LIMIT 1)         AS floor_price_usd,
       (SELECT COALESCE(s.usd_price, 0::numeric) AS deal_price_usd
        FROM nft_deal_history_usd s
        WHERE s.nft::text = n.address::text
        ORDER BY s.ts DESC
        LIMIT 1)         AS deal_price_usd
FROM nft n
         LEFT JOIN nft_metadata m ON m.nft::text = n.address::text
WHERE NOT n.burned;


create or replace view nft_collection_details(address, owner, name, description, created, updated, wallpaper, logo, total_price, max_price, owners_count, verified, nft_count, floor_price_usd, total_volume_usd) as
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
       count(n.*)             AS nft_count,
       sum(n.floor_price_usd) AS floor_price_usd,
       sum(n.deal_price_usd)  AS total_volume_usd
FROM nft_collection c
         JOIN nft_details n ON n.collection::text = c.address::text
GROUP BY c.address, c.owner, c.name, c.description, c.created, c.updated, c.wallpaper, c.logo, c.total_price,
         c.max_price, c.owners_count, c.verified;


create or replace view nft_price_history_usd(source, source_type, ts, price, price_token, nft, collection, usd_price) as
	SELECT h.source,
       h.source_type,
       h.ts,
       h.price,
       h.price_token,
       h.nft,
       h.collection,
       h.price * u.usd_price AS usd_price
FROM nft_price_history h
         JOIN nft n ON n.address::text = h.nft::text AND NOT n.burned
         JOIN token_usd_prices u ON u.token::text = h.price_token::text;


create or replace view nft_auction_bids_view(auction, buyer, price, created_at, next_bid_value, tx_lt, active, usd_price, next_bid_usd_value, nft, collection, price_token, owner) as
	SELECT b.auction,
       b.buyer,
       b.price,
       b.created_at,
       b.next_bid_value,
       b.tx_lt,
       max(l.created_at) = b.created_at AS active,
       b.price * p.usd_price            AS usd_price,
       b.next_bid_value * p.usd_price   AS next_bid_usd_value,
       a.nft,
       n.collection,
       a.price_token,
       n.owner
FROM nft_auction_bid b
         JOIN nft_auction_bid l ON l.auction::text = b.auction::text AND (l.declined IS NULL OR l.declined IS FALSE)
         JOIN nft_auction a ON a.address::text = b.auction::text AND a.status <> 'completed'::auction_status
         JOIN events_whitelist wl ON wl.address::text = a.address::text
         JOIN nft n ON n.address::text = a.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON p.token::text = a.price_token::text
WHERE b.declined IS NULL
   OR b.declined IS FALSE
GROUP BY b.auction, b.buyer, b.price, b.created_at, b.next_bid_value, b.tx_lt, p.usd_price, a.nft, n.collection,
         a.price_token, n.owner;


create or replace view nft_auction_search(address, nft, wallet_for_bids, price_token, start_price, max_bid, min_bid, "status: _", created_at, finished_at, tx_lt, last_bid_from, bids_count, last_bid_value, last_bid_usd_value, last_bid_ts, start_usd_price, max_usd_bid, min_usd_bid) as
	SELECT a.address,
       a.nft,
       a.wallet_for_bids,
       a.price_token,
       a.start_price,
       a.max_bid,
       a.min_bid,
       a.status                    AS "status: _",
       a.created_at,
       a.finished_at,
       a.tx_lt,
       v.buyer                     AS last_bid_from,
       count(b.*)                  AS bids_count,
       max(b.price)                AS last_bid_value,
       max(b.price) * p.usd_price  AS last_bid_usd_value,
       max(b.created_at)           AS last_bid_ts,
       a.start_price * p.usd_price AS start_usd_price,
       a.max_bid * p.usd_price     AS max_usd_bid,
       a.min_bid * p.usd_price     AS min_usd_bid
FROM nft_auction a
         JOIN events_whitelist wl ON wl.address::text = a.address::text
         JOIN nft n ON n.address::text = a.nft::text AND NOT n.burned
         LEFT JOIN nft_auction_bid b
                   ON b.auction::text = a.address::text AND (b.declined IS NULL OR b.declined IS FALSE)
         LEFT JOIN nft_auction_bids_view v ON v.auction::text = a.address::text AND v.active IS TRUE
         LEFT JOIN token_usd_prices p ON p.token::text = a.price_token::text
GROUP BY a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price;


