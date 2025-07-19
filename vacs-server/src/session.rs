use crate::config::AppConfig;
use anyhow::Context;
use tower_sessions::cookie::{time, Key, SameSite};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, SessionStore};
use tower_sessions::service::SignedCookie;
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};
use tracing::instrument;

#[instrument(level = "info", skip_all, err)]
pub async fn setup_redis_connection_pool(config: &AppConfig) -> anyhow::Result<Pool> {
    tracing::trace!("Creating Redis pool");
    let pool_config = Config::from_url_centralized(&config.redis.addr)
        .context("Failed to create redis pool config")?;
    let pool =
        Pool::new(pool_config, None, None, None, 6).context("Failed to create redis pool")?;

    tracing::trace!("Connecting to Redis");
    pool.connect();
    pool.wait_for_connect()
        .await
        .context("Failed to connect to redis")?;

    tracing::info!("Redis connection pool created");
    Ok(pool)
}

#[instrument(level = "info", skip_all, err)]
pub async fn setup_redis_session_manager(
    config: &AppConfig,
    redis_pool: Pool,
) -> anyhow::Result<SessionManagerLayer<RedisStore<Pool>, SignedCookie>> {
    let session_store = RedisStore::new(redis_pool);
    Ok(configure_session_layer(config, session_store))
}

#[instrument(level = "info", skip_all, err)]
pub async fn setup_memory_session_manager(
    config: &AppConfig,
) -> anyhow::Result<SessionManagerLayer<MemoryStore, SignedCookie>> {
    let session_store = MemoryStore::default();
    Ok(configure_session_layer(config, session_store))
}

#[instrument(level = "info", skip_all)]
fn configure_session_layer<S>(config: &AppConfig, session_store: S) -> SessionManagerLayer<S, SignedCookie>
where
    S: SessionStore + Send + Sync + 'static + Clone,
{
    tracing::trace!("Configuring session manager");
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(config.session.secure)
        .with_http_only(config.session.http_only)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(
            config.session.expiry_secs,
        )))
        .with_same_site(SameSite::Lax) // Required for session cookie during OAuth redirect
        .with_signed(Key::from(config.session.signing_key.as_bytes()));

    tracing::info!("Session manager configured");

    session_layer
}
