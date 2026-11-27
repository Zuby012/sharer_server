mod handlers;
use crate::state::AppState;

use axum::{
    Router,
    routing::{get, post, put},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
        .route("/logout", post(handlers::logout_user))
        .route("/username-available", get(handlers::username_availabel))
        .route("/password/change", put(handlers::change_password))
        .route("/password/reset", put(handlers::reset_password))
}
