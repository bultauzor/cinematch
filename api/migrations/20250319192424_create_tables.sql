create table if not exists users
(
  user_id  uuid primary key not null default gen_random_uuid(), 
  username text             not null unique,
  password text             not null,
  avatar   text
);
