use crate::state::AppState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get::health))
}

mod get {
    pub async fn health() -> &'static str {
        "OK"
    }
}
