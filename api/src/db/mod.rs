pub mod content;
pub mod friends;
mod invitations;
pub mod seen;
pub mod session;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct DbHandler {
    pool: PgPool,
}

impl DbHandler {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect(uri: &str) -> Option<Self> {
        PgPoolOptions::new().connect(uri).await.map(Self::new).ok()
    }
}
