drop index nft_events_short_created_at_index;
create index nft_events_created_at_created_lt_index
	on nft_events (created_at desc, created_lt desc);


