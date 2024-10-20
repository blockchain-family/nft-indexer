create index nft_collection_verified_index
	on nft_collection (verified);

create index nft_name_index
	on nft (name);

create index nft_burned_index
	on nft (burned);

CREATE INDEX ix_nft_burned_collection ON nft (burned, collection);