drop view nft_details;
drop view nft_direct_buy_best_offer;
drop view nft_auction_search;
drop view nft_auction_active_bids;
drop view nft_deal_history_usd;
drop view nft_collection_details;
drop view nft_price_history_usd;
drop view nft_auction_bids_view;
drop view nft_direct_sell_usd;
drop view nft_current_price_usd;
drop view nft_direct_buy_usd;

alter table token_usd_prices alter column usd_price type numeric(40,30) using usd_price::numeric(40,30);

CREATE VIEW nft_direct_buy_usd AS SELECT s.address,
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
    (s.price * p.usd_price) AS usd_price
   FROM (((nft_direct_buy s
     JOIN events_whitelist wl ON (((wl.address)::text = (s.address)::text)))
     JOIN nft n ON ((((n.address)::text = (s.nft)::text) AND (NOT n.burned))))
     LEFT JOIN token_usd_prices p ON (((s.price_token)::text = (p.token)::text)));
CREATE VIEW nft_current_price_usd AS SELECT h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    h.collection,
    (h.price * u.usd_price) AS usd_price
   FROM (nft_current_price h
     JOIN token_usd_prices u ON (((u.token)::text = (h.price_token)::text)));
     
CREATE VIEW nft_direct_sell_usd AS SELECT s.address,
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
    (s.price * p.usd_price) AS usd_price
   FROM (((nft_direct_sell s
     JOIN events_whitelist wl ON (((wl.address)::text = (s.address)::text)))
     JOIN nft n ON ((((n.address)::text = (s.nft)::text) AND (NOT n.burned))))
     LEFT JOIN token_usd_prices p ON (((s.price_token)::text = (p.token)::text)));
CREATE VIEW nft_auction_bids_view AS SELECT b.auction,
    b.buyer,
    b.price,
    b.created_at,
    b.next_bid_value,
    b.tx_lt,
    (max(l.created_at) = b.created_at) AS active,
    (b.price * p.usd_price) AS usd_price,
    (b.next_bid_value * p.usd_price) AS next_bid_usd_value,
    a.nft,
    n.collection,
    a.price_token,
    n.owner
   FROM (((((nft_auction_bid b
     JOIN nft_auction_bid l ON ((((l.auction)::text = (b.auction)::text) AND ((l.declined IS NULL) OR (l.declined IS FALSE)))))
     JOIN nft_auction a ON ((((a.address)::text = (b.auction)::text) AND (a.status <> 'completed'::auction_status))))
     JOIN events_whitelist wl ON (((wl.address)::text = (a.address)::text)))
     JOIN nft n ON ((((n.address)::text = (a.nft)::text) AND (NOT n.burned))))
     LEFT JOIN token_usd_prices p ON (((p.token)::text = (a.price_token)::text)))
  WHERE ((b.declined IS NULL) OR (b.declined IS FALSE))
  GROUP BY b.auction, b.buyer, b.price, b.created_at, b.next_bid_value, b.tx_lt, p.usd_price, a.nft, n.collection, a.price_token, n.owner;
CREATE VIEW nft_price_history_usd AS SELECT h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    h.collection,
    (h.price * u.usd_price) AS usd_price
   FROM ((nft_price_history h
     JOIN nft n ON ((((n.address)::text = (h.nft)::text) AND (NOT n.burned))))
     JOIN token_usd_prices u ON (((u.token)::text = (h.price_token)::text)));
CREATE VIEW nft_collection_details AS SELECT c.address,
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
    nft_counter.cnt AS nft_count,
    LEAST(direct_sell.usd, auction.usd) AS floor_price_usd,
    COALESCE(total_volume.usd, (0)::numeric) AS total_volume_usd,
    attr.list AS attributes,
    c.first_mint
   FROM (((((nft_collection c
     LEFT JOIN LATERAL ( SELECT json_agg(res.json) AS list
           FROM ( SELECT json_build_object('traitType', na.trait_type, 'traitValues', json_agg(DISTINCT TRIM(BOTH FROM (na.value #>> '{}'::text[])))) AS json
                   FROM nft_attributes na
                  WHERE ((na.collection)::text = (c.address)::text)
                  GROUP BY na.trait_type, na.collection) res) attr ON (true))
     LEFT JOIN LATERAL ( SELECT min((na.min_bid * tup.usd_price)) AS usd
           FROM (((nft_auction na
             JOIN events_whitelist ew ON (((ew.address)::text = (na.address)::text)))
             JOIN nft n ON ((((n.address)::text = (na.nft)::text) AND ((n.collection)::text = (c.address)::text) AND (NOT n.burned))))
             LEFT JOIN token_usd_prices tup ON (((tup.token)::text = (na.price_token)::text)))
          WHERE (na.status = 'active'::auction_status)) auction ON (true))
     LEFT JOIN LATERAL ( SELECT min((ds.price * tup.usd_price)) AS usd
           FROM (((nft_direct_sell ds
             JOIN events_whitelist ew ON (((ew.address)::text = (ds.address)::text)))
             JOIN nft n ON ((((n.address)::text = (ds.nft)::text) AND ((n.collection)::text = (c.address)::text) AND (NOT n.burned))))
             LEFT JOIN token_usd_prices tup ON (((tup.token)::text = (ds.price_token)::text)))
          WHERE (ds.state = 'active'::direct_sell_state)) direct_sell ON (true))
     LEFT JOIN LATERAL ( SELECT count(1) AS cnt
           FROM nft n
          WHERE ((n.burned = false) AND ((n.collection)::text = (c.address)::text))) nft_counter ON (true))
     LEFT JOIN LATERAL ( SELECT sum(ag.price_usd) AS usd
           FROM ( SELECT COALESCE((tup.usd_price * ndb.price), (0)::numeric) AS price_usd
                   FROM (((nft_direct_buy ndb
                     JOIN events_whitelist ew ON (((ew.address)::text = (ndb.address)::text)))
                     JOIN nft n ON ((((n.address)::text = (ndb.nft)::text) AND ((n.collection)::text = (c.address)::text) AND (NOT n.burned))))
                     JOIN token_usd_prices tup ON (((tup.token)::text = (ndb.price_token)::text)))
                  WHERE (ndb.state = 'filled'::direct_buy_state)
                UNION ALL
                 SELECT COALESCE((tup.usd_price * nds.price), (0)::numeric) AS "coalesce"
                   FROM (((nft_direct_sell nds
                     JOIN events_whitelist ew ON (((ew.address)::text = (nds.address)::text)))
                     JOIN nft n ON ((((n.address)::text = (nds.nft)::text) AND ((n.collection)::text = (c.address)::text) AND (NOT n.burned))))
                     JOIN token_usd_prices tup ON (((tup.token)::text = (nds.price_token)::text)))
                  WHERE (nds.state = 'filled'::direct_sell_state)
                UNION ALL
                 SELECT COALESCE((tup.usd_price * na.max_bid), (0)::numeric) AS "coalesce"
                   FROM (((nft_auction na
                     JOIN events_whitelist ew ON (((ew.address)::text = (na.address)::text)))
                     JOIN nft n ON ((((na.nft)::text = (n.address)::text) AND (NOT n.burned) AND ((n.collection)::text = (c.address)::text))))
                     JOIN token_usd_prices tup ON (((tup.token)::text = (na.price_token)::text)))
                  WHERE (na.status = 'completed'::auction_status)) ag) total_volume ON (true));
CREATE VIEW nft_deal_history_usd AS SELECT h.source,
    h.source_type,
    h.ts,
    h.price,
    h.price_token,
    h.nft,
    h.collection,
    h.buyer,
    h.seller,
    (h.price * u.usd_price) AS usd_price
   FROM (nft_deal_history h
     JOIN token_usd_prices u ON (((u.token)::text = (h.price_token)::text)));
CREATE VIEW nft_auction_active_bids AS SELECT DISTINCT ON (b.auction) b.auction,
    b.buyer,
    b.price,
    b.next_bid_value,
    b.declined,
    b.created_at,
    b.tx_lt
   FROM nft_auction_bid b
  WHERE ((b.declined IS NULL) OR (b.declined IS FALSE))
  ORDER BY b.auction, b.created_at DESC;
CREATE VIEW nft_auction_search AS SELECT a.address,
    a.nft,
    a.wallet_for_bids,
    a.price_token,
    a.start_price,
    a.max_bid,
    a.min_bid,
    a.status AS "status: _",
    a.created_at,
    a.finished_at,
    a.tx_lt,
    v.buyer AS last_bid_from,
    count(b.*) AS bids_count,
    max(b.price) AS last_bid_value,
    (max(b.price) * p.usd_price) AS last_bid_usd_value,
    max(b.created_at) AS last_bid_ts,
    (a.start_price * p.usd_price) AS start_usd_price,
    (a.max_bid * p.usd_price) AS max_usd_bid,
    (a.min_bid * p.usd_price) AS min_usd_bid
   FROM (((((nft_auction a
     JOIN events_whitelist wl ON (((wl.address)::text = (a.address)::text)))
     JOIN nft n ON ((((n.address)::text = (a.nft)::text) AND (NOT n.burned))))
     LEFT JOIN nft_auction_bid b ON ((((b.auction)::text = (a.address)::text) AND ((b.declined IS NULL) OR (b.declined IS FALSE)))))
     LEFT JOIN nft_auction_bids_view v ON ((((v.auction)::text = (a.address)::text) AND (v.active IS TRUE))))
     LEFT JOIN token_usd_prices p ON (((p.token)::text = (a.price_token)::text)))
  GROUP BY a.address, a.nft, a.wallet_for_bids, a.price_token, a.start_price, a.max_bid, a.min_bid, a.status, a.created_at, a.finished_at, a.tx_lt, v.buyer, p.usd_price;
CREATE VIEW nft_direct_buy_best_offer AS SELECT DISTINCT ON (s.nft) s.address,
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
   FROM ((nft_direct_buy_usd s
     JOIN events_whitelist wl ON (((wl.address)::text = (s.address)::text)))
     JOIN token_usd_prices p ON (((s.price_token)::text = (p.token)::text)))
  WHERE ((s.state = 'active'::direct_buy_state) AND (s.usd_price IS NOT NULL))
  ORDER BY s.nft, s.usd_price DESC;
CREATE VIEW nft_details AS SELECT n.address,
    n.collection,
    n.owner,
    n.manager,
    (n.name)::text AS name,
    n.description,
    n.burned,
    n.updated,
    n.owner_update_lt AS tx_lt,
    m.meta,
    auc.auction,
    auc."auction_status: _",
    sale.forsale,
    sale."forsale_status: _",
    ( SELECT s.address AS best_offer
           FROM (nft_direct_buy_best_offer s
             JOIN events_whitelist wl ON (((wl.address)::text = (s.address)::text)))
          WHERE ((s.nft)::text = (n.address)::text)
         LIMIT 1) AS best_offer,
    LEAST(auc.price_usd, sale.price_usd) AS floor_price_usd,
    last_deal.last_price AS deal_price_usd,
        CASE
            WHEN (LEAST(auc.price_usd, sale.price_usd) = auc.price_usd) THEN auc.min_bid
            WHEN (LEAST(auc.price_usd, sale.price_usd) = sale.price_usd) THEN sale.price
            ELSE NULL::numeric
        END AS floor_price,
        CASE
            WHEN (LEAST(auc.price_usd, sale.price_usd) = auc.price_usd) THEN (auc.token)::character varying
            WHEN (LEAST(auc.price_usd, sale.price_usd) = sale.price_usd) THEN (sale.token)::character varying
            ELSE NULL::character varying
        END AS floor_price_token
   FROM ((((nft n
     LEFT JOIN LATERAL ( SELECT ag.price AS last_price
           FROM ( SELECT (s.price * tup.usd_price) AS price,
                    s.created
                   FROM (nft_direct_sell s
                     JOIN token_usd_prices tup ON (((tup.token)::text = (s.price_token)::text)))
                  WHERE ((s.state = 'filled'::direct_sell_state) AND ((s.nft)::text = (n.address)::text))
                UNION ALL
                 SELECT (s.price * tup.usd_price),
                    s.created
                   FROM (nft_direct_buy s
                     JOIN token_usd_prices tup ON (((tup.token)::text = (s.price_token)::text)))
                  WHERE ((s.state = 'filled'::direct_buy_state) AND ((s.nft)::text = (n.address)::text))
                UNION ALL
                 SELECT (s.max_bid * tup.usd_price),
                    s.created_at
                   FROM (nft_auction s
                     JOIN token_usd_prices tup ON (((tup.token)::text = (s.price_token)::text)))
                  WHERE ((s.status = 'completed'::auction_status) AND ((s.nft)::text = (n.address)::text))) ag
          ORDER BY ag.created DESC
         LIMIT 1) last_deal ON (true))
     LEFT JOIN LATERAL ( SELECT a.address AS auction,
            a.status AS "auction_status: _",
            (a.min_bid * tup.usd_price) AS price_usd,
            tup.token,
            a.min_bid
           FROM ((nft_auction a
             JOIN events_whitelist wl ON (((wl.address)::text = (a.address)::text)))
             LEFT JOIN token_usd_prices tup ON ((((tup.token)::text = (a.price_token)::text) AND (a.status = 'active'::auction_status))))
          WHERE (((a.nft)::text = (n.address)::text) AND (a.status = ANY (ARRAY['active'::auction_status, 'expired'::auction_status])))
         LIMIT 1) auc ON (true))
     LEFT JOIN nft_metadata m ON (((m.nft)::text = (n.address)::text)))
     LEFT JOIN LATERAL ( SELECT s.address AS forsale,
            s.state AS "forsale_status: _",
            (s.price * tup.usd_price) AS price_usd,
            s.price,
            tup.token
           FROM ((nft_direct_sell s
             JOIN events_whitelist wl ON (((wl.address)::text = (s.address)::text)))
             LEFT JOIN token_usd_prices tup ON ((((tup.token)::text = (s.price_token)::text) AND (s.state = 'active'::direct_sell_state))))
          WHERE (((s.nft)::text = (n.address)::text) AND (s.state = ANY (ARRAY['active'::direct_sell_state, 'expired'::direct_sell_state])))
         LIMIT 1) sale ON (true))
  WHERE (NOT n.burned);
