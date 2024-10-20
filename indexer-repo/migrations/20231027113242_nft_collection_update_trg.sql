CREATE OR REPLACE FUNCTION update_nft_collection_details_on_change()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if any of the specified columns have changed
    IF OLD.name IS DISTINCT FROM NEW.name OR
       OLD.description IS DISTINCT FROM NEW.description OR
       OLD.verified IS DISTINCT FROM NEW.verified OR
       OLD.owner IS DISTINCT FROM NEW.owner OR
       OLD.wallpaper IS DISTINCT FROM NEW.wallpaper OR
       OLD.logo IS DISTINCT FROM NEW.logo THEN

        BEGIN
           WITH locked_rows AS (
            SELECT ncd.*
            FROM nft_collection_details ncd
            JOIN nft_collection_custom ncc ON ncc.address = NEW.address
            WHERE ncd.address = NEW.address
            FOR UPDATE OF ncd NOWAIT
        )
        UPDATE nft_collection_details ncd
        SET name = COALESCE(ncc.name, NEW.name),
            description = COALESCE(ncc.description, NEW.description),
            verified = NEW.verified,
            owner = NEW.owner,
            wallpaper = COALESCE(ncc.wallpaper, NEW.wallpaper),
            logo = COALESCE(ncc.logo, NEW.logo)
        FROM nft_collection_custom ncc, locked_rows
        WHERE ncc.address = NEW.address
        AND ncd.address = NEW.address;

        EXCEPTION
            WHEN LOCK_NOT_AVAILABLE THEN
                RAISE NOTICE 'Could not update nft_collection_details for address % due to lock contention.', NEW.address;
        END;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER tr_update_nft_collection_details
AFTER UPDATE OF name, description, verified, owner, wallpaper, logo ON nft_collection
FOR EACH ROW EXECUTE FUNCTION update_nft_collection_details_on_change();