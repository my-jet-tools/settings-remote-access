use std::sync::Arc;

use flurl::FlUrl;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpResponseHeaders,
    HttpServerMiddleware, WebContentType,
};
use rust_extensions::str_utils::StrUtils;

use crate::app_ctx::AppContext;

const BAD_GATEWAY_STATUS_CODE: u16 = 502;

/// Serves `GET /settings/{product}/{template}` by fetching the same path from
/// settings-service with this data-center's `env-info` header. Anything else is
/// not handled — the admin API, UI and MCP of settings-service are not exposed
/// here.
pub struct SettingsProxyMiddleware {
    app: Arc<AppContext>,
}

impl SettingsProxyMiddleware {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for SettingsProxyMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        if ctx.request.method.as_str() != "GET" {
            return None;
        }

        let path = ctx.request.get_path();

        let mut product_id = None;
        let mut template_id = None;

        for (no, segment) in path.as_str().split('/').enumerate() {
            match no {
                0 => {}
                1 => {
                    if !segment.eq_case_insensitive("settings") {
                        return None;
                    }
                }
                2 => {
                    product_id = Some(segment);
                }
                3 => {
                    template_id = Some(segment);
                }
                _ => {
                    return None;
                }
            }
        }

        let Some(product_id) = product_id else {
            return None;
        };

        let Some(template_id) = template_id else {
            return None;
        };

        Some(get_from_settings_service(&self.app, product_id, template_id).await)
    }
}

async fn get_from_settings_service(
    app: &AppContext,
    product_id: &str,
    template_id: &str,
) -> Result<HttpOkResult, HttpFailResult> {
    let response = FlUrl::new(app.settings.settings_service_url.as_str())
        .append_path_segment("settings")
        .append_path_segment(product_id)
        .append_path_segment(template_id)
        .with_header("env-info", app.env_info.as_str())
        .set_timeout(app.settings.get_request_timeout())
        .set_response_body_timeout(app.settings.get_request_timeout())
        .get()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(err) => return Err(bad_gateway(product_id, template_id, format!("{:?}", err))),
    };

    let status_code = response.get_status_code();

    let content_type = response
        .get_header_case_insensitive("content-type")
        .ok()
        .flatten()
        .map(|v| WebContentType::Raw(v.to_string()));

    let content = match response.receive_body().await {
        Ok(content) => content,
        Err(err) => return Err(bad_gateway(product_id, template_id, format!("{:?}", err))),
    };

    HttpOutput::Content {
        status_code,
        headers: HttpResponseHeaders::new(content_type),
        content,
    }
    .into_ok_result(false)
}

fn bad_gateway(product_id: &str, template_id: &str, err: String) -> HttpFailResult {
    my_logger::LOGGER.write_error(
        "SettingsProxyMiddleware",
        format!("Can not get settings from settings-service. Err: {}", err),
        my_logger::LogEventCtx::new()
            .add("productId", product_id)
            .add("templateId", template_id),
    );

    HttpOutput::Content {
        status_code: BAD_GATEWAY_STATUS_CODE,
        headers: HttpResponseHeaders::new(Some(WebContentType::Text)),
        content: b"Can not get settings from settings-service".to_vec(),
    }
    .into_http_fail_result(false, false)
}
