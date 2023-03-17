alter table nft_events
	add message_hash numeric;

create unique index nft_events_message_hash_uindex
	on nft_events (message_hash);
