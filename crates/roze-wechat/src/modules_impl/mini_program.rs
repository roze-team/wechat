use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    crypto,
    error::Result,
    modules::{DomainModule, PlatformClient},
    types::{StableAccessTokenRequest, StableAccessTokenResponse},
    Client,
};

#[derive(Debug, Clone)]
pub struct MiniProgram {
    inner: PlatformClient,
}

impl MiniProgram {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn auth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.auth")
    }

    pub async fn code2_session(
        &self,
        request: Code2SessionRequest,
    ) -> Result<Code2SessionResponse> {
        self.inner
            .get_with_query(
                "sns/jscode2session",
                None,
                vec![
                    ("appid".to_string(), request.app_id),
                    ("secret".to_string(), request.secret),
                    ("js_code".to_string(), request.js_code),
                    ("grant_type".to_string(), "authorization_code".to_string()),
                ],
            )
            .await
    }

    pub async fn check_session(
        &self,
        app_id: impl Into<String>,
        open_id: impl Into<String>,
        session_key: impl AsRef<[u8]>,
    ) -> Result<WechatStatusResponse> {
        let signature = crypto::hmac_sha256_hex(session_key.as_ref(), b"")?;
        self.inner
            .get_with_query(
                "wxa/checksession",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("openid".to_string(), open_id.into()),
                    ("signature".to_string(), signature),
                    ("sig_method".to_string(), "hmac_sha256".to_string()),
                ],
            )
            .await
    }

    pub async fn reset_user_session_key(
        &self,
        app_id: impl Into<String>,
        open_id: impl Into<String>,
        session_key: impl AsRef<[u8]>,
    ) -> Result<WechatStatusResponse> {
        let signature = crypto::hmac_sha256_hex(session_key.as_ref(), b"")?;
        self.inner
            .get_with_query(
                "wxa/resetusersessionkey",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("openid".to_string(), open_id.into()),
                    ("signature".to_string(), signature),
                    ("sig_method".to_string(), "hmac_sha256".to_string()),
                ],
            )
            .await
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.base")
    }

    pub async fn stable_access_token(
        &self,
        request: StableAccessTokenRequest,
    ) -> Result<StableAccessTokenResponse> {
        self.inner.post("cgi-bin/stable_token", None, request).await
    }

    pub fn customer_service_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.customer_service_message")
    }

    pub async fn send_customer_service_message(
        &self,
        access_token: impl Into<String>,
        request: CustomerServiceMessage,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/custom/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_customer_typing(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        typing: bool,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/custom/typing",
                Some(access_token.into()),
                json!({
                    "touser": openid.into(),
                    "command": if typing { "Typing" } else { "CancelTyping" },
                }),
            )
            .await
    }

    pub fn data_cube(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.data_cube")
    }

    pub async fn daily_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiddailyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn weekly_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidweeklyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn monthly_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidmonthlyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn daily_retain_info(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiddailyretaininfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn visit_page(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidvisitpage",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn user_portrait(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiduserportrait",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn live_broadcast(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.live_broadcast")
    }

    pub async fn create_live_room(
        &self,
        access_token: impl Into<String>,
        request: LiveRoomRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxaapi/broadcast/room/create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn live_info(
        &self,
        access_token: impl Into<String>,
        request: LiveInfoRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxa/business/getliveinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn live_replay(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        start: i64,
        limit: i64,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxa/business/getliveinfo",
                Some(access_token.into()),
                json!({
                    "action": "get_replay",
                    "room_id": room_id,
                    "start": start,
                    "limit": limit,
                }),
            )
            .await
    }

    pub async fn delete_live_room(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxaapi/broadcast/room/deleteroom",
                Some(access_token.into()),
                json!({ "id": room_id }),
            )
            .await
    }

    pub fn phone_number(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.phone_number")
    }

    pub async fn get_user_phone_number(
        &self,
        access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<PhoneNumberResponse> {
        self.inner
            .post(
                "wxa/business/getuserphonenumber",
                Some(access_token.into()),
                json!({ "code": code.into() }),
            )
            .await
    }

    pub fn security(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.security")
    }

    pub async fn security_msg_sec_check(
        &self,
        access_token: impl Into<String>,
        request: SecurityMsgSecCheckRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/msg_sec_check", Some(access_token.into()), request)
            .await
    }

    pub async fn security_media_check_async(
        &self,
        access_token: impl Into<String>,
        request: SecurityMediaCheckAsyncRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/media_check_async", Some(access_token.into()), request)
            .await
    }

    pub fn messages(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.messages")
    }

    pub async fn send_subscribe_message(
        &self,
        access_token: impl Into<String>,
        request: SubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/subscribe/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn url(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.url")
    }

    pub async fn generate_url_scheme(
        &self,
        access_token: impl Into<String>,
        request: UrlSchemeGenerateRequest,
    ) -> Result<UrlSchemeResponse> {
        self.inner
            .post("wxa/generatescheme", Some(access_token.into()), request)
            .await
    }

    pub async fn generate_url_link(
        &self,
        access_token: impl Into<String>,
        request: UrlLinkGenerateRequest,
    ) -> Result<UrlLinkResponse> {
        self.inner
            .post("wxa/generate_urllink", Some(access_token.into()), request)
            .await
    }

    pub async fn generate_short_link(
        &self,
        access_token: impl Into<String>,
        page_url: impl Into<String>,
        page_title: impl Into<String>,
        is_permanent: bool,
    ) -> Result<ShortLinkResponse> {
        self.inner
            .post(
                "wxa/genwxashortlink",
                Some(access_token.into()),
                json!({
                    "page_url": page_url.into(),
                    "page_title": page_title.into(),
                    "is_permanent": is_permanent,
                }),
            )
            .await
    }

    pub fn wxa_code(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.wxa_code")
    }

    pub async fn create_qr_code(
        &self,
        access_token: impl Into<String>,
        request: CreateQrCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/wxaapp/createwxaqrcode",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn create_qr_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: CreateQrCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "cgi-bin/wxaapp/createwxaqrcode",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }

    pub async fn get_wxa_code(
        &self,
        access_token: impl Into<String>,
        request: GetWxaCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/getwxacode", Some(access_token.into()), request)
            .await
    }

    pub async fn get_wxa_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: GetWxaCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "wxa/getwxacode",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }

    pub async fn get_unlimited_wxa_code(
        &self,
        access_token: impl Into<String>,
        request: GetUnlimitedWxaCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/getwxacodeunlimit", Some(access_token.into()), request)
            .await
    }

    pub async fn get_unlimited_wxa_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: GetUnlimitedWxaCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "wxa/getwxacodeunlimit",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code2SessionRequest {
    pub app_id: String,
    pub secret: String,
    pub js_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code2SessionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub session_key: Option<String>,
    #[serde(default)]
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub phone_info: Option<PhoneInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneInfo {
    pub phone_number: String,
    pub pure_phone_number: String,
    pub country_code: String,
    #[serde(default)]
    pub watermark: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessage {
    pub touser: String,
    pub msgtype: String,
    #[serde(flatten)]
    pub payload: Value,
}

impl CustomerServiceMessage {
    pub fn text(touser: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            touser: touser.into(),
            msgtype: "text".to_string(),
            payload: json!({ "text": { "content": content.into() } }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeDateRange {
    pub begin_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMsgSecCheckRequest {
    pub content: String,
    #[serde(default = "default_security_scene")]
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

fn default_security_scene() -> i64 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMediaCheckAsyncRequest {
    pub media_url: String,
    pub media_type: i64,
    #[serde(default = "default_security_scene")]
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRoomRequest {
    pub name: String,
    #[serde(rename = "coverImg")]
    pub cover_img: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "anchorName")]
    pub anchor_name: String,
    #[serde(rename = "anchorWechat")]
    pub anchor_wechat: String,
    #[serde(rename = "shareImg")]
    pub share_img: String,
    #[serde(rename = "type")]
    pub type_id: i64,
    #[serde(rename = "screenType")]
    pub screen_type: i64,
    #[serde(rename = "closeLike")]
    pub close_like: i64,
    #[serde(rename = "closeGoods")]
    pub close_goods: i64,
    #[serde(rename = "closeComment")]
    pub close_comment: i64,
    #[serde(rename = "feedsImg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeds_img: Option<String>,
    #[serde(rename = "closeReplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_replay: Option<i64>,
    #[serde(rename = "closeShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_share: Option<i64>,
    #[serde(rename = "closeKf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_kf: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInfoRequest {
    pub start: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlSchemeGenerateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_wxa: Option<JumpWxa>,
    #[serde(default)]
    pub is_expire: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_interval: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlLinkGenerateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
    #[serde(default)]
    pub is_expire: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_base: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpWxa {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlSchemeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openlink: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrCodeRequest {
    pub path: String,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWxaCodeRequest {
    pub path: String,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
    #[serde(default)]
    pub auto_color: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Value>,
    #[serde(default)]
    pub is_hyaline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnlimitedWxaCodeRequest {
    pub scene: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default)]
    pub check_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
    #[serde(default)]
    pub auto_color: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Value>,
    #[serde(default)]
    pub is_hyaline: bool,
}

fn default_wxa_code_width() -> i64 {
    430
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        Code2SessionResponse, CreateQrCodeRequest, CustomerServiceMessage, DataCubeDateRange,
        JumpWxa, LiveInfoRequest, LiveRoomRequest, PhoneNumberResponse, SecurityMsgSecCheckRequest,
        SubscribeMessageRequest, UrlSchemeGenerateRequest,
    };

    #[test]
    fn serializes_subscribe_message_request() {
        let value = serde_json::to_value(SubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "tpl".to_string(),
            data: json!({ "thing1": { "value": "hello" } }),
            page: Some("pages/index".to_string()),
            miniprogram_state: Some("formal".to_string()),
            lang: Some("zh_CN".to_string()),
        })
        .unwrap();

        assert_eq!(value["template_id"], "tpl");
        assert_eq!(value["data"]["thing1"]["value"], "hello");
    }

    #[test]
    fn serializes_qr_code_default_width() {
        let value = serde_json::to_value(CreateQrCodeRequest {
            path: "pages/index".to_string(),
            width: 430,
        })
        .unwrap();

        assert_eq!(value, json!({ "path": "pages/index", "width": 430 }));
    }

    #[test]
    fn serializes_url_scheme_request() {
        let value = serde_json::to_value(UrlSchemeGenerateRequest {
            jump_wxa: Some(JumpWxa {
                path: Some("pages/index".to_string()),
                query: Some("a=1".to_string()),
                env_version: Some("release".to_string()),
            }),
            is_expire: true,
            expire_time: Some(1_800_000_000),
            expire_type: None,
            expire_interval: None,
        })
        .unwrap();

        assert_eq!(value["jump_wxa"]["path"], "pages/index");
        assert_eq!(value["is_expire"], true);
    }

    #[test]
    fn serializes_customer_service_text_message() {
        let value = serde_json::to_value(CustomerServiceMessage::text("openid", "hello")).unwrap();

        assert_eq!(value["touser"], "openid");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");
    }

    #[test]
    fn serializes_data_cube_date_range() {
        let value = serde_json::to_value(DataCubeDateRange {
            begin_date: "20260704".to_string(),
            end_date: "20260704".to_string(),
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "begin_date": "20260704", "end_date": "20260704" })
        );
    }

    #[test]
    fn serializes_security_msg_check_default_scene() {
        let value = serde_json::to_value(SecurityMsgSecCheckRequest {
            content: "hello".to_string(),
            scene: 1,
            version: Some(2),
            openid: Some("openid".to_string()),
        })
        .unwrap();

        assert_eq!(value["scene"], 1);
        assert_eq!(value["version"], 2);
        assert_eq!(value["openid"], "openid");
    }

    #[test]
    fn deserializes_code2_session_response() {
        let response: Code2SessionResponse = serde_json::from_value(json!({
            "openid": "openid",
            "session_key": "session-key",
            "unionid": "unionid"
        }))
        .unwrap();

        assert_eq!(response.openid.as_deref(), Some("openid"));
        assert_eq!(response.session_key.as_deref(), Some("session-key"));
        assert_eq!(response.unionid.as_deref(), Some("unionid"));
    }

    #[test]
    fn deserializes_phone_number_response() {
        let response: PhoneNumberResponse = serde_json::from_value(json!({
            "phone_info": {
                "phone_number": "+8613800000000",
                "pure_phone_number": "13800000000",
                "country_code": "86",
                "watermark": { "appid": "wxappid" }
            }
        }))
        .unwrap();
        let phone_info = response.phone_info.expect("phone_info");

        assert_eq!(phone_info.phone_number, "+8613800000000");
        assert_eq!(phone_info.pure_phone_number, "13800000000");
        assert_eq!(phone_info.country_code, "86");
        assert_eq!(phone_info.watermark.expect("watermark")["appid"], "wxappid");
    }

    #[test]
    fn serializes_live_room_request() {
        let value = serde_json::to_value(LiveRoomRequest {
            name: "launch".to_string(),
            cover_img: "media-cover".to_string(),
            start_time: 1_800_000_000,
            end_time: 1_800_003_600,
            anchor_name: "host".to_string(),
            anchor_wechat: "host-wechat".to_string(),
            share_img: "media-share".to_string(),
            type_id: 1,
            screen_type: 0,
            close_like: 0,
            close_goods: 0,
            close_comment: 0,
            feeds_img: None,
            close_replay: Some(1),
            close_share: None,
            close_kf: None,
        })
        .unwrap();

        assert_eq!(value["name"], "launch");
        assert_eq!(value["coverImg"], "media-cover");
        assert_eq!(value["startTime"], 1_800_000_000);
        assert_eq!(value["endTime"], 1_800_003_600);
        assert_eq!(value["anchorName"], "host");
        assert_eq!(value["anchorWechat"], "host-wechat");
        assert_eq!(value["shareImg"], "media-share");
        assert_eq!(value["type"], 1);
        assert_eq!(value["screenType"], 0);
        assert_eq!(value["closeLike"], 0);
        assert_eq!(value["closeGoods"], 0);
        assert_eq!(value["closeComment"], 0);
        assert_eq!(value["closeReplay"], 1);
        assert!(value.get("feedsImg").is_none());
    }

    #[test]
    fn serializes_live_info_request() {
        let value = serde_json::to_value(LiveInfoRequest {
            start: 0,
            limit: 20,
        })
        .unwrap();

        assert_eq!(value, json!({ "start": 0, "limit": 20 }));
    }
}
