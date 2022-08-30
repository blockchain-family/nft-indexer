--
CREATE DOMAIN t_address AS 
   varchar(67) NOT NULL;

CREATE TABLE nft (
    address t_address PRIMARY KEY,
    owner t_address,
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
    created timestamp NOT NULL,
    updated timestamp,
);

CREATE TABLE nft_auction_bid (
    auction_id t_address PRIMARY KEY,
    from t_address PRIMARY KEY,
    to t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price int64 NOT NULL,
);

CREATE TABLE nft_forsale (
    nft t_address PRIMARY KEY,
    from t_address PRIMARY KEY,
    to t_address PRIMARY KEY,
    created timestamp NOT NULL,
    price_token t_address NOT NULL,
    price int64 NOT NULL,
);

