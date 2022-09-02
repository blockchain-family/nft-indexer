create table nft_metadata(
	id bigint not null generated always as identity,

    nft varchar(255) not null,
    meta jsonb not null,

    constraint nft_metadata_pk primary key (id),
	constraint nft_metadata_unique unique (nft)
);

create table auction_deployed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	offer_address varchar(255) not null,

    collection varchar(255) not null,
    nft_owner varchar(255) not null,
    nft varchar(255) not null,
    offer varchar(255) not null,
    price numeric(40) not null,
    auction_duration bigint not null,
    deploy_nonce numeric(40) not null,
	
	constraint auction_deployed_pk primary key (id),
	constraint auction_deployed_unique unique (account_addr, created_lt, created_at)
);

create table auction_ownership_transferred(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	old_owner varchar(255) not null,
    new_owner varchar(255) not null,
	
	constraint auction_ownership_transferred_pk primary key (id),
	constraint auction_ownership_transferred_unique unique (account_addr, created_lt, created_at)
);

create table auction_created(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	auction_subject varchar(255) not null,
    subject_owner varchar(255) not null,
	payment_token_root varchar(255) not null,
	wallet_for_bids varchar(255) not null,
	start_time bigint not null,
	duration bigint not null,
	finish_time bigint not null,
	now_time bigint not null,
	
	constraint auction_created_pk primary key (id),
	constraint auction_created_unique unique (account_addr, created_lt, created_at)
);

create table auction_active(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	auction_subject varchar(255) not null,
    subject_owner varchar(255) not null,
	payment_token_root varchar(255) not null,
	wallet_for_bids varchar(255) not null,
	start_time bigint not null,
	duration bigint not null,
	finish_time bigint not null,
	now_time bigint not null,
	
	constraint auction_active_pk primary key (id),
	constraint auction_active_unique unique (account_addr, created_lt, created_at)
);

create table auction_declined(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	nft_owner varchar(255) not null,
    data_address varchar(255) not null,
	
	constraint auction_declined_pk primary key (id),
	constraint auction_declined_unique unique (account_addr, created_lt, created_at)
);

create table bid_placed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	buyer_address varchar(255) not null,
    value numeric(40) not null,
	
	constraint bid_placed_pk primary key (id),
	constraint bid_placed_unique unique (account_addr, created_lt, created_at)
);

create table bid_declined(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	buyer_address varchar(255) not null,
    value numeric(40) not null,
	
	constraint bid_declined_pk primary key (id),
	constraint bid_declined_unique unique (account_addr, created_lt, created_at)
);

create table auction_complete(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,

	buyer_address varchar(255) not null,
    value numeric(40) not null,
	
	constraint auction_complete_pk primary key (id),
	constraint auction_complete_unique unique (account_addr, created_lt, created_at)
);

create table auction_cancelled(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	constraint auction_cancelled_pk primary key (id),
	constraint auction_cancelled_unique unique (account_addr, created_lt, created_at)
);

create table direct_buy_deployed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	direct_buy_address varchar(255) not null,
	sender varchar(255) not null,
	token_root varchar(255) not null,
	nft varchar(255) not null,
	nonce numeric(40) not null,
	amount numeric(40) not null,
	
	constraint direct_buy_deployed_pk primary key (id),
	constraint direct_buy_deployed_unique unique (account_addr, created_lt, created_at)
);

create table direct_buy_declined(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	sender varchar(255) not null,
	token_root varchar(255) not null,
	amount numeric(40) not null,
	
	constraint direct_buy_declined_pk primary key (id),
	constraint direct_buy_declined_unique unique (account_addr, created_lt, created_at)
);

create table direct_buy_ownership_transferred(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	old_owner varchar(255) not null,
	new_owner varchar(255) not null,
	
	constraint direct_buy_ownership_transferred_pk primary key (id),
	constraint direct_buy_ownership_transferred_unique unique (account_addr, created_lt, created_at)
);

create table direct_sell_deployed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	_direct_sell_address varchar(255) not null,
	sender varchar(255) not null,
	payment_token varchar(255) not null,
	nft varchar(255) not null,
	_nonce numeric(40) not null,
	price numeric(40) not null,
	
	constraint direct_sell_deployed_pk primary key (id),
	constraint direct_sell_deployed_unique unique (account_addr, created_lt, created_at)
);

create table direct_sell_declined(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	sender varchar(255) not null,
	_nft_address varchar(255) not null,
	
	constraint direct_sell_declined_pk primary key (id),
	constraint direct_sell_declined_unique unique (account_addr, created_lt, created_at)
);

create table direct_sell_ownership_transferred(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	old_owner varchar(255) not null,
	new_owner varchar(255) not null,
	
	constraint direct_sell_ownership_transferred_pk primary key (id),
	constraint direct_sell_ownership_transferred_unique unique (account_addr, created_lt, created_at)
);

create table direct_buy_state_changed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	from_state smallint not null,
	to_state smallint not null,
	
	factory varchar(255) not null,
	creator varchar(255) not null,
	spent_token varchar(255) not null,
	nft varchar(255) not null,
	_time_tx bigint not null,
	_price numeric(40) not null,
	spent_wallet varchar(255) not null,
	status smallint not null,
	sender varchar(255) not null,
	start_time_buy bigint not null,
	duration_time_buy bigint not null,
	end_time_buy bigint not null,
	
	constraint direct_buy_state_changed_pk primary key (id),
	constraint direct_buy_state_changed_unique unique (account_addr, created_lt, created_at)
);

create table direct_sell_state_changed(
	id bigint not null generated always as identity,
	
	account_addr varchar(255) not null,
    created_lt bigint not null,
    created_at bigint not null,
	
	from_state smallint not null,
	to_state smallint not null,
	
	factory varchar(255) not null,
	creator varchar(255) not null,
	token varchar(255) not null,
	nft varchar(255) not null,
	_time_tx bigint not null,
	start_time bigint not null,
	end_time bigint not null,
	_price numeric(40) not null,
	wallet varchar(255) not null,
	status smallint not null,
	sender varchar(255) not null,
	
	constraint direct_sell_state_changed_pk primary key (id),
	constraint direct_sell_state_changed_unique unique (account_addr, created_lt, created_at)
);
