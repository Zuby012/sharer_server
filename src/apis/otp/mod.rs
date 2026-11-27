mod handlers;

use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/otp/request", post(handlers::request_otp))
        .route("/otp/verify", get(handlers::verify_otp))
}
