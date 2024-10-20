alter table nft_direct_sell add royalty_numerator int;

alter table nft_direct_sell add royalty_denominator int;

alter table nft_direct_buy add royalty_numerator int;

alter table nft_direct_buy add royalty_denominator int;

alter table nft_auction add royalty_numerator int;

alter table nft_auction add royalty_denominator int;
