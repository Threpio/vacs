use serde::Deserialize;

pub(crate) const WEBRTC_TRACK_ID: &str = "audio";
pub(crate) const WEBRTC_TRACK_STREAM_ID: &str = "main";

#[derive(Debug, Deserialize)]
pub struct WebrtcConfig {
    pub ice_servers: Vec<String>,
}
