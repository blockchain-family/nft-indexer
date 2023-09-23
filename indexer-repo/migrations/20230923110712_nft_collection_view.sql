create materialized view nft_collection_type as
select c.address as "collection_address", n.address as "nft_address", jsonb_array_elements(nm.meta->'files')->>'mimetype' as mimetype
from nft n
    left join nft_collection c on n.collection = c.address
    left join nft_metadata nm on nm.nft = n.address
where jsonb_typeof(nm.meta->'files') = 'array';

create index on nft_collection_type (collection_address);
create index on nft_collection_type (nft_address);
create index on nft_collection_type (mimetype);

select cron.schedule('refresh nft_collection_type', '*/30 * * * *',
                     'refresh materialized view concurrently nft_collection_type;');