create index meta_handled_addresses_failed_updated_at_index
	on meta_handled_addresses (failed asc, updated_at desc);