use crate::db::DbHandler;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct SessionRequest {
    pub owner_id: Uuid,
    pub session_id: Uuid,
    pub owner_username: String
}

impl DbHandler {
    pub async fn get_invitations(&self, user_id: Uuid) -> Result<Vec<SessionRequest>, sqlx::Error> {
        let session_requests = sqlx::query_as!(
            SessionRequest,
            r#"
            select 
                sr.owner_id,
                sr.session_id,
                u.username as owner_username
            from session_requests sr
            join users u ON sr.owner_id = u.user_id
            where sr.user_id = $1
            "#,
            &user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(session_requests)
    }
    pub async fn create_invitation(
        &self,
        owner_id: Uuid,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                insert into session_requests (user_id, owner_id, session_id)
                values ($1, $2, $3)
                "#,
            user_id,
            owner_id,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn delete_session_invitations(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            delete from session_requests where session_id = $1
            "#,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn clean_invitations(&self, ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            delete from session_requests
            "#
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
