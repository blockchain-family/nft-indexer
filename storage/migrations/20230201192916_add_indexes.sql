CREATE INDEX nft_events_args_idx1 ON nft_events USING btree (((args ->> 'from')::int), ((args ->> 'to')::int));
create index nft_events_created_lt_index
 on nft_events (created_lt);

create index nft_direct_buy_finished_at_index
	on nft_direct_buy (finished_at);

create index nft_direct_sell_finished_at_index
	on nft_direct_sell (finished_at);

create index nft_auction_finished_at_index
	on nft_auction (finished_at);

create index nft_updated_index
	on nft (updated);

create index nft_auction_nft_status_index
	on nft_auction (nft, status);

create index nft_direct_sell_nft_state_index
	on nft_direct_sell (nft, state);

drop index ix_nft_direct_sell_state;

create index nft_direct_buy_nft_state_index
	on nft_direct_buy (nft, state);

drop index ix_nft_direct_buy_state;

