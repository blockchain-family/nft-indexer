ALTER TYPE event_type ADD VALUE 'market_fee_default_changed';
ALTER TYPE event_type ADD VALUE 'market_fee_changed';
ALTER TYPE event_type ADD VALUE 'add_collection_rules';
ALTER TYPE event_type ADD VALUE 'remove_collection_rules';

alter table nft_collection
	add fee_numerator int;

alter table nft_collection
	add fee_denominator int;

