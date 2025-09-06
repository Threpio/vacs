pub mod file;

use crate::http::error::AppError;
use semver::Version;
use serde::{Deserialize, Serialize};
use vacs_protocol::http::version::ReleaseChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseMeta {
    pub version: Version,
    pub channel: ReleaseChannel,
    pub required: bool,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub target: String,
    pub arch: String,
    pub url: String,
    pub signature: Option<String>,
}

pub trait Catalog: Send + Sync + 'static {
    fn list(&self, channel: ReleaseChannel) -> Result<Vec<ReleaseMeta>, AppError>;
}
