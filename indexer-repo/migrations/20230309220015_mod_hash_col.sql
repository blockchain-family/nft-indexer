alter table nft_events alter column message_hash type text using message_hash::varchar(256);
