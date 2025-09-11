pub mod file;

use crate::http::error::AppError;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum BundleType {
    #[default]
    Unknown,
    AppImage,
    Deb,
    Rpm,
    App,
    Msi,
    Nsis,
}

impl FromStr for BundleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "unknown" => Ok(BundleType::Unknown),
            "appimage" => Ok(BundleType::AppImage),
            "deb" => Ok(BundleType::Deb),
            "rpm" => Ok(BundleType::Rpm),
            "app" => Ok(BundleType::App),
            "msi" => Ok(BundleType::Msi),
            "nsis" => Ok(BundleType::Nsis),
            _ => Err(format!("unknown bundle type {}", s)),
        }
    }
}

impl TryFrom<&str> for BundleType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for BundleType {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub target: String,
    pub arch: String,
    pub bundle_type: BundleType,
    pub url: String,
    pub signature: Option<String>,
}

pub trait Catalog: Send + Sync + 'static {
    fn list(&self, channel: ReleaseChannel) -> Result<Vec<ReleaseMeta>, AppError>;
}
