create table nft_attributes(
    nft t_address not null,
    collection t_address null,
    raw jsonb not null,
    trait_type varchar(200) not null,
    value jsonb null,

    constraint nft_attributes_no_dups unique (nft, collection, raw, trait_type, value)
);

create index ix_nft_attributes_nft on nft_attributes using btree (nft);
create index ix_nft_attributes_collection on nft_attributes using btree (collection);
create index ix_nft_attributes_trait_type on nft_attributes using btree (trait_type);