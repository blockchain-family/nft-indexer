alter table handled_nft rename to meta_handled_addresses;
alter table meta_handled_addresses add column updated_at bigint not null;
alter table meta_handled_addresses add column failed boolean default false;