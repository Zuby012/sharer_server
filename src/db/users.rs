use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

pub async fn create(
    pool: &PgPool,
    username: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    email: Option<&str>,
    date_of_birth: Option<NaiveDate>,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"INSERT INTO users (username, first_name, last_name, phone, email, date_of_birth, password_hash)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, username, first_name, last_name, phone, email,
                     date_of_birth, phone_verified, email_verified, created_at"#,
        username, first_name, last_name, phone, email, date_of_birth, password_hash
    )
    .fetch_one(pool)
    .await
}
