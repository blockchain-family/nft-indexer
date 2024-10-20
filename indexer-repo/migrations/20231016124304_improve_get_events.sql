drop function get_events(t_address, event_kind[], t_address, t_address[], integer, integer, boolean, boolean);

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
    elsif p_event_kind is not null and p_nft is null and p_collections is null then
        EXECUTE 'SET enable_indexscan = on';
        EXECUTE 'SET enable_sort = off';
    elsif ARRAY_LENGTH(p_collections::t_address[], 1) = 1 is not null and p_nft is null and p_event_kind is null then
        EXECUTE 'SET enable_indexscan = on';
        EXECUTE 'SET enable_sort = off';
    else
        EXECUTE 'SET enable_bitmapscan = on';
        EXECUTE 'SET enable_indexscan = on';
        EXECUTE 'SET enable_sort = on';
    END IF;


    if p_collections = '{}' then

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
    JOIN nft_collection nc ON nc.address = ne.collection AND (nc.verified = true or not p_verified)

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
    AND ne.computed_event_kind IS NOT NULL
    AND (ne.computed_event_kind = ANY (p_event_kind) OR (p_event_kind) = '{}')
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
                ELSE ne.collection = ANY (p_collections)
            END
        )
        AND ne.computed_event_kind IS NOT NULL
        AND (ne.computed_event_kind = ANY (p_event_kind) OR (p_event_kind) = '{}')
        order by created_at desc, created_lt desc
        LIMIT p_limit OFFSET p_offset;
    end if;
END;
$$ VOLATILE;