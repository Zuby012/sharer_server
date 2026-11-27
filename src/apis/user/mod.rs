mod handlers;

use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/register", post(handlers::create_user))
        .route("/user/login", post(handlers::login_users))
        .route("/user/{id}", delete(handlers::delete_user))
        .route("/user/{id}", put(handlers::update_user))
        .route("/user/{id}", get(handlers::get_user))
}
