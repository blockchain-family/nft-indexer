alter table nft_events drop column local_created_at;
alter table nft_events add local_created_at timestamp default now() - interval '1 year' not null;
ALTER TABLE nft_events ALTER COLUMN local_created_at SET DEFAULT now();
create index nft_events_local_created_at_index on nft_events (local_created_at);

