create extension if not exists vector;

-- Materialized view to get the 64000 most rated content ordered by id
create materialized view if not exists contents_seen_mrated as
select res.content_id
from (select content_id
      from contents_seen
      where grade is not null
      group by content_id
      order by count(content_id) desc
      limit 64000) as res
order by content_id;

-- Add some indexes
create unique index if not exists contents_seen_mrated_content_id_idx on contents_seen_mrated (content_id);

-- Materialized view to get bit quantized scores for contents_seen
create materialized view if not exists contents_seen_quantized as
select user_id, content_id, round(grade / 10 - 0.000001)::int::bit as grade
from contents_seen
where grade is not null;

-- Add some indexes
create index if not exists contents_seen_quantized_user_id_idx on contents_seen_quantized (user_id);
create index if not exists contents_seen_quantized_content_id_idx on contents_seen_quantized (content_id);
create unique index if not exists contents_seen_quantized_user_id_content_id_idx on contents_seen_quantized (user_id, content_id);

-- The magic part, function that compute embedding for a user
create or replace function recommander_vectorize_user(_user_id uuid)
    returns text as
$func$
with csq_where as (select *
                   from public.contents_seen_quantized -- have to force the schema here
                   where user_id = _user_id),
     res as (select csm.content_id,
                    _user_id                     as user_id,
                    case
                        when csq.content_id is null then '0' -- let's say the user doesn't like anything by default
                        else csq.grade::text end as grade
             from public.contents_seen_mrated as csm -- have to force the schema here
                      full outer join csq_where as csq on csq.content_id = csm.content_id
             order by csm.content_id)
select string_agg(grade, '') -- magic happens here actually
from res
group by user_id
$func$
    language sql;

-- Materialized view to with embeddings
create materialized view if not exists recommender_vectors as
select user_id, recommander_vectorize_user(user_id)::bit(64000) as embedding
from contents_seen_quantized
group by user_id;

-- Add some indexes
create unique index if not exists recommender_vectors_user_id_idx on recommender_vectors (user_id);
create index if not exists recommender_vectors_embedding_idx on recommender_vectors using hnsw ((embedding) bit_jaccard_ops);

-- Providers recommendation part

create table if not exists recommender_providers
(
    content_id uuid primary key references contents,
    updated_at timestamp not null
);

create table if not exists recommender_providers_rel
(
    a   uuid references contents,
    b   uuid references contents,
    idx smallint not null,
    primary key (a, b)
);


-- The other magic or rather the dark magic part, function that actually do the recommendations 🤯
-- It creates a materialized view that holds the recommendation for a set of parameters thus enabling:
--   - "Paging" support for recommendation
--   - Extremely light pressure between the api and the db
--   - Advanced mechanism to cache recommendations
-- It combines the two recommendation methods (that should be documented)
create or replace function create_recommendation(id uuid, users_input uuid[], not_seen_by uuid[],
                                                 disable_content_type_filter bool,
                                                 content_type content_type, disable_genre_filter bool, genres text[])
    returns void as
$func$
begin
    execute format('create materialized view if not exists %I as
            with user_embedding as (select user_id, embedding
                                    from recommender_vectors
                                    where user_id = any (%L)), -- $1 Users input
                 recommended_users as (select rv.user_id, 1 - (rv.embedding <%%> ue.embedding) as score
                                       from recommender_vectors as rv
                                                join user_embedding as ue on true
                                       where rv.user_id != ue.user_id
                                       order by rv.embedding <%%> ue.embedding
                                       limit 50),
                 seen as (select content_id, grade
                          from contents_seen
                          where user_id = any (%L) -- $1 Users input
                            and grade > 5),
                 r1 as (select rpr.b                                                         as content_id,
                               1 - (rpr.idx::float / max(rpr.idx) over (partition by rpr.a)) as coef,
                               s.grade,
                               0                                                             as magic
                        from recommender_providers_rel as rpr
                                 join seen as s on rpr.a = s.content_id),
                 r2 as (select rpr.b                                                                    as content_id,
                               (1 - (rpr.idx::float / max(rpr.idx) over (partition by rpr.a))) * s.coef as coef,
                               s.grade,
                               0.05                                                                     as magic
                        from recommender_providers_rel as rpr
                                 join r1 as s on rpr.a = s.content_id),
                 r3 as (select rpr.b                                                                    as content_id,
                               (1 - (rpr.idx::float / max(rpr.idx) over (partition by rpr.a))) * s.coef as coef,
                               s.grade,
                               0.1                                                                      as magic
                        from recommender_providers_rel as rpr
                                 join r2 as s on rpr.a = s.content_id),
                 r as (select * from r1 union table r2 union table r3),
                 cs_me as (select content_id
                           from contents_seen
                           where user_id = any (%L)),          -- $2 Exclude seen by user
                 provider as (select r.content_id, max((0.9 + r.coef / 10 - r.magic) * r.grade / 10) as score
                              from r
                              where content_id not in (select content_id from cs_me)
                              group by r.content_id),
                 provider_filtered as (select p.content_id, any_value(p.score) as score, ''provider'' as method
                                       from provider as p
                                                join contents as c on p.content_id = c.content_id
                                                full join contents_genres as cg on c.content_id = cg.content_id
                                       where p.content_id is not null
                                         and (%L or c.content_type = %L) -- $3 and $4 Disable type filter or filter by type
                                         and (%L or cg.genre = any (%L))
                                       group by p.content_id), -- $5 and $6 Disable genre filter or filter by genre
                 embeddings as (select other.content_id, max(other.grade / 10 * ru.score) as score
                                from contents_seen as other
                                         full outer join cs_me as me
                                                         on other.content_id = me.content_id
                                         join recommended_users as ru on other.user_id = ru.user_id
                                where other.user_id in (select user_id from recommended_users)
                                  and me.content_id is null
                                  and other.grade > 5
                                group by other.content_id),
                 embeddings_filtered as (select e.content_id, any_value(e.score) as score, ''embeddings'' as method
                                         from embeddings as e
                                                  join contents as c on e.content_id = c.content_id
                                                  full join contents_genres as cg on c.content_id = cg.content_id
                                         where e.content_id is not null
                                           and (%L or c.content_type = %L) -- $3 and $4 Disable type filter or filter by type
                                           and (%L or cg.genre = any (%L)) -- $5 and $6 Disable genre filter or filter by genre
                                         group by e.content_id),
                 filtered as (select content_id, score, method from provider_filtered union table embeddings_filtered)
            select content_id,
                   max(score)                                   as score,
                   array_agg(method)                            as method,
                   row_number() over (order by max(score) desc) as o1,
                   row_number() over (order by max(score))      as o2
            from filtered
            group by content_id
            order by score desc;
            create index if not exists %I on %I (o1);
            create index if not exists %I on %I (o2);', 'recommendation_' || id, users_input, users_input,
                   not_seen_by, disable_content_type_filter,
                   content_type, disable_genre_filter, genres, disable_content_type_filter, content_type,
                   disable_genre_filter, genres, 'recommendation_' || id || '_o1_idx',
                   'recommendation_' || id, 'recommendation_' || id || '_o2_idx',
                   'recommendation_' || id);
end;
$func$
    language plpgsql volatile;

-- Safely delete recommendations
create or replace procedure delete_recommendations(id uuid)
    language plpgsql as
$func$
begin
    execute format('drop materialized view %I;', 'recommendation_' || id);
end;
$func$;

-- Safely update recommendations
create or replace procedure update_recommendations(id uuid)
    language plpgsql as
$func$
begin
    execute format('refresh materialized view %I;', 'recommendation_' || id);
end;
$func$;

-- Track recommendations materialized view
create table if not exists recommendations
(
    recommendation_id uuid primary key default gen_random_uuid(),
    hash              bytea unique                   not null,
    updated_at        timestamp        default now() not null,
    refcount          smallint         default 0     not null
);

create index if not exists recommendations_hash_idx on recommendations (hash);

