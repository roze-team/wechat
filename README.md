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
- `apps/wechat-api`: Roze REST service for callbacks, health checks, startup
  probes, and operational endpoints. It uses `roze-config` for `config.yaml`,
  `roze-http` for serving, `roze-health` for probes, and `roze-result` for
  response envelopes.
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

Run the API service with:

```bash
cargo run -p wechat-api
```

The service reads `apps/wechat-api/config.yaml` by default and exposes
`/healthz`, `/readyz`, `/startupz`, `/metrics`, and
`/wechat/callback/verify`.

Callback XML can be verified and parsed with:

```http
POST /wechat/callback/parse
Content-Type: application/json

{
  "token": "token",
  "query": {
    "signature": "sha1-signature",
    "timestamp": "1700000000",
    "nonce": "nonce"
  },
  "xml": "<xml><MsgType><![CDATA[text]]></MsgType></xml>"
}
```

WeChat Pay v3 notifications can be verified and decrypted with:

```http
POST /wechat/payment/notify/decrypt
Content-Type: application/json

{
  "headers": {
    "timestamp": "1700000000",
    "nonce": "nonce",
    "signature": "wechatpay-signature",
    "serial": "wechatpay-serial"
  },
  "public_key_pem": "-----BEGIN PUBLIC KEY-----...",
  "api_v3_key": "32-byte-api-v3-key",
  "body": "{\"id\":\"notify-id\",\"resource\":{\"ciphertext\":\"...\"}}"
}
```
