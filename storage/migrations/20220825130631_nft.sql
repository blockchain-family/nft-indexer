create domain t_address as varchar(67) not null;
create domain t_uri as varchar(200);

create table nft_collection(
    address t_address PRIMARY KEY,
    owner t_address,
    name text,
    description text,
    created timestamp not null,
    updated timestamp,
    verified boolean not null,
    wallpaper t_uri,
    logo t_uri,
    lowest_price bigint,
    total_price bigint,
    owners_count int
);

create table nft(
    address t_address PRIMARY KEY,
    collection t_address NOT NULL references nft_collection(address),
    owner t_address,
    manager t_address,
    name text,
    description text,
    created timestamp NOT NULL,
    updated timestamp
);

create table nft_metadata(
	id bigint not null generated always as identity,
    nft t_address references nft(address),
    meta jsonb not null,
    ts timestamp not null,

    constraint nft_metadata_pk primary key (id),
	constraint nft_metadata_unique unique (nft)
);

create table nft_auction(
    address t_address,
    owner t_address,
    nft t_address references nft(address),
    price_token t_address,
    start_price bigint NOT NULL,
    max_bid bigint NOT NULL,
    created timestamp NOT NULL,
    finished_at timestamp,
    updated timestamp,

    constraint nft_auction_pk primary key (address)
);

create table nft_auction_bid(
    auction t_address references nft_auction(address),
    nft t_address references nft(address),
    owner t_address,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price bigint NOT NULL,

    constraint nft_auction_bid_pk primary key (auction, nft, owner)
);

create table nft_forsale(
    address t_address PRIMARY KEY,
    nft t_address NOT NULL references nft(address),
    created timestamp NOT NULL,
    price_token t_address,
    price bigint NOT NULL
);

create table nft_offer(
    address t_address PRIMARY KEY,
    nft t_address NOT NULL references nft(address),
    owner t_address,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price bigint NOT NULL,
    expired_at timestamp
);


