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
    backdrop     text,
    vote_count   integer,
    vote_average float,
    release_date date
);

create table if not exists contents_genres
(
    content_id uuid references contents,
    genre      text,
    primary key (content_id, genre)
);

create table if not exists contents_seen
(
    content_id uuid not null references contents,
    user_id    uuid not null references users,
    grade      float,
    primary key (content_id, user_id)
);

create table if not exists session_requests
(
    user_id          uuid references users on delete cascade,
    owner_id         uuid references users on delete cascade not null,
    session_id  uuid,
    created_at       timestamp default now() not null,
    primary key (user_id, session_id)
);
create table if not exists friends
(
   user_id uuid references users on delete cascade,
   friend_id uuid references users on delete cascade,
   primary key (user_id, friend_id)
    );

create table if not exists friend_requests
(
    user_id          uuid references users on delete cascade,
    friend_asked_id  uuid references users on delete cascade,
    created_at       timestamp default now(),
    primary key (user_id, friend_asked_id)
    );