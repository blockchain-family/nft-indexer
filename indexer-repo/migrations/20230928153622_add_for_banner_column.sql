alter table nft_collection add column for_banner boolean default false;
create index on nft_collection (for_banner asc);