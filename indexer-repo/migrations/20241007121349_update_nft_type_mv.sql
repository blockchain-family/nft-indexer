drop materialized view if exists collection_type_mv;
drop materialized view if exists nft_type_mv;

create materialized view nft_type_mv as
select distinct n.address                                               as "nft_address",
                jsonb_array_elements(nm.meta -> 'files') ->> 'mimetype' as "mimetype",
                c.verified                                              as "verified"
from nft_collection c
         join nft n on n.collection = c.address
         join nft_metadata nm on nm.nft = n.address
where jsonb_typeof(nm.meta -> 'files') = 'array';

create unique index on nft_type_mv (nft_address, mimetype, verified);
create index nft_type_index on nft_type_mv (nft_address, mimetype);

create materialized view collection_type_mv as
select distinct nc.address   as "collection_address",
                ntm.mimetype as "mimetype",
                nc.verified  as "verified"
from nft_type_mv ntm
         join nft n on n.address = ntm.nft_address
         join nft_collection nc on nc.address = n.collection;

create unique index on collection_type_mv (collection_address, mimetype, verified);
create index collection_type_verified_index on collection_type_mv (collection_address, mimetype);
