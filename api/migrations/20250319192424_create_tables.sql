create table if not exists users
(
    user_id  uuid primary key default gen_random_uuid(),
    username text not null unique,
    password text not null,
    avatar   text
);

do
$$
    begin
        create type content_type as enum ('movie', 'show');
    exception
        when duplicate_object then null;
    end
$$;

create table if not exists contents
(
    content_id   uuid primary key default gen_random_uuid(),
    provider_id  text         not null unique,
    updated_at   timestamp    not null,
    content_type content_type not null,
    title        text         not null,
    overview     text         not null,
    poster       text,
    release_date date
);

create table if not exists contents_genres
(
    content_id uuid references contents,
    genre      text,
    primary key (content_id, genre)
);
