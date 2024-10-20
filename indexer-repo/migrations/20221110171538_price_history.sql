create type nft_price_source as enum(
    'auctionBid',
    'directBuy',
    'directSell'
);

create table nft_price_history(
    source t_address not null,
    source_type nft_price_source not null,
    ts timestamp not null,
    price numeric(40) not null,
    price_token t_address null,
    nft t_address null,
    collection t_address null
);

create index idx_nft_price_history_nft on nft_price_history using btree (nft);
create index idx_nft_price_history_collection on nft_price_history using btree (collection);
create index idx_nft_price_history_ts on nft_price_history using btree (ts);