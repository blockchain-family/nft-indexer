create type event_type as enum (
    'auction_deployed',
    'auction_created',
    'auction_root_ownership_transferred',
    'auction_active',
    'auction_declined',
    'auction_bid_placed',
    'auction_bid_declined',
    'auction_cancelled',
    'auction_complete',

    'direct_buy_deployed',
    'direct_buy_declined',
    'factory_direct_buy_ownership_transferred',
    'direct_buy_state_changed',

    'direct_sell_deployed',
    'direct_sell_declined',
    'factory_direct_sell_ownership_transferred',
    'direct_sell_state_changed',

    'nft_owner_changed',
    'nft_manager_changed',

    'collection_ownership_transferred',

    'nft_created',
    'nft_burned'
);

create type event_category as enum (
    'auction',
    'direct_buy',
    'direct_sell',
    'nft',
    'collection'
);

create domain t_address as varchar(67);
create domain t_uri as varchar(200);

create table events_whitelist(
    address t_address primary key
);

create table nft_events(
    id bigint not null generated always as identity,
    event_cat event_category not null,
	event_type event_type not null,
	address t_address not null,
    nft t_address,
    collection t_address,
    created_lt bigint not null,
    created_at bigint not null,
	args jsonb,
  
	constraint nft_events_pk primary key (id),
	constraint nft_events_unique unique (address, event_type, created_lt, created_at)
);

create index ix_nft_events_address on nft_events using btree (address);
create index ix_nft_events_type on nft_events using btree (event_type);
create index ix_nft_events_cat on nft_events using btree (event_cat);

create table nft(
    address t_address not null primary key,
    collection t_address,
    owner t_address,
    manager t_address,
    name text,
    description text,
    burned boolean not null default false,
    updated timestamp not null,
    tx_lt bigint not null
);

create index ix_nft_collection on nft using btree (collection);
create index ix_nft_owner on nft using btree (owner);
create index ix_nft_manager on nft using btree (manager);

create table nft_metadata(
    nft t_address primary key references nft(address),
    meta jsonb not null,
    updated timestamp not null
);

create table nft_collection(
    address t_address primary key,
    owner t_address not null,
    name text,
    description text,
    created timestamp not null,
    updated timestamp not null,
    wallpaper t_uri,
    logo t_uri,
    total_price numeric(50, 12),
    max_price numeric(50, 12),
    owners_count int,
    verified boolean not null default false
);

create index ix_nft_collection_owner on nft_collection using btree (owner);

create view nft_details as 
    SELECT n.*,
        c.owner as collection_owner,
        c.name as collection_name,
        c.description as collection_description,
        m.meta as meta
    FROM nft n 
    LEFT JOIN nft_collection c ON n.collection = c.address
    LEFT JOIN nft_metadata m ON m.nft = n.address;

create type auction_status as enum (
    'created',
    'active',
    'cancelled',
    'completed',
    'expired'
);

create table nft_auction(
    address t_address,
    nft t_address,
    wallet_for_bids t_address,
    price_token t_address,
    start_price numeric(40),
    min_bid numeric(40),
    max_bid numeric(40),
    closing_price_usd numeric(50, 12),
    status auction_status,
    created_at timestamp,
    finished_at timestamp,
    tx_lt bigint not null,

    constraint nft_auction_pk primary key (address)
);

create index ix_nft_auction_nft on nft_auction using btree (nft);

create table nft_auction_bid(
    auction t_address not null,
    buyer t_address not null,
    price numeric(40) not null,
    next_bid_value numeric(40),
    declined boolean default false,
    created_at timestamp not null,
    tx_lt bigint not null,

    constraint nft_auction_bid_pk primary key (auction, buyer, price)
);

create index ix_nft_auction_bid_auction on nft_auction_bid using btree (auction);
create index ix_nft_auction_bid_buyer on nft_auction_bid using btree (buyer);

create type direct_sell_state as enum(
    'create',
    'await_nft',
    'active',
    'filled',
    'cancelled',
    'expired'
);

create table nft_direct_sell(
    address t_address primary key,
    nft t_address not null,
    collection t_address null,
    price_token t_address not null,
    price numeric(40) not null,
    sell_price_usd numeric(50, 12),
    seller t_address null,
    finished_at timestamp null,
    expired_at timestamp null,
    state direct_sell_state not null,
    created timestamp not null,
    updated timestamp not null,
    tx_lt bigint not null
);

create index ix_nft_direct_sell_nft on nft_direct_sell using btree (nft);
create index ix_nft_direct_sell_collection on nft_direct_sell using btree (collection);
create index ix_nft_direct_sell_seller on nft_direct_sell using btree (seller);

create type direct_buy_state as enum(
    'create',
    'await_tokens',
    'active',
    'filled',
    'cancelled',
    'expired'
);

create table nft_direct_buy(
    address t_address primary key,
    nft t_address not null,
    collection t_address null,
    price_token t_address not null,
    price numeric(40) not null,
    buy_price_usd numeric(50, 12),
    buyer t_address null,
    finished_at timestamp null,
    expired_at timestamp null,
    state direct_buy_state not null,
    created timestamp not null,
    updated timestamp not null,
    tx_lt bigint not null
);

create type nft_price_source as enum(
    'auctionBid',
    'directBuy',
    'directSell'
);

create index ix_nft_direct_buy_nft on nft_direct_buy using btree (nft);
create index ix_nft_direct_buy_collection on nft_direct_buy using btree (collection);
create index ix_nft_direct_buy_buyer on nft_direct_buy using btree (buyer);

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

create table nft_attributes(
    nft t_address not null,
    collection t_address null,
    raw jsonb not null,
    trait_type varchar(200) not null,
    value jsonb null,

    constraint nft_attributes_no_dups unique (nft, collection, raw, trait_type, value)
);

create index ix_nft_attributes_nft on nft_attributes using btree (nft);
create index ix_nft_attributes_collection on nft_attributes using btree (collection);
create index ix_nft_attributes_trait_type on nft_attributes using btree (trait_type);