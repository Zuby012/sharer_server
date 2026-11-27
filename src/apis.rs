mod auths;
mod health;
mod otp;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auths::router())
        .merge(health::router())
        .nest("/api", otp::router())
}
