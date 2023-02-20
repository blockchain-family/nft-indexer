create index nft_events_nft_index
	on nft_events (nft);

create index nft_events_created_at_index
	on nft_events (created_at desc);

