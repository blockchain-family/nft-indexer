create table deployed_offers (
  address t_address primary key,
  root t_address not null
);

alter table meta_handled_addresses alter column failed set not null;

alter table nft alter column collection set not null;
alter table nft alter column owner set not null;
alter table nft alter column manager set not null;
alter table nft alter column burned set not null;
alter table nft alter column updated set not null;
alter table nft alter column owner_update_lt set not null;
alter table nft alter column manager_update_lt set not null;

alter table nft_attributes alter column collection set not null;

alter table nft_auction alter column nft set not null;
alter table nft_auction alter column status set not null;
alter table nft_auction alter column root set not null;

alter table nft_auction_bid alter column next_bid_value set not null;
alter table nft_auction_bid alter column declined set not null;

alter table nft_collection alter column first_mint set not null;

alter table nft_direct_buy alter column buyer set not null;
alter table nft_direct_buy alter column expired_at set not null;
alter table nft_direct_buy alter column root set not null;

alter table nft_direct_sell alter column seller set not null;
alter table nft_direct_sell alter column expired_at set not null;
alter table nft_direct_sell alter column root set not null;
