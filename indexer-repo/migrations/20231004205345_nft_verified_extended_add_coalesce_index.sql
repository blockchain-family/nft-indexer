create index nft_verified_extended_price4_index
	on nft_verified_extended (coalesce(least(floor_price_auc_usd, floor_price_sell_usd),0) desc);
