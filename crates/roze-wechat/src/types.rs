use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatApiEnvelope<T> {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StableAccessTokenRequest {
    pub grant_type: String,
    pub appid: String,
    pub secret: String,
    #[serde(default)]
    pub force_refresh: bool,
}

impl StableAccessTokenRequest {
    pub fn client_credential(
        appid: impl Into<String>,
        secret: impl Into<String>,
        force_refresh: bool,
    ) -> Self {
        Self {
            grant_type: "client_credential".to_string(),
            appid: appid.into(),
            secret: secret.into(),
            force_refresh,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StableAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub signature: Option<String>,
    pub msg_signature: Option<String>,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{StableAccessTokenRequest, StableAccessTokenResponse};

    #[test]
    fn serializes_stable_access_token_request() {
        let value = serde_json::to_value(StableAccessTokenRequest::client_credential(
            "appid", "secret", true,
        ))
        .unwrap();

        assert_eq!(
            value,
            json!({
                "grant_type": "client_credential",
                "appid": "appid",
                "secret": "secret",
                "force_refresh": true
            })
        );
    }

    #[test]
    fn deserializes_stable_access_token_response() {
        let response: StableAccessTokenResponse =
            serde_json::from_value(json!({ "access_token": "token", "expires_in": 7200 })).unwrap();

        assert_eq!(response.access_token.as_deref(), Some("token"));
        assert_eq!(response.expires_in, Some(7200));
    }
}
