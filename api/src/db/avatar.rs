use uuid::Uuid;

use super::DbHandler;

impl DbHandler {
    pub async fn update_avatar(&self, user_id: &Uuid, avatar: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            update users
            set avatar = $1
            where user_id = $2"#,
            avatar,
            user_id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }

    pub async fn get_avatar(&self, user_id: Uuid) -> Result<Option<String>, sqlx::Error> {
        sqlx::query!(
            r#"
            select avatar
            from users
            where user_id = $1"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.avatar)
    }
}
