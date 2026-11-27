use crate::error::ApiError;
use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Query},
};

#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
pub struct AppJson<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(Path), rejection(ApiError))]
pub struct AppPath<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(ApiError))]
pub struct AppQuery<T>(pub T);
