use crate::oauth::OAuthClient;
use async_trait::async_trait;
use oauth2::url::Url;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, RefreshToken};
use std::str::FromStr;
use tracing::instrument;

pub struct MockVatsimOAuthClient {}

impl MockVatsimOAuthClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl OAuthClient for MockVatsimOAuthClient {
    #[instrument(level = "debug", skip_all)]
    fn auth_url(&self) -> (Url, CsrfToken) {
        (
            Url::from_str("http://localhost").unwrap(),
            CsrfToken::new("csrf1".to_string()),
        )
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn exchange_code(
        &self,
        code: String,
    ) -> anyhow::Result<(String, String)> {
        if code != "authorization_code1" {
            anyhow::bail!("Invalid authorization code");
        }

        Ok((
            "access_token1".to_string(),
            "refresh_token1".to_string(),
        ))
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> anyhow::Result<(AccessToken, RefreshToken)> {
        if refresh_token.secret() != "refresh_token1" {
            anyhow::bail!("Invalid refresh token");
        }

        Ok((
            AccessToken::new("access_token2".to_string()),
            RefreshToken::new("refresh_token2".to_string()),
        ))
    }
}
