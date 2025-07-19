use serde::Deserialize;
use std::time::Duration;
use tower_sessions::cookie::SameSite;
use vacs_vatsim::oauth::connect::OAuthConfig;

pub const BROADCAST_CHANNEL_CAPACITY: usize = 100;
pub const CLIENT_CHANNEL_CAPACITY: usize = 100;
pub const CLIENT_WEBSOCKET_RECEIVE_CHANNEL_CAPACITY: usize = 100;
pub const SERVER_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub auth: AuthConfig,
    pub vatsim: VatsimConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:3000".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub addr: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            addr: "redis://127.0.0.1:6379".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SessionConfig {
    pub secure: bool,
    pub http_only: bool,
    pub expiry_secs: i64,
    pub signing_key: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            secure: true,
            http_only: true,
            expiry_secs: 604800, // 7 days
            signing_key: "super-sikrit-signing-key".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub login_flow_timeout_millis: u64,
    pub oauth: OAuthConfig,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            login_flow_timeout_millis: 10000,
            oauth: OAuthConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VatsimConfig {
    pub user_service: VatsimUserServiceConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VatsimUserServiceConfig {
    pub user_details_endpoint_url: String,
}

impl Default for VatsimUserServiceConfig {
    fn default() -> Self {
        Self {
            user_details_endpoint_url: "https://auth-dev.vatsim.net/api/user".to_string(),
        }
    }
}
