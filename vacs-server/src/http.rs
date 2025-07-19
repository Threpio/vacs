use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

mod auth;
pub mod error;
mod ws;

pub type ApiResult<T> = Result<axum::Json<T>, error::AppError>;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/ws", ws::routes())
}
