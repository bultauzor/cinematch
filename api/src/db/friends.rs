use crate::api::friends::{Friend, FriendRequest};
use crate::db::DbHandler;
use uuid::Uuid;

impl DbHandler {
    pub async fn get_all_friends(&self, user_id: Uuid) -> Result<Vec<Friend>, sqlx::Error> {
        let res = sqlx::query_as!(
            Friend,
            "SELECT user_id, friend_id FROM friends WHERE user_id = $1",
            &user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }

    pub async fn create_invitation(
        &self,
        user_id: Uuid,
        user_username: String,
        username: String,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        let friend_id = sqlx::query!(
            r#"
                select user_id from users
                where username = $1"#,
            &username
        )
        .fetch_one(&self.pool)
        .await?;

        // FAIRE UN CAS OU L'INVITATION DE L'AUTRE MEMBRE EST DEJA CREEE ET AJOUTER AUTOMATIQUEMENT LES USERS EN AMIS

        let res = sqlx::query!(
            r#"
                insert into friend_requests (user_id, friend_asked_id) values ($1, $2)
                returning user_id, friend_asked_id"#,
            &user_id,
            friend_id.user_id
        )
        .fetch_one(&mut *trx)
        .await?;

        trx.commit().await?;

        Ok(())
    }

    pub async fn delete_friend(&self, user_id: Uuid, friend_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                delete from friends
                where (user_id = $1 and friend_id = $2) or (user_id = $2 and friend_id = $1)"#,
            user_id,
            friend_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
