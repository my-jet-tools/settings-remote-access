use std::time::Duration;

use serde::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub http_port: Option<u16>,
    /// Where settings-service is reached from this data-center.
    /// `https://domain@ip:port` pins the connection to `ip:port` while `domain`
    /// stays the Host / TLS SNI. A plain `https://domain` works as well.
    pub settings_service_url: String,
    pub request_timeout_sec: Option<u64>,
}

impl SettingsModel {
    pub fn get_http_port(&self) -> u16 {
        match self.http_port {
            Some(port) => port,
            None => 8000,
        }
    }

    pub fn get_request_timeout(&self) -> Duration {
        match self.request_timeout_sec {
            Some(sec) => Duration::from_secs(sec),
            None => Duration::from_secs(10),
        }
    }
}
