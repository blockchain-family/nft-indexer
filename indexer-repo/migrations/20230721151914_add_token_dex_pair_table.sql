-- Add migration script here

create type bc_name as enum(
  'everscale',
  'venom'
);

create table token_to_dex (
  token t_address,
  pair t_address not null,
  source bc_name not null,
  is_l2r boolean not null,
  decimals integer not null,
  constraint token_to_dex_pk primary key(token)
);

insert into token_to_dex(token, pair, source, is_l2r, decimals) 
values 
  ('0:2c3a2ff6443af741ce653ae4ef2c85c2d52a9df84944bbe14d702c3131da3f14', '0:96d1750dc6727af21d0ab3a15cf4b63af449e8948328bff03873164d4c1342a7', 'venom', false, 9),
  ('0:28237a5d5abb32413a79b5f98573074d3b39b72121305d9c9c97912fc06d843c', '0:96d1750dc6727af21d0ab3a15cf4b63af449e8948328bff03873164d4c1342a7', 'venom', false, 9);
