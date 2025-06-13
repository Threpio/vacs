use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    pub input: AudioDeviceConfig,
    pub output: AudioDeviceConfig,
}

#[derive(Debug, Deserialize)]
pub struct AudioDeviceConfig {
    pub host_name: Option<String>,
    pub device_name: Option<String>,
    pub channels: u16,
}
