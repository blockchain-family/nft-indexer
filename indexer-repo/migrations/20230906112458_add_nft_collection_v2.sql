create table nft_collection_custom(
    address t_address primary key,
    owner t_address not null,
    updated timestamp not null,
    name text,
    description text,
    wallpaper t_uri,
    logo t_uri,
    social jsonb
);

create index ix_nft_collection_custom_owner on nft_collection_custom using btree (owner);