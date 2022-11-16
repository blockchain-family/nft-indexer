drop view nft_details;
create or replace view nft_details as 
    SELECT n.*,
        m.meta as meta,
        (select a.address from nft_auction a where a.nft = n.address and a.status in ('active', 'expired') limit 1) as auction,
        (select s.address from nft_direct_sell s where s.nft = n.address and s.state = 'active' limit 1) as forsale
    FROM nft n 
    LEFT JOIN nft_metadata m ON m.nft = n.address;


create or replace view nft_auction_bids_view as 
    SELECT 
        b.auction,
        b.buyer,
        b.price,
        b.created_at,
        b.next_bid_value,
        b.tx_lt,
        (max(l.created_at) = b.created_at) as active
    FROM nft_auction_bid b
    INNER JOIN nft_auction_bid l ON l.auction = b.auction and (l.declined is null or l.declined is false)
    WHERE b.declined is null or b.declined is false
    GROUP BY b.auction, b.buyer, b.price, b.created_at, b.next_bid_value, b.tx_lt;

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
        max(b.price) as last_bid_value
        --max(b.created_at) as last_bid_ts
    FROM nft_auction a
    LEFT JOIN nft_auction_bid b ON b.auction = a.address and (b.declined is null or b.declined is false)
    LEFT JOIN nft_auction_bids_view v ON v.auction = a.address and v.active is true
    GROUP BY 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12;