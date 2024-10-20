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
         left join lateral ( select (ne.args ->> 'fee_numerator')::int   as fee_numerator,
                                    (ne.args ->> 'fee_denominator')::int as fee_denominator
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
left join lateral ( select (ne.args ->> 'fee_numerator')::int   as fee_numerator,
                                    (ne.args ->> 'fee_denominator')::int as fee_denominator
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
        left join lateral ( select (ne.args ->> 'fee_numerator')::int   as fee_numerator,
                                            (ne.args ->> 'fee_denominator')::int as fee_denominator
                                     from nft_events ne
                                     where ne.event_type = 'market_fee_changed'
                                       and ne.args ->> 'auction' = a.address ) as ev on true
GROUP BY a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status,
         a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price, ev.fee_numerator,  ev.fee_denominator;

CREATE INDEX nft_events_args_auc_idx ON nft_events USING btree (((args ->> 'auction')::text));





