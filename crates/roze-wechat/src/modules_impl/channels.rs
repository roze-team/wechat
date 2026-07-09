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

    pub fn store(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "channels.e_commerce.store")
    }

    pub async fn shop_basic_info(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get("channels/ec/basics/info/get", Some(access_token.into()))
            .await
    }

    pub async fn store_basic_info(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ChannelsStoreBasicInfoResponse> {
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

    pub async fn store_shop_qr_code(
        &self,
        access_token: impl Into<String>,
        request: ChannelsStoreShopQrCodeRequest,
    ) -> Result<ChannelsStoreShopQrCodeResponse> {
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
    pub wecom_corp_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wecom_user_id: Option<String>,
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
    #[serde(default)]
    pub shop_qrcode: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsStoreShopQrCodeRequest {
    pub wecom_corp_id: String,
    pub wecom_user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsStoreInfo {
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub headimg_url: Option<String>,
    #[serde(default)]
    pub subject_type: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsStoreBasicInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<ChannelsStoreInfo>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsStoreShopQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub shop_qrcode: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        ChannelsStoreBasicInfoResponse, ChannelsStoreShopQrCodeRequest,
        ChannelsStoreShopQrCodeResponse, ShopQrCodeRequest, ShopQrCodeResponse,
    };

    #[test]
    fn serializes_shop_qr_code_request() {
        let value = serde_json::to_value(ShopQrCodeRequest {
            wecom_corp_id: None,
            wecom_user_id: None,
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

    #[test]
    fn serializes_channels_store_qr_code_request() {
        let value = serde_json::to_value(ChannelsStoreShopQrCodeRequest {
            wecom_corp_id: "ww-corp".to_string(),
            wecom_user_id: "user-id".to_string(),
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "wecom_corp_id": "ww-corp", "wecom_user_id": "user-id" })
        );
    }

    #[test]
    fn deserializes_channels_store_responses() {
        let basic: ChannelsStoreBasicInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "info": {
                "nickname": "Roze Shop",
                "headimg_url": "https://example.com/avatar.png",
                "subject_type": "enterprise",
                "status": "open",
                "username": "gh_xxx"
            }
        }))
        .unwrap();
        let info = basic.info.expect("store info");
        assert_eq!(info.nickname.as_deref(), Some("Roze Shop"));
        assert_eq!(info.username.as_deref(), Some("gh_xxx"));

        let qr: ChannelsStoreShopQrCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "shop_qrcode": "https://example.com/qrcode.png"
        }))
        .unwrap();
        assert_eq!(
            qr.shop_qrcode.as_deref(),
            Some("https://example.com/qrcode.png")
        );

        let compat: ShopQrCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "shop_qrcode": "https://example.com/qrcode.png"
        }))
        .unwrap();
        assert_eq!(
            compat.shop_qrcode.as_deref(),
            Some("https://example.com/qrcode.png")
        );
    }
}
