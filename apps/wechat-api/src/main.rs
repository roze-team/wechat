use axum::{
    extract::{rejection::JsonRejection, MatchedPath, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use roze_error::RozeError;
use roze_health::{HealthRegistry, ProbeKind};
use roze_http::rest::RestServer;
use roze_result::ApiResponse;
use roze_wechat::{
    crypto,
    types::{CallbackMessage, CallbackQuery},
};
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Debug, Clone)]
struct AppState {
    service_name: Arc<str>,
    health: HealthRegistry,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config()?;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let rest = config
        .rest
        .clone()
        .ok_or_else(|| anyhow::anyhow!("missing rest config"))?;
    let health = HealthRegistry::new();
    health.register_static(roze_health::HealthCheck::healthy("wechat-sdk"));
    health.mark_ready();

    let state = AppState {
        service_name: Arc::from(config.name.as_str()),
        health,
    };
    let metrics_state = state.clone();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/startupz", get(startupz))
        .route("/metrics", get(metrics))
        .route("/wechat/callback/verify", post(verify_callback))
        .route("/wechat/callback/parse", post(parse_callback))
        .with_state(state)
        .layer(middleware::from_fn(move |req: Request, next: Next| {
            let state = metrics_state.clone();
            async move { record_metrics(state, req, next).await }
        }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    tracing::info!(service = %config.name, addr = %rest.addr, "starting Roze REST service");
    RestServer::new(rest.addr, app).serve().await?;
    Ok(())
}

fn load_config() -> anyhow::Result<roze_config::ServiceConfig> {
    Ok(roze_config::load(config_path())?)
}

fn config_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_config = manifest_dir.join("config.yaml");
    if manifest_config.exists() {
        manifest_config
    } else {
        PathBuf::from("config.yaml")
    }
}

async fn healthz(State(state): State<AppState>) -> Json<ApiResponse<HealthProbeResponse>> {
    let report = state.health.liveness_report().await;
    Json(ApiResponse::ok(HealthProbeResponse {
        service: state.service_name.to_string(),
        probe: report.probe(ProbeKind::Liveness),
    }))
}

async fn readyz(State(state): State<AppState>) -> Json<ApiResponse<HealthProbeResponse>> {
    let report = state.health.readiness_report().await;
    Json(ApiResponse::ok(HealthProbeResponse {
        service: state.service_name.to_string(),
        probe: report.probe(ProbeKind::Readiness),
    }))
}

async fn startupz(State(state): State<AppState>) -> Json<ApiResponse<HealthProbeResponse>> {
    let report = state.health.startup_report().await;
    Json(ApiResponse::ok(HealthProbeResponse {
        service: state.service_name.to_string(),
        probe: report.probe(ProbeKind::Startup),
    }))
}

async fn metrics() -> String {
    roze_metrics::http_metrics()
}

async fn record_metrics(state: AppState, req: Request, next: Next) -> Response {
    let started = Instant::now();
    let method = req.method().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());

    let response = next.run(req).await;
    record_request_metrics(
        &state.service_name,
        &route,
        &method,
        response.status(),
        started.elapsed(),
    );
    response
}

fn record_request_metrics(
    service_name: &str,
    route: &str,
    method: &str,
    status: StatusCode,
    elapsed: Duration,
) {
    roze_metrics::record_http_request(status.is_success(), elapsed);
    roze_metrics::record_http_route(
        service_name.to_string(),
        route.to_string(),
        method.to_string(),
        status.as_u16().to_string(),
        elapsed,
    );
}

async fn verify_callback(
    input: Result<Json<VerifyCallbackRequest>, JsonRejection>,
) -> Result<Json<ApiResponse<VerifyCallbackResponse>>, RozeError> {
    let Json(input) = input.map_err(|err| RozeError::BadRequest(err.to_string()))?;
    let ok = verify_callback_query(&input.token, &input.query);
    Ok(Json(ApiResponse::ok(VerifyCallbackResponse { ok })))
}

async fn parse_callback(
    input: Result<Json<ParseCallbackRequest>, JsonRejection>,
) -> Result<Json<ApiResponse<ParseCallbackResponse>>, RozeError> {
    let Json(input) = input.map_err(|err| RozeError::BadRequest(err.to_string()))?;
    if !verify_callback_query(&input.token, &input.query) {
        return Err(RozeError::Unauthorized);
    }

    let message = CallbackMessage::parse_xml(&input.xml)
        .map_err(|err| RozeError::BadRequest(err.to_string()))?;
    Ok(Json(ApiResponse::ok(ParseCallbackResponse {
        signature_ok: true,
        encrypted: message.is_encrypted(),
        message,
    })))
}

fn verify_callback_query(token: &str, query: &CallbackQuery) -> bool {
    crypto::verify_callback_signature(
        token,
        &query.timestamp,
        &query.nonce,
        query.signature.as_deref().unwrap_or_default(),
    )
}

#[derive(Debug, serde::Deserialize)]
struct VerifyCallbackRequest {
    token: String,
    query: CallbackQuery,
}

#[derive(Debug, serde::Deserialize)]
struct ParseCallbackRequest {
    token: String,
    query: CallbackQuery,
    xml: String,
}

#[derive(Debug, Serialize)]
struct VerifyCallbackResponse {
    ok: bool,
}

#[derive(Debug, Serialize)]
struct ParseCallbackResponse {
    signature_ok: bool,
    encrypted: bool,
    message: CallbackMessage,
}

#[derive(Debug, Serialize)]
struct HealthProbeResponse {
    service: String,
    probe: roze_health::ProbeReport,
}

#[cfg(test)]
mod tests {
    use super::{
        metrics, parse_callback, verify_callback, ParseCallbackRequest, VerifyCallbackRequest,
    };
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::Json;
    use roze_wechat::{crypto, types::CallbackQuery};
    use std::time::Duration;

    #[test]
    fn loads_roze_service_config() {
        let config = super::load_config().expect("config should parse");

        assert_eq!(config.name, "wechat-api");
        assert_eq!(
            config.rest.expect("rest config").addr.to_string(),
            "0.0.0.0:8080"
        );
    }

    #[tokio::test]
    async fn verify_callback_returns_roze_api_response() {
        let timestamp = "1700000000";
        let nonce = "nonce";
        let token = "token";
        let signature = crypto::sha1_signature(&[token, timestamp, nonce]);
        let response = verify_callback(Ok(Json(VerifyCallbackRequest {
            token: token.to_string(),
            query: CallbackQuery {
                signature: Some(signature),
                msg_signature: None,
                timestamp: timestamp.to_string(),
                nonce: nonce.to_string(),
                echostr: None,
            },
        })))
        .await
        .expect("handler should succeed")
        .0;

        assert_eq!(response.code, 0);
        assert!(response.data.expect("data").ok);
    }

    #[tokio::test]
    async fn parse_callback_verifies_and_returns_message() {
        let timestamp = "1700000000";
        let nonce = "nonce";
        let token = "token";
        let signature = crypto::sha1_signature(&[token, timestamp, nonce]);
        let response = parse_callback(Ok(Json(ParseCallbackRequest {
            token: token.to_string(),
            query: CallbackQuery {
                signature: Some(signature),
                msg_signature: None,
                timestamp: timestamp.to_string(),
                nonce: nonce.to_string(),
                echostr: None,
            },
            xml: r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[text]]></MsgType>
                <Content><![CDATA[hello]]></Content>
            </xml>"#
                .to_string(),
        })))
        .await
        .expect("handler should succeed")
        .0;

        let data = response.data.expect("data");
        assert!(data.signature_ok);
        assert!(!data.encrypted);
        assert_eq!(data.message.msg_type.as_deref(), Some("text"));
        assert_eq!(data.message.content.as_deref(), Some("hello"));
    }

    #[tokio::test]
    async fn parse_callback_rejects_invalid_signature() {
        let err = parse_callback(Ok(Json(ParseCallbackRequest {
            token: "token".to_string(),
            query: CallbackQuery {
                signature: Some("bad".to_string()),
                msg_signature: None,
                timestamp: "1700000000".to_string(),
                nonce: "nonce".to_string(),
                echostr: None,
            },
            xml: "<xml><MsgType><![CDATA[text]]></MsgType></xml>".to_string(),
        })))
        .await
        .expect_err("invalid signature should be rejected");

        assert_eq!(err, roze_error::RozeError::Unauthorized);
    }

    #[test]
    fn roze_error_uses_http_status_and_json_body() {
        let response: Response =
            roze_error::RozeError::BadRequest("invalid json".to_string()).into_response();

        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn metrics_returns_roze_http_metrics() {
        let body = metrics().await;

        assert!(body.contains("roze_http_requests_total"));
        assert!(body.contains("roze_http_requests_failed_total"));
    }

    #[test]
    fn records_roze_http_route_metrics() {
        super::record_request_metrics(
            "wechat-api",
            "/wechat/callback/verify",
            "POST",
            StatusCode::OK,
            Duration::from_millis(5),
        );

        let body = roze_metrics::http_metrics();

        assert!(body.contains("roze_http_route_requests_total"));
        assert!(body.contains(r#"service="wechat-api""#));
        assert!(body.contains(r#"route="/wechat/callback/verify""#));
        assert!(body.contains(r#"method="POST""#));
        assert!(body.contains(r#"status="200""#));
    }
}
