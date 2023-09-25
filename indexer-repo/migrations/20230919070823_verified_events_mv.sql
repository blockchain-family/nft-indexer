CREATE INDEX IF NOT EXISTS nft_direct_sell_expired_at_index
ON nft_direct_sell (expired_at);

DROP MATERIALIZED VIEW IF EXISTS nft_verified_mv;
CREATE MATERIALIZED VIEW nft_verified_mv AS
SELECT n.*
FROM nft n
JOIN nft_collection nc ON n.collection = nc.address AND nc.verified;

create index on nft_verified_mv (name asc);
create index on nft_verified_mv (address asc);
create index on nft_verified_mv (collection asc);

CREATE INDEX IF NOT EXISTS idx_event_type_and_auction
ON nft_events (event_type, (args ->> 'auction'));