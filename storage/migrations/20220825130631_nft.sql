--
create table nft_metadata(
	id bigint not null generated always as identity,

    nft varchar(255) not null,
    meta jsonb not null,

    constraint nft_metadata_pk primary key (id),
	constraint nft_metadata_unique unique (nft)
);

CREATE DOMAIN t_address AS varchar(67) NOT NULL;

CREATE TABLE nft_collection(
    address t_address PRIMARY KEY,
    owner t_address,
    name text,
    description text,
    created timestamp NOT NULL,
    updated timestamp
);


CREATE TABLE nft(
    address t_address PRIMARY KEY,
    collection t_address NOT NULL,
    owner t_address,
    name text,
    description text,
    created timestamp NOT NULL,
    updated timestamp
);


CREATE TABLE nft_auction(
    address t_address,
    owner t_address,
    nft t_address,
    price_token t_address,
    start_price bigint NOT NULL,
    max_bid bigint NOT NULL,
    created timestamp NOT NULL,
    finished_at timestamp,
    updated timestamp,

    constraint nft_auction_pk primary key (address, owner, nft)
);

CREATE TABLE nft_auction_bid(
    auction t_address,
    nft t_address,
    owner t_address,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price bigint NOT NULL,

    constraint nft_auction_bid_pk primary key (auction, nft, owner)
);

CREATE TABLE nft_forsale(
    nft t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address,
    price bigint NOT NULL
);

CREATE TABLE nft_offer(
    nft t_address,
    owner t_address,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price bigint NOT NULL,
    expired_at timestamp,

    constraint nft_offer_pk primary key (nft, owner)
);


