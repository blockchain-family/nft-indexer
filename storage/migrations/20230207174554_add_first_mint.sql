alter table nft_collection
	add first_mint timestamp;

create index nft_collection_first_mint_index
	on nft_collection (first_mint);