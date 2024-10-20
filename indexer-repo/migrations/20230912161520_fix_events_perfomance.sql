create type event_kind as enum ('mint',
    'transfer',
    'auction_active',
    'auction_bid_placed',
    'auction_canceled',
    'auction_complete',
    'offer_active',
    'sell_active',
    'offer_filled',
    'sell_purchased',
    'sell_canceled',
    'offer_canceled');

alter table nft_events
	add computed_event_kind event_kind;

drop index ix_nft_events_address;
drop index ix_nft_events_cat;
drop index ix_nft_events_type;
drop index nft_events_args_auc_idx;
drop index nft_events_args_idx1;
drop index nft_events_created_at_index;
drop index nft_events_created_lt_index;
drop index nft_events_nft_index;


create index nft_events_collection_computed_event_kind_index
	on nft_events (collection, computed_event_kind);

create index nft_events_computed_event_kind_index
	on nft_events (computed_event_kind);

create index nft_events_short_created_at_index
	on nft_events (created_at desc);

create index nft_events_short_address_index
	on nft_events (address);

create index nft_events_short_address_created_at_index
	on nft_events (address asc, created_at desc);

create index idx_subject_owner
	on nft_events (((args -> 'value0'::text) ->> 'subject_owner'::text));

create index idx_creator
	on nft_events (((args -> 'value2'::text) ->> 'creator'::text));

create index idx_buyer
	on nft_events ((args ->> 'buyer'::text));

create index idx_seller
	on nft_events ((args ->> 'seller'::text));

create index idx_old_owner
	on nft_events ((args ->> 'old_owner'::text));

create index idx_new_owner
	on nft_events ((args ->> 'new_owner'::text));


CREATE OR REPLACE FUNCTION update_computed_event_kind()
RETURNS TRIGGER AS $$
BEGIN
    NEW.computed_event_kind = CASE
        WHEN NEW.event_type = 'nft_created' THEN 'mint'::event_kind
        WHEN NEW.event_type = 'nft_owner_changed' THEN 'transfer'::event_kind
        WHEN NEW.event_type = 'auction_active' THEN 'auction_active'::event_kind
        WHEN NEW.event_type = 'auction_bid_placed' THEN 'auction_bid_placed'::event_kind
        WHEN NEW.event_type = 'auction_cancelled' THEN 'auction_canceled'::event_kind
        WHEN NEW.event_type = 'auction_complete' THEN 'auction_complete'::event_kind
        WHEN NEW.event_cat = 'direct_buy' AND (NEW.args ->> 'from')::int = 0 AND (NEW.args ->> 'to')::int = 2 THEN 'offer_active'::event_kind
        WHEN NEW.event_cat = 'direct_sell' AND (NEW.args ->> 'from')::int = 0 AND (NEW.args ->> 'to')::int = 2 THEN 'sell_active'::event_kind
        WHEN NEW.event_cat = 'direct_buy' AND (NEW.args ->> 'from')::int = 2 AND (NEW.args ->> 'to')::int = 3 THEN 'offer_filled'::event_kind
        WHEN NEW.event_cat = 'direct_sell' AND (NEW.args ->> 'from')::int = 2 AND (NEW.args ->> 'to')::int = 3 THEN 'sell_purchased'::event_kind
        WHEN NEW.event_cat = 'direct_sell' AND (NEW.args ->> 'from')::int = 2 AND (NEW.args ->> 'to')::int = 4 THEN 'sell_canceled'::event_kind
        WHEN NEW.event_cat = 'direct_buy' AND (NEW.args ->> 'from')::int = 2 AND (NEW.args ->> 'to')::int = 4 THEN 'offer_canceled'::event_kind
        ELSE NEW.computed_event_kind
    END;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_event_kind_trigger
BEFORE INSERT ON nft_events
FOR EACH ROW
EXECUTE FUNCTION update_computed_event_kind();

