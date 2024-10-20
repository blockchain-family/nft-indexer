create domain t_url as varchar(4096);

ALTER TABLE nft_collection_custom ALTER COLUMN wallpaper TYPE t_url USING wallpaper::t_url;
ALTER TABLE nft_collection_custom ALTER COLUMN logo TYPE t_url USING logo::t_url;

DROP TRIGGER IF EXISTS after_insert_nft_collection ON nft_collection;
DROP TRIGGER IF EXISTS trigger_update_verified_updated_at ON nft_collection;
DROP TRIGGER IF EXISTS tr_update_nft_collection_details ON nft_collection;

ALTER TABLE nft_collection ALTER COLUMN wallpaper TYPE t_url USING wallpaper::t_url;
ALTER TABLE nft_collection ALTER COLUMN logo TYPE t_url USING logo::t_url;

CREATE TRIGGER after_insert_nft_collection
AFTER INSERT ON nft_collection
FOR EACH ROW
EXECUTE FUNCTION trigger_insert_collections_details();

CREATE TRIGGER tr_update_nft_collection_details
AFTER UPDATE OF name, description, verified, owner, wallpaper, logo ON nft_collection
FOR EACH ROW EXECUTE FUNCTION update_nft_collection_details_on_change();

CREATE TRIGGER trigger_update_verified_updated_at
BEFORE UPDATE OF verified
ON nft_collection
FOR EACH ROW
EXECUTE FUNCTION update_verified_updated_at();

ALTER TABLE profile ALTER COLUMN image TYPE t_url USING image::t_url;
ALTER TABLE profile ALTER COLUMN site TYPE t_url USING site::t_url;
ALTER TABLE nft_collection_details ALTER COLUMN wallpaper TYPE t_url USING wallpaper::t_url;
ALTER TABLE nft_collection_details ALTER COLUMN logo TYPE t_url USING logo::t_url;

drop type t_uri;