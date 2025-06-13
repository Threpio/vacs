use serde::Deserialize;
use vacs_audio::config::AudioConfig;
use vacs_webrtc::config::WebrtcConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub audio: AudioConfig,
    pub webrtc: WebrtcConfig,
}
