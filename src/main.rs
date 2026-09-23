use std::sync::Arc;

use crate::settings::SettingsModel;

mod app_ctx;
mod http_server;
mod settings;

#[tokio::main]
async fn main() {
    let settings = SettingsModel::first_load("~/.settings-remote-access")
        .await
        .into();

    let app = Arc::new(crate::app_ctx::AppContext::new(settings));

    crate::http_server::start(&app);

    app.app_states.wait_until_shutdown().await;
}
