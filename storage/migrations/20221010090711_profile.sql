create table if not exists profile(
	address t_address primary key,
    name varchar(100) not null,
    bio text null,
    image t_uri null,
    site t_uri null,
    email varchar(100) null,
    twitter varchar(100) null,
    created timestamp not null
);
