alter table nft_verified_extended
	add price_token t_address;
	
create index nft_verified_extended_price_token1_index
	on nft_verified_extended (price_token, LEAST(floor_price_auc_usd, floor_price_sell_usd), name collate numeric asc, address asc);

create index nft_verified_extended_price_token2_index
	on nft_verified_extended (price_token, COALESCE(LEAST(floor_price_auc_usd, floor_price_sell_usd), 0::numeric) desc, name collate numeric asc, address asc);