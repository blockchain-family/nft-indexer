CREATE INDEX nft_events_args_idx1 ON nft_events USING btree (((args ->> 'from')::int), ((args ->> 'to')::int));
create index nft_events_created_lt_index
 on nft_events (created_lt);