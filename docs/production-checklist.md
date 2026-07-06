# Production Checklist

- All applications expose `/healthz`, `/readyz`, and `/startupz`.
- All HTTP services expose `/metrics` and record route/status/duration metrics through `roze-metrics`.
- HTTP services load `roze-config::ServiceConfig` from `config.yaml`.
- HTTP services run through `roze-http::rest::RestServer`.
- HTTP responses use `roze-result::ApiResponse`.
- HTTP request parsing errors are mapped to `roze-error::RozeError`.
- Health probes use `roze-health::HealthRegistry` and dependency checks.
- WeChat API calls use explicit timeout and structured error mapping.
- Access tokens and tickets are cached with early refresh.
- High-concurrency token refresh uses `roze-singleflight`.
- Callback verification is tested for Official Account, Mini Program, Work,
  Open Platform, and Open Work.
- Callback XML parsing is exposed behind signature verification before payloads
  are returned to business callers.
- WeChat Pay v3 signing and verification are tested with fixed fixtures.
- Payment notification decryption is exposed behind WeChat Pay v3 RSA
  signature verification before plaintext is returned to business callers.
- Notification decryption is tested before accepting payment callbacks.
- Every typed module has request/response DTO tests.
- CI runs format, tests, and clippy.
- CI builds the production API container image.
- The production API container exposes `/healthz` as its Docker health check.
- Kubernetes manifests mount Roze config from ConfigMap and wire startup,
  liveness, and readiness probes to Roze health endpoints.
