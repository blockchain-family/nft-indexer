drop index nft_verified_extended_price5_index;

drop index nft_verified_extended_price6_index;

create index nft_verified_extended_price5_index
	on nft_verified_extended (LEAST(floor_price_auc_usd, floor_price_sell_usd), name collate numeric asc, address asc);

create index nft_verified_extended_price6_index
	on nft_verified_extended (COALESCE(LEAST(floor_price_auc_usd, floor_price_sell_usd), 0::numeric) desc, name collate numeric asc, address asc);


CREATE INDEX idx_nft_attributes_trait_type ON nft_attributes (LOWER(trait_type));
CREATE INDEX idx_nft_attributes_value ON nft_attributes ((LOWER(TRIM(value #>> '{}'))));
drop index ix_nft_attributes_nft;

