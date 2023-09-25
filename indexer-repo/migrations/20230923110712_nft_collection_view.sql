create materialized view collection_type_mv as
select
    mv.collection_address,
    mv.mimetype
from
    (
        select
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

create index on collection_type_mv (mimetype);

create materialized view nft_type_mv as
select
    n.address as "nft_address",
    jsonb_array_elements(nm.meta -> 'files')->> 'mimetype' as mimetype
from
    nft_verified_mv n
    join nft_metadata nm on nm.nft = n.address
where
    jsonb_typeof(nm.meta -> 'files') = 'array';

create index on nft_type_mv (mimetype);