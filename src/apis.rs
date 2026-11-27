mod auths;
mod health;
mod otp;
mod user;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(auths::router())
        .merge(user::router())
        .merge(health::router())
        .nest("/api", otp::router())
}
