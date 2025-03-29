impl DbHandler {
    pub async fn get_invitations(&self, user_id: Uuid) -> Result<Vec<SessionRequest>, sqlx::Error> {
        let session_requests = sqlx::query_as!(
            SessionRequest,
            r#"
            SELECT
                sr.owner_id,
                sr.session_id AS session_id,
            FROM session_requests sr
            WHERE sr.user_id = $1
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
        user_username: String,
        session_id: Uuid
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let user_id = sqlx::query!(
            r#"
                select user_id from users
                where username = $1"#,
            &user_username
        )
            .fetch_one(&self.pool)
            .await?;

        let res = sqlx::query!(
            r#"
                insert into session_requests (user_id, owner_id, session_id) values ($1, $2, $3)
                returning user_id, session_id"#,
            &user_id,
            owner_id,
            session_id
        )
            .fetch_one(&mut *trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }
    pub async fn accept_invitation(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            delete from session_requests where user_id = $1 && session_id = $2
            "#,
            &user_id,
            session_id
        )
            .execute(&self.pool)
            .await?;

        trx.commit().await?;

        Ok(())
    }
    pub async fn refuse_invitation(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            delete from session_requests where user_id = $1 && session_id = $2
            "#,
            &user_id,
            session_id
        )
            .fetch_one(&mut *trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }
}