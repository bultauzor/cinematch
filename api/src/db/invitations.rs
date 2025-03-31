use crate::api::friends::FriendRequest;
use crate::db::DbHandler;
use uuid::Uuid;

impl DbHandler {
    pub async fn get_invitations_friends(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<FriendRequest>, sqlx::Error> {
        let friend_requests = sqlx::query_as!(
            FriendRequest,
            r#"
            SELECT
                fr.user_id,
                fr.friend_asked_id AS friend_id,
                u.username AS user_username,
                u.avatar AS user_avatar
            FROM friend_requests fr
            JOIN users u ON u.user_id = fr.user_id
            WHERE fr.friend_asked_id = $1
            "#,
            &user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(friend_requests)
    }

    // Le friend_id est l'ID du gars qui a envoyé la demande, placé en user_id dans la table friend_requests
    pub async fn accept_invitation(
        &self,
        user_id: Uuid,
        friend_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            r#"insert into friends (user_id, friend_id) values ($1, $2), ($2, $1)"#,
            user_id,
            &friend_id
        )
        .execute(&mut *trx)
        .await?;

        sqlx::query!(
            r#"
            delete from friend_requests where friend_asked_id = $2 and user_id = $1
            "#,
            &friend_id,
            &user_id
        )
        .execute(&self.pool)
        .await?;

        trx.commit().await?;

        Ok(())
    }
    pub async fn refuse_invitation(
        &self,
        user_id: Uuid,
        friend_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            r#"
                delete from friend_requests
                where user_id = $1 and friend_asked_id = $2"#,
            user_id,
            friend_id
        )
        .fetch_one(&mut *trx)
        .await?;

        trx.commit().await?;

        Ok(())
    }
}
