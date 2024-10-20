CREATE type sale_status AS enum ('LivePreSale', 'LiveSale', 'ComingSoon', 'Completed');

CREATE TABLE announcements
(
    id serial NOT NULL
        CONSTRAINT announcements_pkey PRIMARY KEY,
    title text NOT NULL,
    status sale_status NOT NULL,
    banner text,
    nft_planned_quantity integer,
    nft_minted_quantity integer,
    lowest_nft_price numeric(20,2),
    marketplace_lowest_price numeric(20,2),
    sale_start_date timestamp NOT NULL,
    presale_start_date timestamp,
    sale_end_date timestamp,
    slug text NOT NULL,
    description text NOT NULL,
    logo text,
    creator_address text,
    collection_address text,
    marketplace_contract_address text,
    final_hash text,
    social_links jsonb,
    collection_general_info jsonb NOT NULL,
    pricing_rules jsonb,
    tokenomics jsonb,
    roadmap jsonb,
    utilities jsonb,
    sponsors jsonb,
    video_block jsonb,
    artist_info jsonb
);

CREATE UNIQUE INDEX idx_lower_slug ON announcements (LOWER(slug));

create index announcements_lowest_nft_price_index
	on announcements (lowest_nft_price);

create index announcements_nft_quantity
	on announcements ((nft_planned_quantity - nft_minted_quantity));

create index announcements_sale_start_date_index
	on announcements (sale_start_date);

create index announcements_status_index
	on announcements (status);

