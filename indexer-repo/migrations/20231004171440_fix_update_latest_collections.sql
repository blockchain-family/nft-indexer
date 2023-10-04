drop procedure update_latest_collections;

create procedure update_latest_collections("interval" interval)
    language plpgsql as
$$
declare
    collections_for_update t_address[];
begin
    collections_for_update := ( select array_agg(distinct ne.collection)
                                from nft_events ne
                                where ne.local_created_at > now() - "interval"
                                  and ne.collection is not null
                                  and ne.computed_event_kind in ('mint'::event_kind, 'transfer'::event_kind) );

    call update_collections_details(collections_for_update, true);
end;
$$;

