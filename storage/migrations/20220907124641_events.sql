CREATE TYPE event_type AS ENUM (
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

CREATE TYPE event_category AS ENUM (
    'auction',
    'direct_buy',
    'direct_sell'
);

create table events_whitelist(
	address t_address PRIMARY KEY
);

create table nft_events(
	id bigint not null generated always as identity,
    event_cat event_category NOT NULL,
	event_type event_type NOT NULL,
	address t_address references events_whitelist(address),
    created_lt bigint not null,
    created_at bigint not null,
    checked boolean not null,
	args jsonb,
	
	constraint nft_events_pk primary key (id),
	constraint nft_events_unique unique (address, created_lt, created_at)
);

create index ix_nft_events_address ON nft_events USING btree (address);
create index ix_nft_events_type ON nft_events USING btree (event_type);
create index ix_nft_events_cat ON nft_events USING btree (event_cat);



