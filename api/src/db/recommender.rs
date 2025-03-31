use crate::db::DbHandler;
use crate::model::content::ContentType;
use crate::model::recommendation::{
    Recommendation, RecommendationDb, RecommendationParameters, RecommendationParametersInput,
};
use chrono::NaiveDateTime;
use chrono::Utc;
use sqlx::QueryBuilder;
use uuid::Uuid;

impl DbHandler {
    pub async fn update_recommender_embeddings(&self) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!("refresh materialized view contents_seen_mrated;")
            .execute(&mut *trx)
            .await?;

        sqlx::query!("refresh materialized view concurrently contents_seen_quantized;")
            .execute(&mut *trx)
            .await?;

        sqlx::query!("refresh materialized view concurrently recommender_vectors;")
            .execute(&mut *trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }

    pub async fn get_user_recommender_embeddings(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Recommendation>, sqlx::Error> {
        let res = sqlx::query!(r#"
            with user_embedding as (select user_id, embedding
                                    from recommender_vectors
                                    where user_id = $1),
                 recommended_users as (select rv.user_id, 1 - (rv.embedding <%> ue.embedding) as score
                                       from recommender_vectors as rv
                                                join user_embedding as ue on true
                                       where rv.user_id != ue.user_id
                                       order by rv.embedding <%> ue.embedding
                                       limit 5),
                 cs_me as (select * from contents_seen where user_id = $1)
            select other.content_id, max(other.grade / 10 * ru.score) as score
            from contents_seen as other
                     full outer join cs_me as me
                                     on other.content_id = me.content_id
                     join recommended_users as ru on other.user_id = ru.user_id
            where other.user_id in (select user_id from recommended_users)
              and me.content_id is null
              and other.grade > 5
            group by other.content_id
            order by score desc"#, user_id).fetch_all(&self.pool).await?;

        Ok(res
            .into_iter()
            .map(|e| Recommendation {
                content_id: e.content_id,
                score: e.score.unwrap_or_default(),
                method: vec![],
                o1: 0,
                o2: 0,
            })
            .collect())
    }

    pub async fn get_recommender_providers_updated_at(
        &self,
        content_id: &Uuid,
    ) -> Result<NaiveDateTime, sqlx::Error> {
        let res = sqlx::query!(
            "select updated_at from recommender_providers where content_id = $1",
            content_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.map(|e| e.updated_at).unwrap_or_default())
    }

    pub async fn update_recommender_providers_rel(
        &self,
        content_id: &Uuid,
        rels: Vec<Uuid>,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            "insert into recommender_providers values ($1, $2) on conflict(content_id) do update set updated_at = $2",
            content_id, Utc::now().naive_utc()
        )
            .execute(&mut *trx)
            .await?;

        sqlx::query!(
            "delete from recommender_providers_rel where a = $1",
            content_id
        )
        .execute(&mut *trx)
        .await?;

        for (pos, rel) in rels.into_iter().enumerate() {
            sqlx::query!(
                "insert into recommender_providers_rel values ($1, $2, $3)",
                content_id,
                rel,
                pos as i32
            )
            .execute(&mut *trx)
            .await?;
        }

        trx.commit().await?;

        Ok(())
    }

    pub async fn create_recommendations(
        &self,
        params: RecommendationParametersInput,
    ) -> Result<RecommendationParameters, sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let res = sqlx::query!(
            "insert into recommendations (hash) values ($1) returning recommendation_id",
            &params.hash()
        )
        .fetch_one(&mut *trx)
        .await?;

        let params = params.hydrate(res.recommendation_id);

        sqlx::query!(
            "select create_recommendation($1, $2, $3, $4, $5, $6, $7)",
            params.recommendation_id,
            &params.users_input,
            &params.not_seen_by,
            params.disable_content_type_filter,
            params.content_type as ContentType,
            params.disable_genre_filter,
            &params.genres
        )
        .execute(&mut *trx)
        .await?;

        trx.commit().await?;

        Ok(params)
    }

    pub async fn get_recommendations_by_hash(
        &self,
        hash: &[u8; 32],
    ) -> Result<Option<RecommendationDb>, sqlx::Error> {
        sqlx::query_as!(RecommendationDb, "select recommendation_id, hash, updated_at, refcount from recommendations where hash = $1", hash)
            .fetch_optional(&self.pool).await
    }

    pub async fn delete_recommendations(
        &self,
        recommendation_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            "delete from recommendations where recommendation_id = $1",
            recommendation_id
        )
        .execute(&mut *trx)
        .await?;

        sqlx::query!("call delete_recommendations($1)", recommendation_id)
            .execute(&mut *trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }

    pub async fn update_recommendations(
        &self,
        recommendation_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            "update recommendations set updated_at = now() where recommendation_id = $1",
            recommendation_id
        )
        .execute(&mut *trx)
        .await?;

        sqlx::query!("call update_recommendations($1)", recommendation_id)
            .execute(&mut *trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }

    pub async fn fetch_recommendations_from(
        &self,
        recommendation_id: &Uuid,
        from: i64,
        limit: i64,
    ) -> Result<Vec<Recommendation>, sqlx::Error> {
        // Thanks rust for being strongly typed 😊
        let mut qb = QueryBuilder::new(format!(
            r#"select content_id, score, method, o1, o2
            from "recommendation_{recommendation_id}"
            where o1 >= '{from}'
            limit '{limit}';"#
        ));
        let qb = qb.build_query_as::<Recommendation>();

        qb.fetch_all(&self.pool).await
    }

    pub async fn fetch_recommendations_from_to(
        &self,
        recommendation_id: &Uuid,
        from: i64,
        to: i64,
    ) -> Result<Vec<Recommendation>, sqlx::Error> {
        // Thanks rust for being strongly typed 😊
        let mut qb = QueryBuilder::new(format!(
            r#"select content_id, score, method, o1, o2
            from "recommendation_{recommendation_id}"
            where o1 >= '{from}' and o1 <= '{to}';"#
        ));
        let qb = qb.build_query_as::<Recommendation>();

        qb.fetch_all(&self.pool).await
    }

    pub async fn inc_recommendation_arc(
        &self,
        recommendation_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "update recommendations set refcount = refcount + 1 where recommendation_id = $1",
            recommendation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn dec_recommendation_arc(
        &self,
        recommendation_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "update recommendations set refcount = refcount - 1 where recommendation_id = $1",
            recommendation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recommendation_outdated(&self) -> Result<Vec<RecommendationDb>, sqlx::Error> {
        sqlx::query_as!(
            RecommendationDb,
            "select recommendation_id, hash, updated_at, refcount
            from recommendations
            where (refcount <= 0 and (updated_at + interval '1 hour') < now())
               or (updated_at + interval '4 hour' < now())"
        )
        .fetch_all(&self.pool)
        .await
    }
}
