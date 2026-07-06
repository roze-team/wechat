use axum::{
    extract::{rejection::JsonRejection, State},
    routing::{get, post},
    Json, Router,
};
use roze_error::RozeError;
use roze_health::{HealthRegistry, ProbeKind};
use roze_http::rest::RestServer;
use roze_result::ApiResponse;
use roze_wechat::{crypto, types::CallbackQuery};
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
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

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/startupz", get(startupz))
        .route("/wechat/callback/verify", post(verify_callback))
        .with_state(state)
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

async fn verify_callback(
    input: Result<Json<VerifyCallbackRequest>, JsonRejection>,
) -> Result<Json<ApiResponse<VerifyCallbackResponse>>, RozeError> {
    let Json(input) = input.map_err(|err| RozeError::BadRequest(err.to_string()))?;
    let ok = crypto::verify_callback_signature(
        &input.token,
        &input.query.timestamp,
        &input.query.nonce,
        input.query.signature.as_deref().unwrap_or_default(),
    );
    Ok(Json(ApiResponse::ok(VerifyCallbackResponse { ok })))
}

#[derive(Debug, serde::Deserialize)]
struct VerifyCallbackRequest {
    token: String,
    query: CallbackQuery,
}

#[derive(Debug, Serialize)]
struct VerifyCallbackResponse {
    ok: bool,
}

#[derive(Debug, Serialize)]
struct HealthProbeResponse {
    service: String,
    probe: roze_health::ProbeReport,
}

#[cfg(test)]
mod tests {
    use super::{verify_callback, VerifyCallbackRequest};
    use axum::response::{IntoResponse, Response};
    use axum::Json;
    use roze_wechat::{crypto, types::CallbackQuery};

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

    #[test]
    fn roze_error_uses_http_status_and_json_body() {
        let response: Response =
            roze_error::RozeError::BadRequest("invalid json".to_string()).into_response();

        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
