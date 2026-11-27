use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db: crate::db::connect().await,
        }
    }
}
