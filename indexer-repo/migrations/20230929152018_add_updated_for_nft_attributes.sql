alter table nft_attributes
    add column updated timestamp not null default to_timestamp(0);

alter table nft_attributes
    alter column updated drop default;