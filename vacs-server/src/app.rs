use crate::state::AppState;
use crate::{config, http, ws};
use axum::Router;
use axum_login::{AuthManagerLayer, AuthnBackend};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_sessions::service::SignedCookie;
use tower_sessions::SessionStore;

pub fn create_app<B, S>(auth_layer: AuthManagerLayer<B, S, SignedCookie>) -> Router<Arc<AppState>>
where
    B: AuthnBackend + Send + Sync + 'static + Clone,
    S: SessionStore + Send + Sync + 'static + Clone,
{
    Router::new()
        .merge(http::routes())
        .merge(ws::routes())
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .layer(TimeoutLayer::new(config::SERVER_SHUTDOWN_TIMEOUT))
        .layer(auth_layer)
}

pub async fn serve(listener: TcpListener, app: Router<Arc<AppState>>, state: Arc<AppState>) {
    axum::serve(
        listener,
        app.with_state(state)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
