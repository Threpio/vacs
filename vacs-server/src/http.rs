use axum::http::StatusCode;

pub mod error;
pub mod session;

pub type ApiResult<T> = Result<axum::Json<T>, error::AppError>;
pub type StatusCodeResult = Result<StatusCode, error::AppError>;
