create or replace view nft_details
            (address, collection, owner, manager, name, description, burned, updated, tx_lt, meta, auction,
             "auction_status: _", forsale, "forsale_status: _", best_offer, floor_price_usd, deal_price_usd,
             floor_price, floor_price_token, nft_id)
as
SELECT n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name::text                         AS name,
       n.description,
       n.burned,
       n.updated,
       n.owner_update_lt                    AS tx_lt,
       m.meta,
       auc.auction,
       auc."auction_status: _",
       sale.forsale,
       sale."forsale_status: _",
       (SELECT s.address AS best_offer
        FROM nft_direct_buy_best_offer s
                 JOIN events_whitelist wl ON wl.address::text = s.address::text
        WHERE s.nft::text = n.address::text
        LIMIT 1)                            AS best_offer,
       LEAST(auc.price_usd, sale.price_usd) AS floor_price_usd,
       last_deal.last_price                 AS deal_price_usd,
       CASE
           WHEN LEAST(auc.price_usd, sale.price_usd) = auc.price_usd THEN auc.min_bid
           WHEN LEAST(auc.price_usd, sale.price_usd) = sale.price_usd THEN sale.price
           ELSE NULL::numeric
           END                              AS floor_price,
       CASE
           WHEN LEAST(auc.price_usd, sale.price_usd) = auc.price_usd THEN auc.token::character varying
           WHEN LEAST(auc.price_usd, sale.price_usd) = sale.price_usd THEN sale.token::character varying
           ELSE NULL::character varying
           END                              AS floor_price_token,
       nft_id.id as nft_id
FROM nft n
         left join lateral (
             select max(ne.args ->> 'id')::text as id from nft_events ne
             where ne.event_type = 'nft_created'
             and ne.nft = n.address
             group by ne.nft
    ) nft_id on true
         LEFT JOIN LATERAL ( SELECT ag.price AS last_price
                             FROM (SELECT s.price * tup.usd_price AS price,
                                          s.created
                                   FROM nft_direct_sell s
                                            JOIN token_usd_prices tup ON tup.token::text = s.price_token::text
                                   WHERE s.state = 'filled'::direct_sell_state
                                     AND s.nft::text = n.address::text
                                   UNION ALL
                                   SELECT s.price * tup.usd_price,
                                          s.created
                                   FROM nft_direct_buy s
                                            JOIN token_usd_prices tup ON tup.token::text = s.price_token::text
                                   WHERE s.state = 'filled'::direct_buy_state
                                     AND s.nft::text = n.address::text
                                   UNION ALL
                                   SELECT s.max_bid * tup.usd_price,
                                          s.created_at
                                   FROM nft_auction s
                                            JOIN token_usd_prices tup ON tup.token::text = s.price_token::text
                                   WHERE s.status = 'completed'::auction_status
                                     AND s.nft::text = n.address::text) ag
                             ORDER BY ag.created DESC
                             LIMIT 1) last_deal ON true
         LEFT JOIN LATERAL ( SELECT a.address                 AS auction,
                                    a.status                  AS "auction_status: _",
                                    a.min_bid * tup.usd_price AS price_usd,
                                    tup.token,
                                    a.min_bid
                             FROM nft_auction a
                                      JOIN events_whitelist wl ON wl.address::text = a.address::text
                                      LEFT JOIN token_usd_prices tup
                                                ON tup.token::text = a.price_token::text AND a.status = 'active'::auction_status
                             WHERE a.nft::text = n.address::text
                               AND (a.status = ANY (ARRAY ['active'::auction_status, 'expired'::auction_status]))
                             LIMIT 1) auc ON true
         LEFT JOIN nft_metadata m ON m.nft::text = n.address::text
         LEFT JOIN LATERAL ( SELECT s.address               AS forsale,
                                    s.state                 AS "forsale_status: _",
                                    s.price * tup.usd_price AS price_usd,
                                    s.price,
                                    tup.token
                             FROM nft_direct_sell s
                                      JOIN events_whitelist wl ON wl.address::text = s.address::text
                                      LEFT JOIN token_usd_prices tup
                                                ON tup.token::text = s.price_token::text AND s.state = 'active'::direct_sell_state
                             WHERE s.nft::text = n.address::text
                               AND (s.state = ANY (ARRAY ['active'::direct_sell_state, 'expired'::direct_sell_state]))
                             LIMIT 1) sale ON true
WHERE NOT n.burned;



create or replace view nft_direct_buy_usd
            (address, nft, collection, price_token, price, buy_price_usd, buyer, finished_at, expired_at, state,
             created, updated, tx_lt, usd_price, fee_numerator, fee_denominator) as
SELECT s.address,
       s.nft,
       n.collection,
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
       s.price * p.usd_price AS usd_price,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_direct_buy s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_direct_sell_usd
            (address, nft, collection, price_token, price, sell_price_usd, seller, finished_at, expired_at, state,
             created, updated, tx_lt, usd_price, fee_numerator, fee_denominator)
as
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
       s.price * p.usd_price AS usd_price,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_direct_sell s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text
left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address ) as ev on true;

create or replace view nft_auction_search
            (address, nft, wallet_for_bids, price_token, start_price, max_bid, min_bid, "status: _", created_at,
             finished_at, tx_lt, last_bid_from, bids_count, last_bid_value, last_bid_usd_value, last_bid_ts,
             start_usd_price, max_usd_bid, min_usd_bid, fee_numerator, fee_denominator)
as
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
       a.min_bid * p.usd_price     AS min_usd_bid,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_auction a
         JOIN events_whitelist wl ON wl.address::text = a.address::text
         JOIN nft n ON n.address::text = a.nft::text AND NOT n.burned
         LEFT JOIN nft_auction_bid b
                   ON b.auction::text = a.address::text AND (b.declined IS NULL OR b.declined IS FALSE)
         LEFT JOIN nft_auction_bids_view v ON v.auction::text = a.address::text AND v.active IS TRUE
         LEFT JOIN token_usd_prices p ON p.token::text = a.price_token::text
        left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                            (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                                     from nft_events ne
                                     where ne.event_type = 'market_fee_changed'
                                       and ne.args ->> 'auction' = a.address ) as ev on true
GROUP BY a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price, ev.fee_numerator,  ev.fee_denominator;


create or replace view nft_direct_buy_usd
            (address, nft, collection, price_token, price, buy_price_usd, buyer, finished_at, expired_at, state,
             created, updated, tx_lt, usd_price, fee_numerator, fee_denominator) as
SELECT s.address,
       s.nft,
       n.collection,
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
       s.price * p.usd_price AS usd_price,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_direct_buy s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text
         left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address order by ne.created_at desc, ne.created_lt desc, id desc limit 1) as ev on true;

create or replace view nft_direct_sell_usd
            (address, nft, collection, price_token, price, sell_price_usd, seller, finished_at, expired_at, state,
             created, updated, tx_lt, usd_price, fee_numerator, fee_denominator)
as
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
       s.price * p.usd_price AS usd_price,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_direct_sell s
         JOIN events_whitelist wl ON wl.address::text = s.address::text
         JOIN nft n ON n.address::text = s.nft::text AND NOT n.burned
         LEFT JOIN token_usd_prices p ON s.price_token::text = p.token::text
left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                    (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                             from nft_events ne
                             where ne.event_type = 'market_fee_changed'
                               and ne.args ->> 'auction' = s.address order by ne.created_at desc, ne.created_lt desc, id desc limit 1) as ev on true;

create or replace view nft_auction_search
            (address, nft, wallet_for_bids, price_token, start_price, max_bid, min_bid, "status: _", created_at,
             finished_at, tx_lt, last_bid_from, bids_count, last_bid_value, last_bid_usd_value, last_bid_ts,
             start_usd_price, max_usd_bid, min_usd_bid, fee_numerator, fee_denominator)
as
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
       a.min_bid * p.usd_price     AS min_usd_bid,
       ev.fee_numerator,
       ev.fee_denominator
FROM nft_auction a
         JOIN events_whitelist wl ON wl.address::text = a.address::text
         JOIN nft n ON n.address::text = a.nft::text AND NOT n.burned
         LEFT JOIN nft_auction_bid b
                   ON b.auction::text = a.address::text AND (b.declined IS NULL OR b.declined IS FALSE)
         LEFT JOIN nft_auction_bids_view v ON v.auction::text = a.address::text AND v.active IS TRUE
         LEFT JOIN token_usd_prices p ON p.token::text = a.price_token::text
        left join lateral ( select (ne.args -> 'fee' -> 'numerator')::int   as fee_numerator,
                                            (ne.args -> 'fee' -> 'denominator')::int as fee_denominator
                                     from nft_events ne
                                     where ne.event_type = 'market_fee_changed'
                                       and ne.args ->> 'auction' = a.address order by ne.created_at desc, ne.created_lt desc, id desc limit 1) as ev on true
GROUP BY a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price, ev.fee_numerator,  ev.fee_denominator;
