create table nft_collection_custom(
    address t_address primary key,
    updated timestamp not null,
    name text,
    description text,
    wallpaper t_uri,
    logo t_uri,
    social jsonb,
    FOREIGN KEY (address) REFERENCES nft_collection(address)
);