use crate::api::friends::Friend;
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

    pub async fn create_invitation_friend(
        &self,
        user_id: Uuid,
        username: String,
    ) -> Result<(), sqlx::Error> {
        let mut trx = self.pool.begin().await?;

        // Getting user_id for the person asked to be friend by username
        let friend_id = sqlx::query!(
            r#"
                select user_id from users
                where username = $1"#,
            &username
        )
        .fetch_one(&self.pool)
        .await?;

        // Checking if a friend request has already been made by the person asked to be friend
        let result = sqlx::query!(
            r#"
            select user_id, friend_asked_id from friend_requests where (user_id = $2 AND friend_asked_id = $1)
            "#,
            &user_id,
            friend_id.user_id

        )
            .fetch_optional(&self.pool)
            .await?;

        // If the result is none that means there is no existing request
        if result.is_none() {
            // Creating a friend_request
            sqlx::query!(
                r#"
                insert into friend_requests (user_id, friend_asked_id) values ($1, $2)"#,
                &user_id,
                friend_id.user_id
            )
            .execute(&mut *trx)
            .await?;

            trx.commit().await?;

        // If the result is not none, that means there is an existing request
        } else {
            // Adding the 2 users as friends
            sqlx::query!(
                r#"insert into friends (user_id, friend_id) values ($1, $2), ($2, $1)"#,
                &user_id,
                friend_id.user_id
            )
            .execute(&mut *trx)
            .await?;

            // Deleting the request that already exists (made by the person asked to be friend)
            sqlx::query!(
                r#"
            delete from friend_requests where friend_asked_id = $1
            "#,
                &user_id
            )
            .execute(&self.pool)
            .await?;
            trx.commit().await?;
        }

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
