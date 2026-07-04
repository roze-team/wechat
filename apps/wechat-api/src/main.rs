use axum::{
    routing::{get, post},
    Json, Router,
};
use roze_wechat::{crypto, types::CallbackQuery};
use serde::Serialize;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Debug, Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/wechat/callback/verify", post(verify_callback))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("wechat-api listening on 0.0.0.0:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn healthz() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "wechat-api",
    })
}

async fn readyz() -> Json<Health> {
    Json(Health {
        status: "ready",
        service: "wechat-api",
    })
}

async fn verify_callback(Json(input): Json<VerifyCallbackRequest>) -> Json<VerifyCallbackResponse> {
    let ok = crypto::verify_callback_signature(
        &input.token,
        &input.query.timestamp,
        &input.query.nonce,
        input.query.signature.as_deref().unwrap_or_default(),
    );
    Json(VerifyCallbackResponse { ok })
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
