alter table nft_collection
 add verified_updated_at timestamp;

create index nft_collection_verified_changed_at_index
 on nft_collection (verified_updated_at);

alter table nft_events
 add local_created_at timestamp default now() not null;

create index nft_events_local_created_at_index
 on nft_events (local_created_at desc);


CREATE OR REPLACE FUNCTION update_verified_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.verified IS DISTINCT FROM NEW.verified THEN
        NEW.verified_updated_at := NOW();
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_verified_updated_at
BEFORE UPDATE OF verified
ON nft_collection
FOR EACH ROW
EXECUTE FUNCTION update_verified_updated_at();