use std::sync::Arc;

use rust_extensions::AppStates;

use crate::settings::SettingsModel;

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: Arc<SettingsModel>,
    /// `ENV_INFO` environment variable. Sent to settings-service as the
    /// `env-info` header on every request. It must not match settings-service
    /// `local_env_prefixes`, so secrets are resolved with their `remote_value`.
    pub env_info: String,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        if let Err(err) = flurl::FlUrl::try_new(settings.settings_service_url.as_str()) {
            panic!(
                "Invalid settings_service_url '{}'. Err: {:?}",
                settings.settings_service_url, err
            );
        }

        let env_info = match std::env::var("ENV_INFO") {
            Ok(env_info) if !env_info.is_empty() => env_info,
            _ => panic!("Environment variable ENV_INFO is not set"),
        };

        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            settings: Arc::new(settings),
            env_info,
        }
    }
}
