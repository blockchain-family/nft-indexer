CREATE OR REPLACE FUNCTION trigger_insert_collections_details()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO nft_collection_details (address, name, description, verified, owner, wallpaper, logo)
    VALUES (NEW.address, NEW.name, NEW.description, NEW.verified, NEW.owner, NEW.wallpaper, NEW.logo)
    ON CONFLICT (address) DO NOTHING;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER after_insert_nft_collection
AFTER INSERT ON nft_collection
FOR EACH ROW
EXECUTE FUNCTION trigger_insert_collections_details();