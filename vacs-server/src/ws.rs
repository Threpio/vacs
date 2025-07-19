mod application_message;
mod auth;
mod client;
mod handler;
pub mod message;
#[cfg(test)]
mod test_util;
pub(crate) mod traits;

use crate::state::AppState;
use axum::routing::any;
use axum::Router;
pub use client::ClientSession;
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", any(handler::ws_handler))
}
