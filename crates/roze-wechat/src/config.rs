use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    OfficialAccount,
    MiniProgram,
    Payment,
    Work,
    OpenPlatform,
    OpenWork,
    Channels,
    BasicService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub platform: Platform,
    pub app_id: String,
    pub secret: Option<String>,
    pub token: Option<String>,
    pub aes_key: Option<String>,
    pub mch_id: Option<String>,
    pub serial_no: Option<String>,
    pub private_key_pem: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatConfig {
    pub base_url: String,
    pub timeout_ms: u64,
    pub retry_attempts: usize,
    pub token_refresh_skew_seconds: i64,
    pub apps: Vec<AppConfig>,
}

impl Default for WechatConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.weixin.qq.com".to_string(),
            timeout_ms: 10_000,
            retry_attempts: 2,
            token_refresh_skew_seconds: 300,
            apps: Vec::new(),
        }
    }
}

impl WechatConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms)
    }
}
