alter table nft_auction
	add fee_numerator int;

alter table nft_auction
	add fee_denominator int;

alter table nft_direct_sell
	add fee_numerator int;

alter table nft_direct_sell
	add fee_denominator int;

alter table nft_direct_buy
	add fee_numerator int;

alter table nft_direct_buy
	add fee_denominator int;

