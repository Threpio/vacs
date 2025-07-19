pub mod error;
pub mod session;

pub type ApiResult<T> = Result<axum::Json<T>, error::AppError>;
