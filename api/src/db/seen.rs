use uuid::Uuid;

use crate::model::{
    content::{ContentType, ContentView},
    seen::{SeenContent, SeenContentInput},
};

use super::DbHandler;

impl DbHandler {
    pub async fn get_seen_content_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<SeenContent>, sqlx::Error> {
        let content = sqlx::query!(
            r#"
            select c.content_id,
                   c.provider_id,
                   c.content_type as "content_type: ContentType",
                   c.title,
                   c.overview,
                   c.poster,
                   c.release_date,
                   s.grade
            from contents_seen as s
            join contents as c on s.content_id=c.content_id
            where s.user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await;

        let contents = match content {
            Ok(content) => content,
            Err(sqlx::Error::RowNotFound) => return Ok(Vec::with_capacity(0)),
            e => e?,
        };

        let mut seen_contents = Vec::with_capacity(contents.len());
        for c in contents {
            let genres = sqlx::query!(
                "select genre from contents_genres where content_id = $1;",
                c.content_id
            )
            .fetch_all(&self.pool)
            .await?;

            seen_contents.push(SeenContent {
                content: ContentView {
                    content_id: c.content_id,
                    content_type: c.content_type,
                    title: c.title,
                    overview: c.overview,
                    poster: c.poster,
                    release_date: c.release_date,
                    genres: genres.into_iter().map(|row| row.genre).collect(),
                },
                grade: c.grade,
            });
        }

        Ok(seen_contents)
    }

    pub async fn insert_seen_content(
        &self,
        seen_content: SeenContentInput,
        user_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            insert into contents_seen (content_id, user_id, grade)
            values ($1, $2, $3)
            "#,
            seen_content.content_id,
            user_id,
            seen_content.grade
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }

    pub async fn delete_seen_content(
        &self,
        content_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        sqlx::query!(
            r#"
            delete from contents_seen 
            where content_id = $1  and user_id= $2
            returning content_id"#,
            content_id,
            user_id,
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.content_id)
    }

    pub async fn update_seen_content_grade(
        &self,
        grade: Option<f64>,
        content_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            update contents_seen
            set grade = $1
            where content_id = $2 and user_id = $3"#,
            grade,
            content_id,
            user_id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
}
