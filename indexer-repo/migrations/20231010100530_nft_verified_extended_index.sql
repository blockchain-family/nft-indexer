create index nft_verified_extended_price5_index
	on nft_verified_extended (LEAST(floor_price_auc_usd, floor_price_sell_usd), name asc, address asc);

create index nft_verified_extended_price6_index
	on nft_verified_extended (COALESCE(LEAST(floor_price_auc_usd, floor_price_sell_usd), 0::numeric) desc, name asc, address asc);

drop index nft_verified_extended_price3_index;

drop index nft_verified_extended_price4_index;
