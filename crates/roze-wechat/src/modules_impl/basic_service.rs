use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::Result,
    modules::{DomainModule, PlatformClient},
    Client,
};

#[derive(Debug, Clone)]
pub struct BasicService {
    inner: PlatformClient,
}

impl BasicService {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn content_security(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.content_security")
    }

    pub async fn msg_sec_check(
        &self,
        access_token: impl Into<String>,
        request: MsgSecCheckRequest,
    ) -> Result<SecurityCheckResponse> {
        self.inner
            .post("wxa/msg_sec_check", Some(access_token.into()), request)
            .await
    }

    pub async fn media_check_async(
        &self,
        access_token: impl Into<String>,
        request: MediaCheckAsyncRequest,
    ) -> Result<SecurityCheckResponse> {
        self.inner
            .post("wxa/media_check_async", Some(access_token.into()), request)
            .await
    }

    pub fn jssdk(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.jssdk")
    }

    pub async fn ticket(
        &self,
        access_token: impl Into<String>,
        ticket_type: impl Into<String>,
    ) -> Result<TicketResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/ticket/getticket",
                Some(access_token.into()),
                vec![("type".to_string(), ticket_type.into())],
            )
            .await
    }

    pub async fn jsapi_ticket(&self, access_token: impl Into<String>) -> Result<TicketResponse> {
        self.ticket(access_token, "jsapi").await
    }

    pub async fn wx_card_ticket(&self, access_token: impl Into<String>) -> Result<TicketResponse> {
        self.ticket(access_token, "wx_card").await
    }

    pub fn media(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.media")
    }

    pub async fn upload_temp_media_from_bytes(
        &self,
        access_token: impl Into<String>,
        media_type: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<TempMediaResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/media/upload",
                Some(access_token.into()),
                vec![("type".to_string(), media_type.into())],
                form,
            )
            .await
    }

    pub async fn upload_image_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MediaUrlResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/media/uploadimg",
                Some(access_token.into()),
                Vec::new(),
                form,
            )
            .await
    }

    pub async fn get_temp_media(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .get_bytes(
                "cgi-bin/media/get",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
            )
            .await
    }

    pub async fn get_jssdk_media(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .get_bytes(
                "cgi-bin/media/get/jssdk",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
            )
            .await
    }

    pub fn qr_code(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.qr_code")
    }

    pub async fn create_qr_code(
        &self,
        access_token: impl Into<String>,
        request: QrCodeCreateRequest,
    ) -> Result<QrCodeCreateResponse> {
        self.inner
            .post("cgi-bin/qrcode/create", Some(access_token.into()), request)
            .await
    }

    pub async fn temporary_qr_code(
        &self,
        access_token: impl Into<String>,
        scene: QrScene,
        expire_seconds: i64,
    ) -> Result<QrCodeCreateResponse> {
        let action_name = match &scene {
            QrScene::Id(_) => "QR_SCENE",
            QrScene::Str(_) => "QR_STR_SCENE",
        };
        self.create_qr_code(
            access_token,
            QrCodeCreateRequest {
                expire_seconds: Some(expire_seconds.min(30 * 86_400)),
                action_name: action_name.to_string(),
                action_info: QrActionInfo {
                    scene: scene.into_value(),
                },
            },
        )
        .await
    }

    pub async fn forever_qr_code(
        &self,
        access_token: impl Into<String>,
        scene: QrScene,
    ) -> Result<QrCodeCreateResponse> {
        let action_name = match &scene {
            QrScene::Id(_) => "QR_LIMIT_SCENE",
            QrScene::Str(_) => "QR_LIMIT_STR_SCENE",
        };
        self.create_qr_code(
            access_token,
            QrCodeCreateRequest {
                expire_seconds: None,
                action_name: action_name.to_string(),
                action_info: QrActionInfo {
                    scene: scene.into_value(),
                },
            },
        )
        .await
    }

    pub fn qr_code_url(ticket: impl AsRef<str>) -> String {
        format!(
            "https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}",
            url::form_urlencoded::byte_serialize(ticket.as_ref().as_bytes()).collect::<String>()
        )
    }

    pub fn subscribe_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.subscribe_message")
    }

    pub async fn send_subscribe_message(
        &self,
        access_token: impl Into<String>,
        request: SubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/subscribe/bizsend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn send_mini_program_subscribe_message(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramSubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/subscribe/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_subscribe_template(
        &self,
        access_token: impl Into<String>,
        tid: impl Into<String>,
        kid_list: Vec<i64>,
        scene_desc: impl Into<String>,
    ) -> Result<SubscribeTemplateAddResponse> {
        self.inner
            .post(
                "wxaapi/newtmpl/addtemplate",
                Some(access_token.into()),
                json!({
                    "tid": tid.into(),
                    "kidList": kid_list,
                    "sceneDesc": scene_desc.into(),
                }),
            )
            .await
    }

    pub async fn delete_subscribe_template(
        &self,
        access_token: impl Into<String>,
        pri_tmpl_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxaapi/newtmpl/deltemplate",
                Some(access_token.into()),
                json!({ "priTmplId": pri_tmpl_id.into() }),
            )
            .await
    }

    pub async fn subscribe_template_categories(
        &self,
        access_token: impl Into<String>,
    ) -> Result<SubscribeTemplateCategoryResponse> {
        self.inner
            .get("wxaapi/newtmpl/getcategory", Some(access_token.into()))
            .await
    }

    pub async fn subscribe_template_keywords(
        &self,
        access_token: impl Into<String>,
        tid: impl Into<String>,
    ) -> Result<SubscribeTemplateKeywordResponse> {
        self.inner
            .get_with_query(
                "wxaapi/newtmpl/getpubtemplatekeywords",
                Some(access_token.into()),
                vec![("tid".to_string(), tid.into())],
            )
            .await
    }

    pub async fn subscribe_template_titles(
        &self,
        access_token: impl Into<String>,
        ids: impl Into<String>,
        start: i64,
        limit: i64,
    ) -> Result<SubscribeTemplateTitleResponse> {
        self.inner
            .get_with_query(
                "wxaapi/newtmpl/getpubtemplatetitles",
                Some(access_token.into()),
                vec![
                    ("ids".to_string(), ids.into()),
                    ("start".to_string(), start.to_string()),
                    ("limit".to_string(), limit.to_string()),
                ],
            )
            .await
    }

    pub async fn subscribe_templates(
        &self,
        access_token: impl Into<String>,
    ) -> Result<SubscribeTemplateListResponse> {
        self.inner
            .get("wxaapi/newtmpl/gettemplate", Some(access_token.into()))
            .await
    }

    pub fn url(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "basic_service.url")
    }

    pub async fn short_key(
        &self,
        access_token: impl Into<String>,
        long_data: impl Into<String>,
        expire_seconds: i64,
    ) -> Result<ShortKeyGenerateResponse> {
        self.generate_short_key(
            access_token,
            ShortKeyGenerateRequest {
                long_data: long_data.into(),
                expire_seconds,
            },
        )
        .await
    }

    pub async fn generate_short_key(
        &self,
        access_token: impl Into<String>,
        request: ShortKeyGenerateRequest,
    ) -> Result<ShortKeyGenerateResponse> {
        self.inner
            .post(
                "cgi-bin/shorten/gen",
                Some(access_token.into()),
                request.with_max_expire_seconds(),
            )
            .await
    }

    pub async fn fetch_short_key(
        &self,
        access_token: impl Into<String>,
        short_key: impl Into<String>,
    ) -> Result<ShortKeyFetchResponse> {
        self.fetch_short_key_data(
            access_token,
            ShortKeyFetchRequest {
                short_key: short_key.into(),
            },
        )
        .await
    }

    pub async fn fetch_short_key_data(
        &self,
        access_token: impl Into<String>,
        request: ShortKeyFetchRequest,
    ) -> Result<ShortKeyFetchResponse> {
        self.inner
            .post("cgi-bin/shorten/fetch", Some(access_token.into()), request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgSecCheckRequest {
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
pub struct MediaCheckAsyncRequest {
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
pub struct SecurityCheckResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
    #[serde(default)]
    pub result: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempMediaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "type")]
    pub media_type: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_seconds: Option<i64>,
    pub action_name: String,
    pub action_info: QrActionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrActionInfo {
    pub scene: Value,
}

#[derive(Debug, Clone)]
pub enum QrScene {
    Id(i64),
    Str(String),
}

impl QrScene {
    fn into_value(self) -> Value {
        match self {
            Self::Id(id) => json!({ "scene_id": id }),
            Self::Str(value) => json!({ "scene_str": value }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub expire_seconds: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    pub url: String,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramSubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTemplateAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "priTmplId")]
    pub pri_tmpl_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTemplateCategoryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTemplateKeywordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTemplateTitleResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTemplateListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortKeyGenerateRequest {
    pub long_data: String,
    pub expire_seconds: i64,
}

impl ShortKeyGenerateRequest {
    fn with_max_expire_seconds(mut self) -> Self {
        self.expire_seconds = self.expire_seconds.min(30 * 86_400);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortKeyGenerateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub short_key: Option<String>,
    #[serde(default)]
    pub expire_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortKeyFetchRequest {
    pub short_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortKeyFetchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub long_data: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub expire_seconds: Option<i64>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        BasicService, MediaCheckAsyncRequest, MiniProgramSubscribeMessageRequest,
        MsgSecCheckRequest, QrActionInfo, QrCodeCreateRequest, ShortKeyFetchRequest,
        ShortKeyFetchResponse, ShortKeyGenerateRequest, ShortKeyGenerateResponse,
        SubscribeMessageRequest, SubscribeTemplateAddResponse, SubscribeTemplateCategoryResponse,
        SubscribeTemplateKeywordResponse, SubscribeTemplateListResponse,
        SubscribeTemplateTitleResponse, TempMediaResponse,
    };

    #[test]
    fn builds_qr_code_url_with_escaped_ticket() {
        let url = BasicService::qr_code_url("abc+/=");
        assert_eq!(
            url,
            "https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket=abc%2B%2F%3D"
        );
    }

    #[test]
    fn serializes_qr_code_request() {
        let value = serde_json::to_value(QrCodeCreateRequest {
            expire_seconds: Some(60),
            action_name: "QR_STR_SCENE".to_string(),
            action_info: QrActionInfo {
                scene: json!({ "scene_str": "abc" }),
            },
        })
        .unwrap();

        assert_eq!(value["action_name"], "QR_STR_SCENE");
        assert_eq!(value["action_info"]["scene"]["scene_str"], "abc");
    }

    #[test]
    fn deserializes_temp_media_type_field() {
        let response: TempMediaResponse =
            serde_json::from_value(json!({ "type": "image", "media_id": "mid" })).unwrap();

        assert_eq!(response.media_type.as_deref(), Some("image"));
        assert_eq!(response.media_id.as_deref(), Some("mid"));
    }

    #[test]
    fn serializes_subscribe_message_request() {
        let value = serde_json::to_value(SubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "tpl".to_string(),
            url: "https://example.com".to_string(),
            data: json!({ "thing1": { "value": "hello" } }),
            miniprogram: None,
            scene: Some("scene".to_string()),
            title: None,
        })
        .unwrap();

        assert_eq!(value["touser"], "openid");
        assert_eq!(value["data"]["thing1"]["value"], "hello");
        assert!(value.get("miniprogram").is_none());
    }

    #[test]
    fn serializes_mini_program_subscribe_message_request() {
        let value = serde_json::to_value(MiniProgramSubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "tpl".to_string(),
            page: Some("pages/index".to_string()),
            miniprogram_state: Some("trial".to_string()),
            lang: Some("zh_CN".to_string()),
            data: json!({ "thing1": { "value": "hello" } }),
        })
        .unwrap();

        assert_eq!(value["touser"], "openid");
        assert_eq!(value["page"], "pages/index");
        assert_eq!(value["miniprogram_state"], "trial");
        assert_eq!(value["data"]["thing1"]["value"], "hello");
    }

    #[test]
    fn deserializes_subscribe_template_responses() {
        let add: SubscribeTemplateAddResponse =
            serde_json::from_value(json!({ "priTmplId": "private-template" })).unwrap();
        assert_eq!(add.pri_tmpl_id.as_deref(), Some("private-template"));

        let categories: SubscribeTemplateCategoryResponse = serde_json::from_value(json!({
            "data": [{ "id": 1, "name": "commerce" }]
        }))
        .unwrap();
        assert_eq!(categories.data[0]["name"], "commerce");

        let keywords: SubscribeTemplateKeywordResponse = serde_json::from_value(json!({
            "data": [{ "kid": 1, "name": "thing1" }]
        }))
        .unwrap();
        assert_eq!(keywords.data[0]["kid"], 1);

        let titles: SubscribeTemplateTitleResponse = serde_json::from_value(json!({
            "count": 1,
            "data": [{ "tid": "tid", "title": "notify" }]
        }))
        .unwrap();
        assert_eq!(titles.count, Some(1));
        assert_eq!(titles.data[0]["title"], "notify");

        let templates: SubscribeTemplateListResponse = serde_json::from_value(json!({
            "data": [{ "priTmplId": "private-template", "title": "notify" }]
        }))
        .unwrap();
        assert_eq!(templates.data[0]["priTmplId"], "private-template");
    }

    #[test]
    fn serializes_msg_sec_check_request() {
        let value = serde_json::to_value(MsgSecCheckRequest {
            content: "hello".to_string(),
            scene: 1,
            version: Some(2),
            openid: Some("openid".to_string()),
        })
        .unwrap();

        assert_eq!(value["content"], "hello");
        assert_eq!(value["scene"], 1);
        assert_eq!(value["version"], 2);
        assert_eq!(value["openid"], "openid");
    }

    #[test]
    fn serializes_media_check_async_request() {
        let value = serde_json::to_value(MediaCheckAsyncRequest {
            media_url: "https://example.com/image.png".to_string(),
            media_type: 2,
            scene: 1,
            version: None,
            openid: None,
        })
        .unwrap();

        assert_eq!(value["media_url"], "https://example.com/image.png");
        assert_eq!(value["media_type"], 2);
        assert!(value.get("version").is_none());
    }

    #[test]
    fn serializes_short_key_generate_request_with_max_expire_seconds() {
        let request = ShortKeyGenerateRequest {
            long_data: "https://example.com/a".to_string(),
            expire_seconds: 40 * 86_400,
        }
        .with_max_expire_seconds();
        let value = serde_json::to_value(request).unwrap();

        assert_eq!(value["long_data"], "https://example.com/a");
        assert_eq!(value["expire_seconds"], 30 * 86_400);
    }

    #[test]
    fn serializes_short_key_fetch_request() {
        let value = serde_json::to_value(ShortKeyFetchRequest {
            short_key: "abc123".to_string(),
        })
        .unwrap();

        assert_eq!(value, json!({ "short_key": "abc123" }));
    }

    #[test]
    fn deserializes_short_key_responses() {
        let generated: ShortKeyGenerateResponse =
            serde_json::from_value(json!({ "short_key": "abc", "expire_seconds": 60 })).unwrap();
        let fetched: ShortKeyFetchResponse =
            serde_json::from_value(json!({ "long_data": "https://example.com/a" })).unwrap();

        assert_eq!(generated.short_key.as_deref(), Some("abc"));
        assert_eq!(generated.expire_seconds, Some(60));
        assert_eq!(fetched.long_data.as_deref(), Some("https://example.com/a"));
    }
}
