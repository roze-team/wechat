# Roze WeChat

Roze WeChat is a Rust-native WeChat ecosystem SDK and service framework.
It uses PowerWeChat as the capability reference and Roze as the production
service foundation.

## Scope

The target is full coverage for:

- Official Account
- Mini Program
- WeChat Pay
- WeCom
- Open Platform
- WeCom Open Platform
- Basic shared services such as media, QR codes, JSSDK, URL generation,
  callback verification, AES decryption, signing, token management, retries,
  metrics, health checks, and production configuration.

See [docs/coverage-matrix.md](docs/coverage-matrix.md) for the implementation
matrix.

## Layout

- `crates/roze-wechat`: core SDK crate.
- `crates/roze-wechat/src/modules_impl`: product modules split by WeChat
  product family and capability area.
- `apps/wechat-api`: Roze-style HTTP service for callbacks, health checks, and
  operational endpoints.
- `docs`: requirements, module mapping, and production checklist.

## Module Entry

```rust
use roze_wechat::{Client, Wechat, WechatConfig};

let client = Client::new(WechatConfig::default())?;
let wechat = Wechat::new(client);
let menu = wechat.official_account().menu();
let payment_notify = wechat.payment().notify();
```

## Verification

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
