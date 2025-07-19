use anyhow::Context;
use config::{Config, Environment, File};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::watch;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vacs_server::app::create_app;
use vacs_server::auth::setup_auth_layer;
use vacs_server::config::AppConfig;
use vacs_server::session::{setup_redis_connection_pool, setup_redis_session_manager};
use vacs_server::state::AppState;
use vacs_vatsim::oauth::connect::ConnectOAuthClient;
use vacs_vatsim::user::connect::ConnectUserService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=trace,tower_http=debug,tower_sessions=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = load_config()?;
    
    let redis_pool = setup_redis_connection_pool(&config).await?;

    let (shutdown_tx, shutdown_rx) = watch::channel(());

    let app_state = Arc::new(AppState::new(
        config.clone(),
        redis_pool.clone(),
        shutdown_rx.clone(),
    ));

    let auth_layer = setup_auth_layer(&config, redis_pool).await?;
    
    let app = create_app(auth_layer);

    let listener = tokio::net::TcpListener::bind(config.server.bind_addr).await?;

    tracing::info!(bind_addr = ?listener.local_addr()?, "Started listening");
    axum::serve(
        listener,
        app.with_state(app_state)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(shutdown_tx))
    .await?;

    Ok(())
}

fn load_config() -> anyhow::Result<AppConfig> {
    Config::builder()
        .set_default("server.bind_addr", "127.0.0.1:3000")?
        .set_default("redis.addr", "redis://127.0.0.1:6379")?
        .set_default("session.secure", true)?
        .set_default("session.http_only", true)?
        .set_default("session.expiry_secs", 604800)?
        .set_default("auth.login_flow_timeout_millis", 10000)?
        .set_default("auth.oauth.auth_url", "https://auth-dev.vatsim.net/oauth/authorize")?
        .set_default("auth.oauth.token_url", "https://auth-dev.vatsim.net/oauth/token")?
        .set_default("auth.oauth.redirect_url", "vacs://auth/callback")?
        .set_default(
            "vatsim.user_service.user_details_endpoint_url",
            "https://auth-dev.vatsim.net/api/user",
        )?
        .add_source(
            File::with_name(
                directories::ProjectDirs::from("app", "vacs", "vacs-server")
                    .expect("Failed to get project dirs")
                    .config_local_dir()
                    .join("config.toml")
                    .to_str()
                    .expect("Failed to get local config path"),
            )
            .required(false),
        )
        .add_source(File::with_name("config.toml").required(false))
        .add_source(Environment::with_prefix("vacs_server"))
        .build()
        .context("Failed to build config")?
        .try_deserialize()
        .context("Failed to deserialize config")
}

async fn shutdown_signal(shutdown_tx: watch::Sender<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("Shutdown signal received, terminating gracefully...");

    shutdown_tx
        .send(())
        .expect("Failed to send shutdown signal");
}
