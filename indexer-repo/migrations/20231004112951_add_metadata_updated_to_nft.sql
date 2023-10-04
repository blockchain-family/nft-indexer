alter table nft add column metadata_updated_at bigint;
CREATE INDEX nft_metadata_updated_index ON public.nft USING btree (metadata_updated_at);