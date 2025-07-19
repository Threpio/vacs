pub mod connect;
pub mod mock;

use async_trait::async_trait;
use oauth2::{url::Url, AccessToken, AuthorizationCode, CsrfToken, RefreshToken};

#[async_trait]
pub trait OAuthClient: Send + Sync {
    fn auth_url(&self) -> (Url, CsrfToken);
    async fn exchange_code(
        &self,
        code: String,
    ) -> anyhow::Result<(String, String)>;
    async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> anyhow::Result<(AccessToken, RefreshToken)>;
}
