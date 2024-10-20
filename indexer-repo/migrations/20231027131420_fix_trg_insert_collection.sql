CREATE OR REPLACE FUNCTION trigger_insert_collections_details()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO nft_collection_details (address, name, description, verified, owner, wallpaper, logo, first_mint)
    VALUES (NEW.address, NEW.name, NEW.description, NEW.verified, NEW.owner, NEW.wallpaper, NEW.logo, NEW.first_mint)
    ON CONFLICT (address) DO NOTHING;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS after_insert_nft_collection ON nft_collection;

CREATE TRIGGER after_insert_nft_collection
AFTER INSERT ON nft_collection
FOR EACH ROW
EXECUTE FUNCTION trigger_insert_collections_details();