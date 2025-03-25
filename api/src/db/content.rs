use crate::api::friends::{Friend, FriendInput, FriendRequest};
use crate::db::DbHandler;
use crate::model::content::{Content, ContentInput, ContentType};
use crate::provider::ProviderKey;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

impl DbHandler {
    pub async fn get_content_by_provider_key(
        &self,
        pk: &ProviderKey,
    ) -> Result<Option<Content>, sqlx::Error> {
        let content = sqlx::query!(
            r#"
            select content_id,
                   provider_id,
                   updated_at,
                   content_type as "content_type: ContentType",
                   title,
                   overview,
                   poster,
                   release_date
            from contents
            where provider_id = $1"#,
            pk.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        let content = match content {
            Ok(content) => content,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            e => e?,
        };

        let genres = sqlx::query!(
            "select genre from contents_genres where content_id = $1;",
            content.content_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(Content {
            content_id: content.content_id,
            provider_id: content.provider_id,
            updated_at: content.updated_at,
            content_type: content.content_type,
            title: content.title,
            overview: content.overview,
            poster: content.poster,
            release_date: content.release_date,
            genres: genres.into_iter().map(|row| row.genre).collect(),
        }))
    }

    pub async fn insert_content(
        &self,
        content: &ContentInput,
    ) -> Result<(Uuid, NaiveDateTime), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let res = sqlx::query!(
            r#"
            insert into contents (provider_id, updated_at, content_type, title, overview, poster, release_date)
            values ($1, $2, $3, $4, $5, $6, $7)
            returning content_id, updated_at"#,
            &content.provider_id.to_string(),
            Utc::now().naive_utc(),
            content.content_type as ContentType,
            &content.title,
            &content.overview,
            content.poster,
            content.release_date
        )
            .fetch_one(&mut *trx)
            .await?;

        for genre in &content.genres {
            sqlx::query!(
                "insert into contents_genres (content_id, genre) values ($1, $2)",
                &res.content_id,
                genre
            )
            .execute(&mut *trx)
            .await?;
        }

        trx.commit().await?;

        Ok((res.content_id, res.updated_at))
    }

    pub async fn update_content(
        &self,
        content: &ContentInput,
    ) -> Result<(Uuid, NaiveDateTime), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let res = sqlx::query!(
            r#"
            update contents
            set updated_at   = $1,
                content_type = $2,
                title        = $3,
                overview     = $4,
                poster       = $5,
                release_date = $6
            where provider_id = $7
            returning content_id, updated_at"#,
            Utc::now().naive_utc(),
            content.content_type as ContentType,
            &content.title,
            &content.overview,
            content.poster,
            content.release_date,
            &content.provider_id.to_string()
        )
        .fetch_one(&mut *trx)
        .await?;

        sqlx::query!(
            "delete from contents_genres where content_id = $1",
            &res.content_id
        )
        .execute(&mut *trx)
        .await?;

        for genre in &content.genres {
            sqlx::query!(
                "insert into contents_genres (content_id, genre) values ($1, $2)",
                &res.content_id,
                genre
            )
            .execute(&mut *trx)
            .await?;
        }

        trx.commit().await?;

        Ok((res.content_id, res.updated_at))
    }

    pub async fn content_exist(&self, content_id: Uuid) -> Result<bool, sqlx::Error> {
        match sqlx::query!(
            r#"
            select content_id
            from contents
            where content_id = $1"#,
            content_id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn friend_request(
        &self,
        friend_request: &FriendInput,
    ) -> Result<FriendRequest, sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let user_id = friend_request.user_id;

        let friend_id = sqlx::query!(
            r#"
            select user_id from users
            where username = $2"#
        )
        .fetch_one(&self.pool)
        .await;

        let res = sqlx::query!(
            r#"
            insert into friend_requests (user_id, friend_asked_id) values ($1, $2)"#,
            &user_id.to_string(),
            &friend_id.to_string()
        )
        .fetch_one(&mut *trx)
        .await;

        trx.commit().await?;

        Ok((res.user_id, res.friend_asked_id))
    }

    pub async fn friend_accept(
        &self,
        friend_request: &FriendInput,
    ) -> Result<Friend, sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let user_id = friend_request.user_id;

        let friend_id = sqlx::query!(
            r#"
            select user_id from users
            where username = $2"#
        )
            .fetch_one(&self.pool)
            .await;

        let res = sqlx::query!(
            r#"
            insert into friends (user_id, friend_id) values ($1, $2)"#,
            &user_id.to_string(),
            &friend_id.to_string()
        )
            .fetch_one(&mut *trx)
            .await;

        trx.commit().await?;

        Ok((res.user_id, res.friend_asked_id))
    }
}
