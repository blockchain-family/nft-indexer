create table nft_verified_extended
(
	address t_address not null
		constraint nft_verified_extended_pkey
			primary key,
	collection t_address not null,
	name varchar(255),
	version timestamp not null default now(),
	updated timestamp not null,
	floor_price_auc_usd numeric,
	floor_price_sell_usd numeric
);


create index nft_verified_extended_collection_index1
	on nft_verified_extended (collection);

create index nft_verified_extended_name_index
	on nft_verified_extended (name collate numeric);

CREATE INDEX nft_verified_name_gin_idx
ON nft_verified_extended
USING gin (name gin_trgm_ops);

CREATE INDEX nft_verified_lower_address_gin_idx
ON nft_verified_extended (lower(address));

create index nft_verified_extended_version_index
	on nft_verified_extended (version);

create index nft_verified_extended_updated_index
	on nft_verified_extended (updated);

create index nft_verified_extended_price1_index
	on nft_verified_extended (floor_price_auc_usd);

create index nft_verified_extended_price2_index
	on nft_verified_extended (floor_price_sell_usd);

create index nft_verified_extended_price3_index
	on nft_verified_extended (least(floor_price_auc_usd, floor_price_sell_usd));

alter table nft add column metadata_updated_at bigint;
CREATE INDEX nft_metadata_updated_index ON nft USING btree (metadata_updated_at);

CREATE OR REPLACE FUNCTION refresh_nft_verified_extended(p_full bool)
    RETURNS void
    LANGUAGE plpgsql AS
$$
DECLARE
BEGIN

    if p_full then
        begin
            delete from nft_verified_extended e;
            insert into nft_verified_extended(address,
                                              collection,
                                              name,
                                              updated,
                                              floor_price_sell_usd,
                                              floor_price_auc_usd)
            SELECT n.address,
                   n.collection,
                   n.name,
                   n.updated,
                   MIN(CASE WHEN ow.address IS NOT NULL THEN s.price * tup.usd_price END)     AS min_sell_price,
                   MIN(CASE WHEN ow2.address IS NOT NULL THEN a.min_bid * tup2.usd_price END) AS min_auc_price
            FROM nft n
                     JOIN nft_collection nc ON nc.address = n.collection AND nc.verified
                     LEFT JOIN nft_auction a ON a.nft = n.address AND a.status = 'active' AND
                                                (a.finished_at = to_timestamp(0) OR a.finished_at > NOW())
                     LEFT JOIN offers_whitelist ow2 ON ow2.address = a.address
                     LEFT JOIN token_usd_prices tup2 ON tup2.token = a.price_token
                     LEFT JOIN nft_direct_sell s ON s.nft = n.address AND s.state = 'active' AND
                                                    (s.expired_at = to_timestamp(0) OR s.expired_at > NOW())
                     LEFT JOIN offers_whitelist ow ON ow.address = s.address
                     LEFT JOIN token_usd_prices tup ON tup.token = s.price_token
            WHERE NOT n.burned
            GROUP BY n.address;
        end;
    else
        ANALYZE nft;
        ANALYZE nft_collection;

        CREATE TEMP TABLE address_time_temp ON COMMIT DROP AS
        SELECT address, event_time
        FROM (
                 SELECT n.address, nc.verified_updated_at::timestamptz as event_time
                 FROM nft n
                          JOIN nft_collection nc ON n.collection = nc.address
                 WHERE nc.verified_updated_at > NOW() - INTERVAL '15 minutes'

                 UNION ALL

                 SELECT m.address, to_timestamp(m.updated_at) as event_time
                 FROM meta_handled_addresses m
                 WHERE m.updated_at > extract(epoch FROM (NOW() - INTERVAL '15 minutes'))::bigint
                   AND NOT m.failed

                 union all
                 select n.address, to_timestamp(n.metadata_updated_at)
                 from nft n
                 where n.metadata_updated_at > extract(epoch FROM (NOW() - INTERVAL '15 minutes'))::bigint
                 union all

                 select address, now() - interval '15 minutes'
                 from nft_verified_extended
                 where version < now() - interval '12 hours'
                 limit 10000
             ) AS combined_address_times;


        set enable_seqscan = off;
        insert into address_time_temp(address, event_time)
        SELECT ne.nft as address, ne.local_created_at as event_time
        FROM nft_events ne
        WHERE ne.local_created_at > (NOW() - INTERVAL '15 minutes');
        set enable_seqscan = on;

        BEGIN
            DELETE
            FROM nft_verified_extended nve
                USING address_time_temp att
            WHERE nve.address = att.address
              AND nve.version < att.event_time;

            insert into nft_verified_extended(address,
                                              collection,
                                              name,
                                              updated,
                                              floor_price_sell_usd,
                                              floor_price_auc_usd)
            select n.address,
                   n.collection,
                   n.name,
                   n.updated,
                   min(
                           case
                               when ow.address is not null then
                                   s.price * tup.usd_price end) min_sell_price,
                   min(
                           case
                               when ow2.address is not null then
                                   a.min_bid * tup2.usd_price end
                       )                                        min_auc_price
            from nft n
                     join nft_collection nc
                          on nc.address = n.collection and nc.verified
                     left join nft_auction a
                               on a.nft = n.address
                                   and a.status = 'active'::auction_status
                                   and (a.finished_at = to_timestamp(0) or a.finished_at > now()::timestamp)

                     left join offers_whitelist ow2 on ow2.address = a.address
                     left join token_usd_prices tup2 on tup2.token = a.price_token


                     left join nft_direct_sell s
                               on s.nft = n.address
                                   and s.state = 'active'::direct_sell_state
                                   and (s.expired_at = to_timestamp(0) or s.expired_at > now())

                     left join offers_whitelist ow on ow.address = s.address
                     left join token_usd_prices tup on tup.token = s.price_token

            WHERE n.address IN (SELECT address FROM address_time_temp)
              AND NOT n.burned
            group by n.address, n.collection, n.name
            on conflict do nothing;

        EXCEPTION
            WHEN OTHERS THEN
                RAISE NOTICE 'An error occurred: %', SQLERRM;
        END;
        DROP TABLE IF EXISTS address_time_temp;
    end if;


END
$$;

select refresh_nft_verified_extended(true);