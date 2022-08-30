--
CREATE DOMAIN t_address AS 
   varchar(67) NOT NULL;

CREATE TABLE nft_collection (
    address t_address PRIMARY KEY,
    owner t_address,
    name text,
    description text,
    created timestamp NOT NULL,
    updated timestamp,
);

CREATE TABLE nft (
    address t_address PRIMARY KEY,
    collection t_address NOT NULL,
    owner t_address,
    name text,
    description text,
    created timestamp NOT NULL,
    updated timestamp,
);

CREATE TABLE nft_meta (
    data_hash bytea PRIMARY KEY,
    nft t_address PRIMARY KEY,
    data bytea,
    created timestamp NOT NULL,
);

CREATE TABLE nft_auction (
    address t_address PRIMARY KEY,
    owner t_address PRIMARY KEY,
    nft t_address PRIMARY KEY,
    price_token t_address NOT NULL,
    start_price int64 NOT NULL,
    max_bid int64 NOT NULL,
    created timestamp NOT NULL,
    finished_at timestamp,
    updated timestamp,
);

CREATE TABLE nft_auction_bid (
    auction_id t_address PRIMARY KEY,
    nft t_address PRIMARY KEY,
    from t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price int64 NOT NULL,
);

CREATE TABLE nft_forsale (
    nft t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price int64 NOT NULL,
);

CREATE TABLE nft_offer (
    nft t_address PRIMARY KEY,
    from t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price int64 NOT NULL,
    expired_at timestamp,
);


