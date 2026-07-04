use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    config::Platform,
    error::Result,
    modules::{DomainModule, PlatformClient},
    Client,
};

#[derive(Debug, Clone)]
pub struct Channels {
    inner: PlatformClient,
}

impl Channels {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn e_commerce(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "channels.e_commerce")
    }

    pub async fn shop_basic_info(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get("channels/ec/basics/info/get", Some(access_token.into()))
            .await
    }

    pub async fn shop_qr_code(
        &self,
        access_token: impl Into<String>,
        request: ShopQrCodeRequest,
    ) -> Result<ShopQrCodeResponse> {
        self.inner
            .post(
                "channels/ec/basics/shop/qrcode/get",
                Some(access_token.into()),
                request,
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopQrCodeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub img_url: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::ShopQrCodeRequest;

    #[test]
    fn serializes_shop_qr_code_request() {
        let value = serde_json::to_value(ShopQrCodeRequest {
            page: Some("pages/index".to_string()),
            scene: Some("a=1".to_string()),
            check_path: Some(true),
            env_version: None,
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "page": "pages/index", "scene": "a=1", "check_path": true })
        );
    }
}
