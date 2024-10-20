create materialized view nft_events_verified_mv as
WITH events_whitelist AS (
    SELECT of.address
    FROM deployed_offers of
    UNION
    SELECT address
    FROM roots
)
select n.*
from nft_events n
         join nft_collection nc on n.collection = nc.address and nc.verified
where (
          (n.computed_event_kind IN ('mint'::event_kind, 'transfer'::event_kind) OR
           EXISTS(SELECT 1 FROM events_whitelist ew WHERE ew.address = n.address))
          );

create unique index nft_events_verified_mv_id_index
	on nft_events_verified_mv (id);

create index nft_events_verified_mv_collection_computed_event_kind_index
	on nft_events_verified_mv (collection, computed_event_kind);

create index nft_events_verified_mv_computed_event_kind_index
	on nft_events_verified_mv (computed_event_kind);

create index nft_events_verified_mv_idx_subject_owner
	on nft_events_verified_mv (((args -> 'value0'::text) ->> 'subject_owner'::text));

create index nft_events_verified_mv_idx_creator
	on nft_events_verified_mv (((args -> 'value2'::text) ->> 'creator'::text));

create index nft_events_verified_mv_idx_buyer
	on nft_events_verified_mv ((args ->> 'buyer'::text));

create index nft_events_verified_mv_idx_seller
	on nft_events_verified_mv ((args ->> 'seller'::text));

create index nft_events_verified_mv_idx_old_owner
	on nft_events_verified_mv ((args ->> 'old_owner'::text));

create index nft_events_verified_mv_idx_new_owner
	on nft_events_verified_mv ((args ->> 'new_owner'::text));

create index nft_events_verified_mv_idx_event_type_and_auction
	on nft_events_verified_mv (event_type, (args ->> 'auction'::text));

create index nft_events_verified_mv_idx_subject_owner2
	on nft_events_verified_mv (((args -> 'value2'::text) ->> 'subject_owner'::text));

create index nft_events_verified_mv_created_at_created_lt_index
	on nft_events_verified_mv (created_at desc, created_lt desc);

create index nft_events_verified_mv_collection_created_at_created_lt_index
	on nft_events_verified_mv (collection asc, created_at desc, created_lt desc);


CREATE OR REPLACE FUNCTION get_events(
    p_owner t_address,
    p_event_kind event_kind[],
    p_nft t_address,
    p_collections t_address[],
    p_limit int,
    p_offset int,
    p_with_count bool,
    p_verified bool
)
RETURNS TABLE
(
    computed_event_kind event_kind,
    id                  bigint,
    event_address       t_address,
    nft                 t_address,
    created_at          bigint,
    created_lt          bigint,
    event_type          event_type,
    event_cat           event_category,
    args                jsonb,
    f                   int,
    t                   int,
    total_rows          bigint,
    new_owner           text,
    old_owner           text
)
LANGUAGE plpgsql
AS
$$
BEGIN
    IF p_owner IS NOT NULL THEN
        EXECUTE 'SET enable_bitmapscan = on';
        EXECUTE 'SET enable_indexscan = off';
        EXECUTE 'SET enable_sort = on';
    elsif ARRAY_LENGTH(p_collections::t_address[], 1) = 1 is not null and p_nft is null and p_event_kind is null then
        EXECUTE 'SET enable_indexscan = on';
        EXECUTE 'SET enable_sort = off';
    else
        EXECUTE 'SET enable_bitmapscan = on';
        EXECUTE 'SET enable_indexscan = on';
        EXECUTE 'SET enable_sort = on';
    END IF;


    if p_verified and p_nft is null and p_owner is null and (p_collections is null or p_collections = '{}' or ARRAY_LENGTH(p_collections::t_address[], 1) > 1 or  ARRAY_LENGTH(p_event_kind::event_kind[], 1) > 0) then
        RAISE NOTICE 'Value: %', p_collections;

     RETURN QUERY
        SELECT
            ne.computed_event_kind AS computed_event_kind,
            ne.id,
            ne.address AS event_address,
            ne.nft,
            ne.created_at,
            ne.created_lt,
            ne.event_type,
            ne.event_cat,
            ne.args,
            (ne.args ->> 'from')::int AS f,
            (ne.args ->> 'to')::int AS t,
            CASE
                WHEN p_with_count THEN COUNT(1) OVER ()
                ELSE 0
            END AS total_rows,
            CASE ne.computed_event_kind
                WHEN 'sell_purchased'::event_kind THEN ne.args ->> 'new_owner'
            END AS new_owner,
            CASE ne.computed_event_kind
                WHEN 'offer_filled'::event_kind THEN ne.args ->> 'old_owner'
            END AS old_owner
        FROM
            nft_events_verified_mv ne
        WHERE
            (p_owner IN (
                ne.args -> 'value0' ->> 'subject_owner',
                ne.args -> 'value2' ->> 'subject_owner',
                ne.args -> 'value2' ->> 'creator',
                ne.args ->> 'buyer',
                ne.args ->> 'seller',
                ne.args ->> 'old_owner',
                ne.args ->> 'new_owner'
            ) OR p_owner IS NULL)

        and (
            CASE
                WHEN ARRAY_LENGTH(p_collections::text[], 1) = 1 THEN ne.collection = p_collections[1]
                ELSE ne.collection = ANY (p_collections) or p_collections is null or p_collections = '{}'
            END
        )
        AND ne.computed_event_kind IS NOT NULL
        AND (ne.computed_event_kind = ANY (p_event_kind) OR (p_event_kind) = '{}' or p_event_kind is null)
        order by created_at desc, created_lt desc
        LIMIT p_limit OFFSET p_offset;
    else
         RETURN QUERY
    WITH events_whitelist AS (
        SELECT of.address
        FROM deployed_offers of
        UNION
        SELECT address
        FROM roots
    )
        SELECT
            ne.computed_event_kind AS computed_event_kind,
            ne.id,
            ne.address AS event_address,
            ne.nft,
            ne.created_at,
            ne.created_lt,
            ne.event_type,
            ne.event_cat,
            ne.args,
            (ne.args ->> 'from')::int AS f,
            (ne.args ->> 'to')::int AS t,
            CASE
                WHEN p_with_count THEN COUNT(1) OVER ()
                ELSE 0
            END AS total_rows,
            CASE ne.computed_event_kind
                WHEN 'sell_purchased'::event_kind THEN ne.args ->> 'new_owner'
            END AS new_owner,
            CASE ne.computed_event_kind
                WHEN 'offer_filled'::event_kind THEN ne.args ->> 'old_owner'
            END AS old_owner
        FROM
            nft_events ne
        WHERE
            (p_owner IN (
                ne.args -> 'value0' ->> 'subject_owner',
                ne.args -> 'value2' ->> 'subject_owner',
                ne.args -> 'value2' ->> 'creator',
                ne.args ->> 'buyer',
                ne.args ->> 'seller',
                ne.args ->> 'old_owner',
                ne.args ->> 'new_owner'
            ) OR p_owner IS NULL)
        AND (
            (ne.computed_event_kind IN ('mint'::event_kind, 'transfer'::event_kind) OR
            EXISTS(SELECT 1 FROM events_whitelist ew WHERE ew.address = ne.address))
        )
        AND (ne.nft = p_nft OR p_nft IS NULL)
        and (
            CASE
                WHEN ARRAY_LENGTH(p_collections::text[], 1) = 1 THEN ne.collection = p_collections[1]
                ELSE ne.collection = ANY (p_collections) or p_collections is null or p_collections = '{}'
            END
        )
        AND ne.computed_event_kind IS NOT NULL
        AND (ne.computed_event_kind = ANY (p_event_kind) OR (p_event_kind) = '{}' or p_event_kind is null)
        order by created_at desc, created_lt desc
        LIMIT p_limit OFFSET p_offset;
    end if;
END;
$$ VOLATILE;