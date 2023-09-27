create materialized view collection_type_mv as
select
    mv.collection_address,
    mv.mimetype
from
    (
        select distinct
            c.address as "collection_address",
            jsonb_array_elements(nm.meta -> 'files')->> 'mimetype' as mimetype
        from
            nft_collection c
            join nft n on n.collection = c.address
            join nft_metadata nm on nm.nft = n.address
        where
            jsonb_typeof(nm.meta -> 'files') = 'array'
    ) mv
GROUP BY
    mv.collection_address,
    mv.mimetype;

create unique index on collection_type_mv (collection_address, mimetype);
create index collection_type_mv_gin_index ON collection_type_mv USING gin (mimetype gin_trgm_ops);

create materialized view nft_type_mv as
select
    n.address as "nft_address",
    jsonb_array_elements(nm.meta -> 'files')->> 'mimetype' as mimetype
from
    nft_verified_mv n
    join nft_metadata nm on nm.nft = n.address
where
    jsonb_typeof(nm.meta -> 'files') = 'array';

create unique index on nft_type_mv (nft_address, mimetype);
create index nft_type_mv_gin_index ON nft_type_mv USING gin (mimetype gin_trgm_ops);