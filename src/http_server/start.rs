use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::app_ctx::AppContext;

use super::SettingsProxyMiddleware;

pub fn start(app: &Arc<AppContext>) {
    let http_port = app.settings.get_http_port();
    let listen_addr = SocketAddr::from(([0, 0, 0, 0], http_port));

    println!("Listening http server: Tcp({})", listen_addr);
    let mut http_server = MyHttpServer::new(listen_addr);

    http_server.add_middleware(Arc::new(SettingsProxyMiddleware::new(app.clone())));

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
