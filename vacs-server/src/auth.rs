use crate::config::AppConfig;
use crate::session::setup_redis_session_manager;
use crate::users::Backend;
use anyhow::Context;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use tower_sessions::service::SignedCookie;
use tower_sessions_redis_store::fred::prelude::Pool;
use tower_sessions_redis_store::RedisStore;
use tracing::instrument;

#[instrument(level = "debug", skip_all, err)]
pub async fn setup_auth_layer(
    config: &AppConfig,
    redis_pool: Pool,
) -> anyhow::Result<AuthManagerLayer<Backend, RedisStore<Pool>, SignedCookie>> {
    tracing::debug!("Setting up authentication layer");
    
    let client = BasicClient::new(ClientId::new(config.auth.oauth.client_id.clone()))
        .set_client_secret(ClientSecret::new(config.auth.oauth.client_secret.clone()))
        .set_auth_uri(AuthUrl::new(config.auth.oauth.auth_url.clone()).context("Invalid auth URL")?)
        .set_token_uri(
            TokenUrl::new(config.auth.oauth.token_url.clone()).context("Invalid token URL")?,
        )
        .set_redirect_uri(
            RedirectUrl::new(config.auth.oauth.redirect_url.clone())
                .context("Invalid redirect URL")?,
        );
    let backend = Backend::new(
        client.into(),
        config.vatsim.user_service.user_details_endpoint_url.clone(),
    )?;

    let session_layer = setup_redis_session_manager(&config, redis_pool).await?;

    tracing::debug!("Authentication layer setup complete");
    Ok(AuthManagerLayerBuilder::new(backend, session_layer).build())
}
