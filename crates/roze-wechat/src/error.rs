use thiserror::Error;

pub type Result<T> = std::result::Result<T, WechatError>;

#[derive(Debug, Error)]
pub enum WechatError {
    #[error("wechat api error {code}: {message}")]
    Api { code: i64, message: String },

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("xml error: {0}")]
    Xml(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("token error: {0}")]
    Token(String),
}
