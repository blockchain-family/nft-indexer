create index IF NOT EXISTS nft_collection_owner_index on nft (collection, owner);
create index nft_verified_mv_collection_owner_index on nft_verified_mv (collection, owner);

DROP TRIGGER IF EXISTS update_event_kind_trigger ON nft_events;

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

    IF NEW.computed_event_kind is not null and NEW.nft IS NOT NULL AND (NEW.collection IS NULL or NEW.collection = '0:0000000000000000000000000000000000000000000000000000000000000000') THEN
        NEW.collection = (SELECT collection FROM nft n WHERE n.address = NEW.nft);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER update_event_kind_trigger
BEFORE INSERT ON nft_events
FOR EACH ROW
EXECUTE FUNCTION update_computed_event_kind();
