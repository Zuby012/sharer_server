mod handlers;

use crate::state::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/otp/request", post(handlers::request_otp))
        .route("/otp/verify", post(handlers::verify_otp))
}
