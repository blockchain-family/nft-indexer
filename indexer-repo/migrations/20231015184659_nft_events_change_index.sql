create index nft_events_collection_created_at_created_lt_index
    on nft_events (collection asc, created_at desc, created_lt desc);