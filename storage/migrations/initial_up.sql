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
    'direct_sell_state_changed'
);

create type event_category as enum (
    'auction',
    'direct_buy',
    'direct_sell'
);

create domain t_address as varchar(67) not null;

CREATE TABLE nft_collection(
    address t_address PRIMARY KEY,
    owner t_address,
    name text,
    description text,
    updated timestamp NOT NULL
);

CREATE TABLE nft(
    address t_address PRIMARY KEY,
    collection t_address NOT NULL,
    owner t_address,
    manager t_address,
    name text,
    description text,
    updated timestamp NOT NULL
);

CREATE TABLE nft_metadata(
    id bigint not null generated always as identity,
    nft t_address,
    meta jsonb not null,
    ts timestamp not null,

    constraint nft_metadata_pk primary key (id),
    constraint nft_metadata_unique unique (nft)
);

create table events_whitelist(
	address t_address primary key
);

create table nft_events(
	id bigint not null generated always as identity,
    event_cat event_category not null,
	event_type event_type not null,
	address t_address,
    created_lt bigint not null,
    created_at bigint not null,
	args jsonb,
	
	constraint nft_events_pk primary key (id),
	constraint nft_events_unique unique (address, created_lt, created_at)
);

create index ix_nft_events_address on nft_events using btree (address);
create index ix_nft_events_type on nft_events using btree (event_type);
create index ix_nft_events_cat on nft_events using btree (event_cat);
