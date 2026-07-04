# Requirements

This project must implement the full WeChat ecosystem capability surface
represented by PowerWeChat, using Roze conventions for production services.

## Goals

- Provide a Rust-native SDK crate for every supported WeChat product family.
- Provide Roze-style service applications for callbacks, payment notifications,
  Open Platform events, operational health, and internal integration APIs.
- Keep WeChat protocol details out of application code: signing, AES, token
  refresh, retry, error mapping, XML/JSON decoding, and callback verification
  belong in the SDK.
- Keep production behavior uniform: config, tracing, metrics, health checks,
  retries, timeouts, cache, and service governance follow Roze conventions.

## Product Families

- Official Account
- Mini Program
- WeChat Pay
- WeCom
- Open Platform
- WeCom Open Platform
- Channels
- Basic Service

## Module Boundaries

- `client`, `config`, `error`, `crypto`, `token`, and `types` are shared kernel
  modules.
- `modules_impl/basic_service.rs` covers shared WeChat basic capabilities.
- `modules_impl/official_account.rs` covers Official Account capabilities.
- `modules_impl/mini_program.rs` covers Mini Program capabilities.
- `modules_impl/payment.rs` covers WeChat Pay capabilities.
- `modules_impl/work.rs` covers WeCom capabilities.
- `modules_impl/open_platform.rs` covers WeChat Open Platform capabilities.
- `modules_impl/open_work.rs` covers WeCom Open Platform capabilities.
- `modules_impl/channels.rs` covers Channels capabilities.

## Non-Functional Requirements

- Every public API wrapper must have a documented request and response type.
- Security-sensitive protocol code must have deterministic tests.
- Token refresh must avoid cache stampedes and keep the last valid token when a
  refresh fails.
- All callback and payment notification handlers must verify signatures before
  exposing decrypted payloads to business logic.
- HTTP clients must set explicit timeouts and expose traceable errors.
- Production services must expose `/healthz` and `/readyz`.
