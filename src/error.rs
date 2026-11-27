//----------importing creates-----------------
use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        FromRequest, FromRequestParts,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::PoisonError;
use thiserror::Error;

//============================= 1. declear error type enums =====================================
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Internal(String),
}

//=============================== 2. Turn errors into HTTP reponses =================================
#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Internal(e) => {
                eprintln!("Internal error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let message = match &self {
            ApiError::Internal(_) => "Internal Server Error".to_string(),
            other => other.to_string(),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

//================================ 3. specific From impl (so ? will work) ==============================
impl From<JsonRejection> for ApiError {
    fn from(r: JsonRejection) -> Self {
        ApiError::BadRequest(r.body_text())
    }
}

impl From<PathRejection> for ApiError {
    fn from(r: PathRejection) -> Self {
        ApiError::BadRequest(r.body_text())
    }
}

impl From<QueryRejection> for ApiError {
    fn from(r: QueryRejection) -> Self {
        ApiError::BadRequest(r.body_text())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}

impl<T> From<PoisonError<T>> for ApiError {
    fn from(_: PoisonError<T>) -> Self {
        ApiError::Internal("lock poisoned".to_string())
    }
}

//============================= 4. Custom extractors ==============================================
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct AppJson<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct AppPath<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct AppQuery<T>(pub T);
