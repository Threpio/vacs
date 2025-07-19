pub mod app;
pub mod auth;
pub mod config;
pub mod state;
#[cfg(feature = "test-utils")]
pub mod test_utils;
pub mod ws;
pub mod session;
pub mod http;
mod middlewares;
mod users;

/// User-Agent string used for all HTTP requests.
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));