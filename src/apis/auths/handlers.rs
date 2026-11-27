use crate::{db, error::ApiError, extractors::AppJson, state::AppState};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{Json, extract::State, http::StatusCode};
use chrono::NaiveDate;
use rand_core::OsRng;
use serde::Deserialize;

//users registration data types
#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    first_name: String,
    last_name: String,
    phone: Option<String>,
    email: Option<String>,
    date_of_birth: Option<NaiveDate>,
    password: String,
}

// turns "" or "   " into None so empty strings never hit the UNIQUE columns
fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

//######################################################################################################################
///Users registration handler.
pub async fn register_user(
    State(state): State<AppState>,           // extract the shared state
    AppJson(body): AppJson<RegisterRequest>, // extract the JSON body
) -> Result<(StatusCode, Json<db::users::User>), ApiError> {
    let username = body.username.trim().to_lowercase(); // normalize username to lowercase
    let first_name = body.first_name.trim().to_string(); // trim whitespace from first name
    let last_name = body.last_name.trim().to_string(); // trim whitespace from last name
    let phone = clean(body.phone); // clean phone number
    let email = clean(body.email).map(|e| e.to_lowercase()); // clean useremail 

    // ---- validation ----
    if !(3..=30).contains(&username.len())
        || !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(ApiError::BadRequest(
            "username must be 3-30 characters: letters, numbers or underscore".to_string(),
        ));
    }
    if first_name.is_empty() || last_name.is_empty() {
        return Err(ApiError::BadRequest(
            "first name and last name are required".to_string(),
        ));
    }
    if phone.is_none() && email.is_none() {
        return Err(ApiError::BadRequest(
            "provide a phone number or an email".to_string(),
        ));
    }
    if let Some(p) = &phone {
        let valid = p.starts_with('+')
            && (8..=15).contains(&(p.len() - 1))
            && p[1..].chars().all(|c| c.is_ascii_digit());
        if !valid {
            return Err(ApiError::BadRequest(
                "phone must be in international format, e.g. +2348012345678".to_string(),
            ));
        }
    }
    if let Some(e) = &email {
        if !e.contains('@') || e.len() < 5 {
            return Err(ApiError::BadRequest("invalid email address".to_string()));
        }
    }
    if body.password.len() < 8 {
        return Err(ApiError::BadRequest(
            "password must be at least 8 characters".to_string(),
        ));
    }

    // ---- hash the password (CPU heavy, so off the async threads) ----
    let password = body.password;
    let hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
    })
    .await
    .map_err(|_| ApiError::Internal("could not hash password".to_string()))?
    .map_err(|_| ApiError::Internal("could not hash password".to_string()))?;

    // ---- save to the database ----
    match db::users::create(
        &state.db,
        &username,
        &first_name,
        &last_name,
        phone.as_deref(),
        email.as_deref(),
        body.date_of_birth,
        &hash,
    )
    .await
    {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            let msg = match e.constraint() {
                Some("users_phone_key") => "phone number already registered",
                Some("users_email_key") => "email already registered",
                _ => "username already taken",
            };
            Err(ApiError::Conflict(msg.to_string()))
        }
        Err(err) => {
            println!("create user failed: {err}");
            tracing::error!("create user failed: {err}");
            Err(ApiError::Internal("could not create user".to_string()))
        }
    }
}
//#############################################################################################################################

pub async fn login_user() {
    //Impementation for user login
}

pub async fn logout_user() {
    //Implementation for user logout
}

pub async fn username_availabel() {
    //Implimentation for user name avaiabiity
}

pub async fn change_password() {
    //Implimentation of change password
}

pub async fn reset_password() {
    //Implementation of reset password
}
