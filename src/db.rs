use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub mod users;

pub async fn connect() -> PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .min_connections(5) // keep at least 5 open
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10)) // Neon may be waking up
        .connect(&url)
        .await
        .expect("failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");
    pool
}
