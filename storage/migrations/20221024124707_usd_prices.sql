create table if not exists token_usd_prices(
	token t_address not null primary key,
    usd_price decimal(40, 12) not null,
    ts timestamp not null
);

create or replace view nft_auction_active_bids as 
    SELECT DISTINCT ON (b.auction) b.*
    FROM nft_auction_bid b
    WHERE b.declined is null or b.declined is false
    ORDER BY b.auction ASC, b.created_at DESC;

create or replace view nft_auction_bids_view as 
    SELECT 
        b.auction,
        b.buyer,
        b.price,
        b.created_at,
        b.next_bid_value,
        b.tx_lt,
        (max(l.created_at) = b.created_at) as active,
        b.price * p.usd_price as usd_price,
        b.next_bid_value * p.usd_price as next_bid_usd_value,
        a.nft,
        n.collection,
        a.price_token,
        n.owner
    FROM nft_auction_bid b
    INNER JOIN nft_auction_bid l ON l.auction = b.auction and (l.declined is null or l.declined is false)
    INNER JOIN nft_auction a ON a.address = b.auction and a.status != 'completed'
    INNER JOIN events_whitelist wl ON wl.address = a.address
    INNER JOIN nft n ON n.address = a.nft
    LEFT JOIN token_usd_prices p ON p.token = a.price_token
    WHERE b.declined is null or b.declined is false
    GROUP BY b.auction, b.buyer, b.price, 
            b.created_at, b.next_bid_value, b.tx_lt, 
            p.usd_price, a.nft, n.collection, a.price_token, n.owner;


create or replace view nft_auction_search as 
    SELECT
        a.address,
        a.nft,
        a.wallet_for_bids,
        a.price_token,
        a.start_price,
        a.max_bid, a.min_bid,
        a.status AS "status: _",
        a.created_at,
        a.finished_at,
        a.tx_lt,
        v.buyer as last_bid_from,
        count(b.*) as bids_count,
        max(b.price) as last_bid_value,
        max(b.price) * p.usd_price as last_bid_usd_value,
        max(b.created_at) as last_bid_ts,
        a.start_price * p.usd_price as start_usd_price,
        a.max_bid * p.usd_price as max_usd_bid,
        a.min_bid * p.usd_price as min_usd_bid
    FROM nft_auction a
    INNER JOIN events_whitelist wl ON wl.address = a.address
    LEFT JOIN nft_auction_bid b ON b.auction = a.address and (b.declined is null or b.declined is false)
    LEFT JOIN nft_auction_bids_view v ON v.auction = a.address and v.active is true
    LEFT JOIN token_usd_prices p ON p.token = a.price_token
    GROUP BY 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, p.usd_price;


create or replace view nft_direct_buy_usd as 
    SELECT s.*, (s.price * p.usd_price) as usd_price
    FROM nft_direct_buy s
    INNER JOIN events_whitelist wl ON wl.address = s.address
    LEFT JOIN token_usd_prices p ON s.price_token = p.token;

create or replace view nft_direct_sell_usd as 
    SELECT s.*, (s.price * p.usd_price) as usd_price
    FROM nft_direct_sell s
    INNER JOIN events_whitelist wl ON wl.address = s.address
    LEFT JOIN token_usd_prices p ON s.price_token = p.token;

create or replace view nft_direct_buy_best_offer as 
    SELECT DISTINCT ON (s.nft) s.*
    FROM nft_direct_buy_usd s
    INNER JOIN events_whitelist wl ON wl.address = s.address
    INNER JOIN token_usd_prices p ON s.price_token = p.token
    WHERE s.state = 'active' AND s.usd_price is not null
    ORDER BY s.nft ASC, s.usd_price DESC;

drop view nft_details;
create or replace view nft_details as 
    SELECT n.address, n.collection, n.owner, n.manager, n.name, n.description, n.burned, n.updated, n.owner_update_lt as tx_lt,
        m.meta as meta,
        (select a.address as auction from nft_auction a 
        INNER JOIN events_whitelist wl ON wl.address = a.address 
        where a.nft = n.address and a.status in ('active', 'expired') limit 1),
        (select a.status as "auction_status: _" from nft_auction a 
        INNER JOIN events_whitelist wl ON wl.address = a.address 
        where a.nft = n.address and a.status in ('active', 'expired') limit 1),
        (select s.address as forsale from nft_direct_sell s 
        INNER JOIN events_whitelist wl ON wl.address = s.address 
        where s.nft = n.address and s.state in ('active', 'expired') limit 1),
        (select s.state as "forsale_status: _" from nft_direct_sell s 
        INNER JOIN events_whitelist wl ON wl.address = s.address 
        where s.nft = n.address and s.state in ('active', 'expired') limit 1),
        (select s.address as best_offer from nft_direct_buy_best_offer s 
        INNER JOIN events_whitelist wl ON wl.address = s.address where s.nft = n.address limit 1)
    FROM nft n 
    LEFT JOIN nft_metadata m ON m.nft = n.address;

