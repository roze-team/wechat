# Production Checklist

- All applications expose `/healthz`, `/readyz`, and `/startupz`.
- HTTP services load `roze-config::ServiceConfig` from `config.yaml`.
- HTTP services run through `roze-http::rest::RestServer`.
- HTTP responses use `roze-result::ApiResponse`.
- Health probes use `roze-health::HealthRegistry` and dependency checks.
- WeChat API calls use explicit timeout and structured error mapping.
- Access tokens and tickets are cached with early refresh.
- High-concurrency token refresh uses singleflight before production launch.
- Callback verification is tested for Official Account, Mini Program, Work,
  Open Platform, and Open Work.
- WeChat Pay v3 signing and verification are tested with fixed fixtures.
- Notification decryption is tested before accepting payment callbacks.
- Every typed module has request/response DTO tests.
- CI runs format, tests, and clippy.
