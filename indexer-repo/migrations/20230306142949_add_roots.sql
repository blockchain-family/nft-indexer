create type t_root_types as enum ('auction', 'sell', 'buy');

create table roots
(
	event_whitelist_address t_address not null
		constraint roots_pk
			primary key
		constraint roots_events_whitelist_address_fk
			references events_whitelist,
	code t_root_types
);


