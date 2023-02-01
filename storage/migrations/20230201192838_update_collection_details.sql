create or replace view nft_collection_details
            (address, owner, name, description, created, updated, wallpaper, logo, total_price, max_price, owners_count,
             verified, nft_count, floor_price_usd, total_volume_usd, attributes)
as
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
       nft_counter.cnt                        AS nft_count,
       least(direct_sell.usd, auction.usd) AS floor_price_usd,
       COALESCE(total_volume.usd, 0::numeric) AS total_volume_usd,
       attr.list                              AS attributes
FROM nft_collection c
         LEFT JOIN LATERAL ( SELECT json_agg(res.json) AS list
                             FROM (SELECT json_build_object('traitType', na.trait_type, 'traitValues',
                                                            json_agg(DISTINCT TRIM(BOTH FROM na.value #>> '{}'::text[]))) AS json
                                   FROM nft_attributes na
                                   WHERE na.collection::text = c.address::text
                                   GROUP BY na.trait_type, na.collection) res) attr ON true
         LEFT JOIN LATERAL ( SELECT min(na.min_bid * tup.usd_price) AS usd
                             FROM nft_auction na
                                      JOIN nft n ON n.address::text = na.nft::text AND
                                                    n.collection::text = c.address::text AND NOT n.burned
                                      LEFT JOIN token_usd_prices tup ON tup.token::text = na.price_token::text
                             WHERE na.status = 'active'::auction_status) auction ON true
         LEFT JOIN LATERAL ( SELECT min(ds.price * tup.usd_price) AS usd
                             FROM nft_direct_sell ds
                                      JOIN nft n ON n.address::text = ds.nft::text AND
                                                    n.collection::text = c.address::text AND NOT n.burned
                                      LEFT JOIN token_usd_prices tup ON tup.token::text = ds.price_token::text
                             WHERE ds.state = 'active'::direct_sell_state) direct_sell ON true
         LEFT JOIN LATERAL ( SELECT count(1) AS cnt
                             FROM nft n
                             WHERE n.burned = false
                               AND n.collection::text = c.address::text) nft_counter ON true
         LEFT JOIN LATERAL ( SELECT sum(ag.price_usd) AS usd
                             FROM (SELECT COALESCE(tup.usd_price * ndb.price, 0::numeric) AS price_usd
                                   FROM nft_direct_buy ndb
                                            JOIN nft n ON n.address::text = ndb.nft::text AND
                                                          n.collection::text = c.address::text AND NOT n.burned
                                            JOIN token_usd_prices tup ON tup.token::text = ndb.price_token::text
                                   WHERE ndb.state = 'filled'::direct_buy_state
                                   UNION ALL
                                   SELECT COALESCE(tup.usd_price * nds.price, 0::numeric) AS "coalesce"
                                   FROM nft_direct_sell nds
                                            JOIN nft n ON n.address::text = nds.nft::text AND
                                                          n.collection::text = c.address::text AND NOT n.burned
                                            JOIN token_usd_prices tup ON tup.token::text = nds.price_token::text
                                   WHERE nds.state = 'filled'::direct_sell_state
                                   UNION ALL
                                   SELECT COALESCE(tup.usd_price * na.max_bid, 0::numeric) AS "coalesce"
                                   FROM nft_auction na
                                            JOIN nft n ON na.nft::text = n.address::text AND NOT n.burned AND
                                                          n.collection::text = c.address::text
                                            JOIN token_usd_prices tup ON tup.token::text = na.price_token::text
                                   WHERE na.status = 'completed'::auction_status) ag) total_volume ON true;

alter table nft_collection_details
    owner to indexator;

