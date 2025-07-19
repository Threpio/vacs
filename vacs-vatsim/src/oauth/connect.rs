use crate::oauth::OAuthClient;
use anyhow::Context;
use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet,
    EndpointSet, RedirectUrl, RefreshToken, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Clone, Debug)]
pub struct OAuthConfig {
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            auth_url: "https://auth-dev.vatsim.net/oauth/authorize".to_string(),
            token_url: "https://auth-dev.vatsim.net/oauth/token".to_string(),
            redirect_url: "vacs://auth/callback".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
        }
    }
}

pub struct ConnectOAuthClient {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    http_client: reqwest::Client,
}

impl ConnectOAuthClient {
    pub fn new(config: OAuthConfig) -> anyhow::Result<Self> {
        let client = BasicClient::new(ClientId::new(config.client_id))
            .set_client_secret(ClientSecret::new(config.client_secret))
            .set_auth_uri(AuthUrl::new(config.auth_url).context("Invalid auth URL")?)
            .set_token_uri(TokenUrl::new(config.token_url).context("Invalid token URL")?)
            .set_redirect_uri(
                RedirectUrl::new(config.redirect_url).context("Invalid redirect URL")?,
            );

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(crate::APP_USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            http_client,
        })
    }
}

#[async_trait]
impl OAuthClient for ConnectOAuthClient {
    #[instrument(level = "debug", skip_all)]
    fn auth_url(&self) -> (Url, CsrfToken) {
        tracing::trace!("Generating VATSIM OAuth2 URL");

        let (url, csrf_token) = self.client.authorize_url(CsrfToken::new_random).url();
        (url, csrf_token)
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn exchange_code(
        &self,
        code: String,
    ) -> anyhow::Result<(String, String)> {
        tracing::trace!("Exchanging OAuth2 code for token");

        let response = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&self.http_client)
            .await
            .context("Failed to exchange OAuth2 code for token")?;

        if response.refresh_token().is_none() {
            tracing::warn!("No refresh token received");
            anyhow::bail!("No refresh token received");
        }

        Ok((
            response.access_token().secret().clone(),
            response.refresh_token().unwrap().secret().clone(),
        ))
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> anyhow::Result<(AccessToken, RefreshToken)> {
        tracing::trace!("Refreshing OAuth2 token");
        let response = self
            .client
            .exchange_refresh_token(refresh_token)
            .request_async(&self.http_client)
            .await
            .context("Failed to refresh OAuth2 token")?;

        if response.refresh_token().is_none() {
            tracing::warn!("No refresh token received");
            anyhow::bail!("No refresh token received");
        }

        Ok((
            response.access_token().clone(),
            response.refresh_token().unwrap().clone(),
        ))
    }
}
