create or replace view nft_details as
select n.address,
       n.collection,
       n.owner,
       n.manager,
       n.name::text                          as name,
       n.description,
       n.burned,
       n.updated,
       n.owner_update_lt                     as tx_lt,
       m.meta,
       auc.auction,
       auc."auction_status: _",
       sale.forsale,
       sale."forsale_status: _",
       ( select distinct on (s.address) first_value(s.address) over w
         from nft_direct_buy_usd s
         where state = 'active'
           and nft = n.address
         window w as (partition by nft order by usd_price desc)
         limit 1 )                           as best_offer,
       least(auc.price_usd, sale.price_usd)  as floor_price_usd,
       last_deal.last_price                  as deal_price_usd,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.min_bid
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.price
            else null::numeric end           as floor_price,
       case when least(auc.price_usd, sale.price_usd) = auc.price_usd then auc.token::character varying
            when least(auc.price_usd, sale.price_usd) = sale.price_usd then sale.token::character varying
            else null::character varying end as floor_price_token,
       n.id::text                            as nft_id
from nft n
         left join lateral ( select nph.price * tup.usd_price as last_price
                             from nft_price_history nph
                                      join offers_whitelist ow on ow.address = nph.source
                                      left join token_usd_prices tup on tup.token = nph.price_token
                             where nph.nft = n.address
                             order by nph.ts desc
                             limit 1 ) last_deal on true
         left join lateral ( select a.address                 as auction,
                                    case when a.status = 'active' and to_timestamp(0) < a.finished_at and
                                              a.finished_at < now() then 'expired'
                                         else a.status end    as "auction_status: _",
                                    a.min_bid * tup.usd_price as price_usd,
                                    tup.token,
                                    a.min_bid
                             from nft_auction a
                                      join offers_whitelist ow on ow.address = a.address
                                      left join token_usd_prices tup on tup.token = a.price_token
                             where a.nft = n.address
                               and a.status in ('active', 'expired')
                             limit 1 ) auc on true
         left join nft_metadata m on m.nft = n.address
         left join lateral ( select s.address                                                      as forsale,
                                    case when s.state = 'active' and to_timestamp(0) < s.expired_at and
                                              s.expired_at < now() then 'expired'
                                         else s.state end                                          as "forsale_status: _",
                                    s.price * tup.usd_price                                        as price_usd,
                                    s.price,
                                    tup.token
                             from nft_direct_sell s
                                      join offers_whitelist ow on ow.address = s.address
                                      left join token_usd_prices tup on tup.token = s.price_token
                             where s.nft = n.address
                               and s.state in ('active', 'expired')
                             limit 1 ) sale on true
where not n.burned;