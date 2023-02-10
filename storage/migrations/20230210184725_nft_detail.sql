create or replace view nft_details
            (address, collection, owner, manager, name, description, burned, updated, tx_lt, meta, auction,
             "auction_status: _", forsale, "forsale_status: _", best_offer, floor_price_usd, deal_price_usd,
             floor_price, floor_price_token)
as
SELECT n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name,
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
           END                              AS floor_price_token
FROM nft n
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