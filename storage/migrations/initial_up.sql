create type event_type as enum (
    'auction_deployed',
    'auction_created',
    'auction_ownership_transferred',
    'auction_active',
    'auction_declined',
    'auction_bid_placed',
    'auction_bid_declined',
    'auction_cancelled',
    'auction_complete',

    'direct_buy_deployed',
    'direct_buy_declined',
    'direct_buy_ownership_transferred',
    'direct_buy_state_changed',

    'direct_sell_deployed',
    'direct_sell_declined',
    'direct_sell_ownership_transferred',
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

create table events_whitelist(
	address t_address primary key
);

create table nft_events(
	id bigint not null generated always as identity,
    event_cat event_category not null,
	event_type event_type not null,
	address t_address not null,
    created_lt bigint not null,
    created_at bigint not null,
	args jsonb,
	
	constraint nft_events_pk primary key (id),
	constraint nft_events_unique unique (address, created_lt, created_at)
);

create index ix_nft_events_address on nft_events using btree (address);
create index ix_nft_events_type on nft_events using btree (event_type);
create index ix_nft_events_cat on nft_events using btree (event_cat);

create table nft(
    address t_address primary key,
    collection t_address,
    owner t_address,
    manager t_address,
    name text not null,
    description text not null,
    burned boolean default false,
    updated timestamp not null,
    tx_lt bigint not null
);

create table nft_metadata(
    nft t_address primary key,
    meta jsonb not null,
    updated timestamp not null
);

create table nft_collection(
    address t_address primary key,
    owner t_address not null,
    name text not null,
    description text not null,
    updated timestamp not null,
    tx_lt bigint not null
);
