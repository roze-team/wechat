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
pub struct OfficialAccount {
    inner: PlatformClient,
}

impl OfficialAccount {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn oauth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.oauth")
    }

    pub fn oauth_authorize_url(request: OauthAuthorizeUrlRequest) -> String {
        let scope = request.scope.unwrap_or_else(|| "snsapi_base".to_string());
        let state = request.state.unwrap_or_else(|| "STATE".to_string());
        let mut url = url::Url::parse("https://open.weixin.qq.com/connect/oauth2/authorize")
            .expect("static oauth url is valid");
        url.query_pairs_mut()
            .append_pair("appid", &request.app_id)
            .append_pair("redirect_uri", &request.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &scope)
            .append_pair("state", &state);
        format!("{url}#wechat_redirect")
    }

    pub async fn oauth_access_token(
        &self,
        app_id: impl Into<String>,
        secret: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<OauthAccessTokenResponse> {
        self.inner
            .get_with_query(
                "sns/oauth2/access_token",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("secret".to_string(), secret.into()),
                    ("code".to_string(), code.into()),
                    ("grant_type".to_string(), "authorization_code".to_string()),
                ],
            )
            .await
    }

    pub async fn oauth_refresh_token(
        &self,
        app_id: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Result<OauthAccessTokenResponse> {
        self.inner
            .get_with_query(
                "sns/oauth2/refresh_token",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("grant_type".to_string(), "refresh_token".to_string()),
                    ("refresh_token".to_string(), refresh_token.into()),
                ],
            )
            .await
    }

    pub async fn oauth_user_info(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        lang: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "sns/userinfo",
                None,
                vec![
                    ("access_token".to_string(), access_token.into()),
                    ("openid".to_string(), openid.into()),
                    ("lang".to_string(), lang.into()),
                ],
            )
            .await
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.base")
    }

    pub async fn stable_access_token(
        &self,
        request: StableAccessTokenRequest,
    ) -> Result<StableAccessTokenResponse> {
        self.inner.post("cgi-bin/stable_token", None, request).await
    }

    pub fn broadcasting(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.broadcasting")
    }

    pub async fn mass_send_all(
        &self,
        access_token: impl Into<String>,
        request: MassSendAllRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/message/mass/sendall",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mass_send_openids(
        &self,
        access_token: impl Into<String>,
        request: MassSendOpenIdsRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/message/mass/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mass_delete(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
        article_idx: Option<i64>,
    ) -> Result<WechatStatusResponse> {
        let mut body = json!({ "msg_id": msg_id.into() });
        if let Some(article_idx) = article_idx {
            body["article_idx"] = json!(article_idx);
        }
        self.inner
            .post(
                "cgi-bin/message/mass/delete",
                Some(access_token.into()),
                body,
            )
            .await
    }

    pub async fn mass_preview(
        &self,
        access_token: impl Into<String>,
        request: Value,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/message/mass/preview",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mass_status(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/message/mass/get",
                Some(access_token.into()),
                json!({ "msg_id": msg_id.into() }),
            )
            .await
    }

    pub fn card(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.card")
    }

    pub async fn create_card(
        &self,
        access_token: impl Into<String>,
        request: CardCreateRequest,
    ) -> Result<CardCreateResponse> {
        self.inner
            .post("card/create", Some(access_token.into()), request)
            .await
    }

    pub async fn get_card(
        &self,
        access_token: impl Into<String>,
        card_id: impl Into<String>,
    ) -> Result<CardGetResponse> {
        self.inner
            .post(
                "card/get",
                Some(access_token.into()),
                CardIdRequest {
                    card_id: card_id.into(),
                },
            )
            .await
    }

    pub async fn delete_card(
        &self,
        access_token: impl Into<String>,
        card_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "card/delete",
                Some(access_token.into()),
                CardIdRequest {
                    card_id: card_id.into(),
                },
            )
            .await
    }

    pub async fn get_card_code(
        &self,
        access_token: impl Into<String>,
        request: CardCodeRequest,
    ) -> Result<CardCodeResponse> {
        self.inner
            .post("card/code/get", Some(access_token.into()), request)
            .await
    }

    pub async fn decrypt_card_code(
        &self,
        access_token: impl Into<String>,
        encrypt_code: impl Into<String>,
    ) -> Result<CardCodeDecryptResponse> {
        self.inner
            .post(
                "card/code/decrypt",
                Some(access_token.into()),
                CardCodeDecryptRequest {
                    encrypt_code: encrypt_code.into(),
                },
            )
            .await
    }

    pub async fn create_card_qr_code(
        &self,
        access_token: impl Into<String>,
        request: CardQrCodeRequest,
    ) -> Result<CardQrCodeResponse> {
        self.inner
            .post("card/qrcode/create", Some(access_token.into()), request)
            .await
    }

    pub fn customer_service(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.customer_service")
    }

    pub async fn list_customer_service_accounts(
        &self,
        access_token: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get("cgi-bin/customservice/getkflist", Some(access_token.into()))
            .await
    }

    pub async fn list_online_customer_service_accounts(
        &self,
        access_token: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get(
                "cgi-bin/customservice/getonlinekflist",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn create_customer_service_account(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        nickname: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "customservice/kfaccount/add",
                Some(access_token.into()),
                json!({
                    "kf_account": account.into(),
                    "nickname": nickname.into(),
                }),
            )
            .await
    }

    pub async fn update_customer_service_account(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        nickname: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "customservice/kfaccount/update",
                Some(access_token.into()),
                json!({
                    "kf_account": account.into(),
                    "nickname": nickname.into(),
                }),
            )
            .await
    }

    pub async fn delete_customer_service_account(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .get_with_query(
                "customservice/kfaccount/del",
                Some(access_token.into()),
                vec![("kf_account".to_string(), account.into())],
            )
            .await
    }

    pub async fn invite_customer_service_worker(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        wechat_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "customservice/kfaccount/inviteworker",
                Some(access_token.into()),
                json!({
                    "kf_account": account.into(),
                    "invite_wx": wechat_id.into(),
                }),
            )
            .await
    }

    pub async fn send_customer_service_message(
        &self,
        access_token: impl Into<String>,
        message: CustomerServiceMessage,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/custom/send",
                Some(access_token.into()),
                message,
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

    pub fn jssdk(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.jssdk")
    }

    pub fn build_jsapi_config(
        &self,
        app_id: impl Into<String>,
        jsapi_ticket: impl AsRef<str>,
        url: impl AsRef<str>,
        js_api_list: Vec<String>,
    ) -> JsapiConfig {
        let nonce_str = crypto::nonce_string(16);
        let timestamp = chrono::Utc::now().timestamp();
        let app_id = app_id.into();
        let plain = format!(
            "jsapi_ticket={}&noncestr={}&timestamp={}&url={}",
            jsapi_ticket.as_ref(),
            nonce_str,
            timestamp,
            url.as_ref()
        );
        let signature = crypto::sha1_signature(&[plain.as_str()]);

        JsapiConfig {
            app_id,
            timestamp,
            nonce_str,
            signature,
            js_api_list,
        }
    }

    pub fn material(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.material")
    }

    pub async fn upload_material_from_bytes(
        &self,
        access_token: impl Into<String>,
        kind: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        let kind = kind.into();
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/material/add_material",
                Some(access_token.into()),
                vec![("type".to_string(), kind)],
                form,
            )
            .await
    }

    pub async fn upload_video_material_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
        title: impl Into<String>,
        introduction: impl Into<String>,
    ) -> Result<MaterialMediaResponse> {
        let description = json!({
            "title": title.into(),
            "introduction": introduction.into(),
        })
        .to_string();
        let form = reqwest::multipart::Form::new()
            .part(
                "media",
                reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
            )
            .text("description", description.clone())
            .text("Description", description);
        self.inner
            .post_multipart(
                "cgi-bin/material/add_material",
                Some(access_token.into()),
                vec![("type".to_string(), "video".to_string())],
                form,
            )
            .await
    }

    pub async fn upload_article_image_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
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

    pub async fn add_news_material(
        &self,
        access_token: impl Into<String>,
        articles: Vec<Article>,
    ) -> Result<MaterialMediaResponse> {
        self.inner
            .post(
                "cgi-bin/material/add_news",
                Some(access_token.into()),
                json!({ "articles": articles }),
            )
            .await
    }

    pub async fn update_news_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
        index: i64,
        article: Article,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/material/update_news",
                Some(access_token.into()),
                json!({
                    "media_id": media_id.into(),
                    "index": index,
                    "articles": article,
                }),
            )
            .await
    }

    pub async fn get_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/material/get_material",
                Some(access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn delete_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/material/del_material",
                Some(access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn list_materials(
        &self,
        access_token: impl Into<String>,
        request: MaterialListRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/material/batchget_material",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn material_stats(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/material/get_materialcount",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub fn menu(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.menu")
    }

    pub async fn get_menu(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get("cgi-bin/menu/get", Some(access_token.into()))
            .await
    }

    pub async fn current_self_menu(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get(
                "cgi-bin/get_current_selfmenu_info",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn create_menu(
        &self,
        access_token: impl Into<String>,
        request: CreateMenuRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/menu/create",
                Some(access_token.into()),
                json!({ "button": request.button }),
            )
            .await
    }

    pub async fn create_conditional_menu(
        &self,
        access_token: impl Into<String>,
        request: CreateConditionalMenuRequest,
    ) -> Result<CreateConditionalMenuResponse> {
        self.inner
            .post(
                "cgi-bin/menu/addconditional",
                Some(access_token.into()),
                json!({
                    "button": request.button,
                    "matchrule": request.matchrule,
                }),
            )
            .await
    }

    pub async fn delete_menu(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .get("cgi-bin/menu/delete", Some(access_token.into()))
            .await
    }

    pub async fn delete_conditional_menu(
        &self,
        access_token: impl Into<String>,
        menu_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/menu/delconditional",
                Some(access_token.into()),
                json!({ "menuid": menu_id.into() }),
            )
            .await
    }

    pub async fn try_match_menu(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/menu/trymatch",
                Some(access_token.into()),
                json!({ "user_id": user_id.into() }),
            )
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.server")
    }

    pub fn template_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.template_message")
    }

    pub async fn send_template_message(
        &self,
        access_token: impl Into<String>,
        request: TemplateMessageRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/message/template/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_template_industry(
        &self,
        access_token: impl Into<String>,
        industry_id1: impl Into<String>,
        industry_id2: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/template/api_set_industry",
                Some(access_token.into()),
                json!({
                    "industry_id1": industry_id1.into(),
                    "industry_id2": industry_id2.into(),
                }),
            )
            .await
    }

    pub async fn get_template_industry(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get("cgi-bin/template/get_industry", Some(access_token.into()))
            .await
    }

    pub async fn add_template(
        &self,
        access_token: impl Into<String>,
        template_id_short: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/template/api_add_template",
                Some(access_token.into()),
                json!({ "template_id_short": template_id_short.into() }),
            )
            .await
    }

    pub async fn list_private_templates(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get(
                "cgi-bin/template/get_all_private_template",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn delete_private_template(
        &self,
        access_token: impl Into<String>,
        template_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/template/del_private_template",
                Some(access_token.into()),
                json!({ "template_id": template_id.into() }),
            )
            .await
    }

    pub fn user(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.user")
    }

    pub async fn get_user_info(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        lang: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/info",
                Some(access_token.into()),
                vec![
                    ("openid".to_string(), openid.into()),
                    ("lang".to_string(), lang.into()),
                ],
            )
            .await
    }

    pub async fn batch_get_user_info(
        &self,
        access_token: impl Into<String>,
        request: BatchGetUserInfoRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/user/info/batchget",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_users(
        &self,
        access_token: impl Into<String>,
        next_openid: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/get",
                Some(access_token.into()),
                vec![("next_openid".to_string(), next_openid.into())],
            )
            .await
    }

    pub async fn update_user_remark(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        remark: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/user/info/updateremark",
                Some(access_token.into()),
                json!({
                    "openid": openid.into(),
                    "remark": remark.into(),
                }),
            )
            .await
    }

    pub async fn blacklist(
        &self,
        access_token: impl Into<String>,
        begin_openid: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/tags/members/getblacklist",
                Some(access_token.into()),
                json!({ "begin_openid": begin_openid.into() }),
            )
            .await
    }

    pub async fn block_users(
        &self,
        access_token: impl Into<String>,
        openid_list: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/tags/members/batchblacklist",
                Some(access_token.into()),
                json!({ "openid_list": openid_list }),
            )
            .await
    }

    pub async fn unblock_users(
        &self,
        access_token: impl Into<String>,
        openid_list: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/tags/members/batchunblacklist",
                Some(access_token.into()),
                json!({ "openid_list": openid_list }),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuButton {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_button: Vec<MenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthAuthorizeUrlRequest {
    pub app_id: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMenuRequest {
    pub button: Vec<MenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalMenuRequest {
    pub button: Vec<MenuButton>,
    pub matchrule: MatchRule,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_platform_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalMenuResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub menuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassSendAllRequest {
    pub filter: MassSendFilter,
    pub msgtype: String,
    #[serde(flatten)]
    pub message: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_ignore_reprint: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassSendFilter {
    pub is_to_all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassSendOpenIdsRequest {
    pub touser: Vec<String>,
    pub msgtype: String,
    #[serde(flatten)]
    pub message: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_ignore_reprint: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCreateRequest {
    pub card: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardIdRequest {
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCodeRequest {
    pub card_id: String,
    pub code: String,
    #[serde(default)]
    pub check_consume: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card: Option<Value>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub can_consume: Option<bool>,
    #[serde(default)]
    pub user_card_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCodeDecryptRequest {
    pub encrypt_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCodeDecryptResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardQrCodeRequest {
    pub action_name: String,
    pub action_info: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardQrCodeResponse {
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
    #[serde(default)]
    pub show_qrcode_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsapiConfig {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub timestamp: i64,
    #[serde(rename = "nonceStr")]
    pub nonce_str: String,
    pub signature: String,
    #[serde(rename = "jsApiList")]
    pub js_api_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessage {
    pub touser: String,
    pub msgtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxcard: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogrampage: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customservice: Option<Value>,
}

impl CustomerServiceMessage {
    pub fn text(touser: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            touser: touser.into(),
            msgtype: "text".to_string(),
            text: Some(json!({ "content": content.into() })),
            image: None,
            voice: None,
            video: None,
            music: None,
            news: None,
            mpnews: None,
            wxcard: None,
            miniprogrampage: None,
            customservice: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub thumb_media_id: String,
    pub author: String,
    pub digest: String,
    pub show_cover_pic: i64,
    pub content: String,
    pub content_source_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_open_comment: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fans_can_comment: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialMediaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialListRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub offset: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMessageRequest {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<TemplateMiniProgram>,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMiniProgram {
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUserInfoRequest {
    pub user_list: Vec<UserInfoQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoQuery {
    pub openid: String,
    #[serde(default = "default_zh_cn")]
    pub lang: String,
}

fn default_zh_cn() -> String {
    "zh_CN".to_string()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{config::Platform, Client, WechatConfig};

    use super::{
        BatchGetUserInfoRequest, CardCodeDecryptRequest, CardCodeDecryptResponse, CardCodeRequest,
        CardCodeResponse, CardCreateRequest, CardCreateResponse, CardIdRequest, CardQrCodeRequest,
        CardQrCodeResponse, CustomerServiceMessage, JsapiConfig, MassSendAllRequest,
        MassSendFilter, MaterialListRequest, MenuButton, OauthAuthorizeUrlRequest, OfficialAccount,
        TemplateMessageRequest, TemplateMiniProgram, UserInfoQuery,
    };

    #[test]
    fn serializes_menu_button_wire_names() {
        let value = serde_json::to_value(MenuButton {
            name: "Open".to_string(),
            kind: Some("view".to_string()),
            key: None,
            url: Some("https://example.com".to_string()),
            media_id: None,
            appid: None,
            pagepath: None,
            sub_button: Vec::new(),
        })
        .unwrap();

        assert_eq!(value["type"], "view");
        assert_eq!(value["url"], "https://example.com");
    }

    #[test]
    fn serializes_batch_user_query() {
        let value = serde_json::to_value(BatchGetUserInfoRequest {
            user_list: vec![UserInfoQuery {
                openid: "openid".to_string(),
                lang: "zh_CN".to_string(),
            }],
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "user_list": [{ "openid": "openid", "lang": "zh_CN" }] })
        );
    }

    #[test]
    fn serializes_customer_service_text_message() {
        let value = serde_json::to_value(CustomerServiceMessage::text("openid", "hello")).unwrap();
        assert_eq!(value["touser"], "openid");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");
    }

    #[test]
    fn serializes_card_create_request() {
        let value = serde_json::to_value(CardCreateRequest {
            card: json!({
                "card_type": "GROUPON",
                "groupon": {
                    "base_info": { "brand_name": "brand", "title": "title" },
                    "deal_detail": "detail"
                }
            }),
        })
        .unwrap();

        assert_eq!(value["card"]["card_type"], "GROUPON");
        assert_eq!(value["card"]["groupon"]["base_info"]["brand_name"], "brand");
    }

    #[test]
    fn deserializes_card_create_response() {
        let response: CardCreateResponse =
            serde_json::from_value(json!({ "errcode": 0, "card_id": "card" })).unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.card_id.as_deref(), Some("card"));
    }

    #[test]
    fn serializes_card_id_request() {
        let value = serde_json::to_value(CardIdRequest {
            card_id: "card".to_string(),
        })
        .unwrap();

        assert_eq!(value, json!({ "card_id": "card" }));
    }

    #[test]
    fn serializes_card_code_request() {
        let value = serde_json::to_value(CardCodeRequest {
            card_id: "card".to_string(),
            code: "code".to_string(),
            check_consume: true,
        })
        .unwrap();

        assert_eq!(value["card_id"], "card");
        assert_eq!(value["code"], "code");
        assert_eq!(value["check_consume"], true);
    }

    #[test]
    fn deserializes_card_code_response() {
        let response: CardCodeResponse = serde_json::from_value(json!({
            "openid": "openid",
            "can_consume": true,
            "user_card_status": "NORMAL",
            "card": { "card_id": "card" }
        }))
        .unwrap();

        assert_eq!(response.openid.as_deref(), Some("openid"));
        assert_eq!(response.can_consume, Some(true));
        assert_eq!(response.user_card_status.as_deref(), Some("NORMAL"));
        assert_eq!(response.card.expect("card")["card_id"], "card");
    }

    #[test]
    fn serializes_card_code_decrypt_request() {
        let value = serde_json::to_value(CardCodeDecryptRequest {
            encrypt_code: "encrypted".to_string(),
        })
        .unwrap();

        assert_eq!(value, json!({ "encrypt_code": "encrypted" }));
    }

    #[test]
    fn deserializes_card_code_decrypt_response() {
        let response: CardCodeDecryptResponse =
            serde_json::from_value(json!({ "code": "plain-code" })).unwrap();

        assert_eq!(response.code.as_deref(), Some("plain-code"));
    }

    #[test]
    fn serializes_card_qr_code_request() {
        let value = serde_json::to_value(CardQrCodeRequest {
            action_name: "QR_CARD".to_string(),
            action_info: json!({ "card": { "card_id": "card" } }),
            expire_seconds: Some(1800),
        })
        .unwrap();

        assert_eq!(value["action_name"], "QR_CARD");
        assert_eq!(value["action_info"]["card"]["card_id"], "card");
        assert_eq!(value["expire_seconds"], 1800);
    }

    #[test]
    fn deserializes_card_qr_code_response() {
        let response: CardQrCodeResponse = serde_json::from_value(json!({
            "ticket": "ticket",
            "expire_seconds": 1800,
            "url": "https://example.com/card",
            "show_qrcode_url": "https://example.com/qrcode"
        }))
        .unwrap();

        assert_eq!(response.ticket.as_deref(), Some("ticket"));
        assert_eq!(response.expire_seconds, Some(1800));
        assert_eq!(
            response.show_qrcode_url.as_deref(),
            Some("https://example.com/qrcode")
        );
    }

    #[test]
    fn serializes_material_list_request_type_name() {
        let value = serde_json::to_value(MaterialListRequest {
            kind: "news".to_string(),
            offset: 0,
            count: 20,
        })
        .unwrap();
        assert_eq!(value, json!({ "type": "news", "offset": 0, "count": 20 }));
    }

    #[test]
    fn builds_jsapi_config() {
        let account = OfficialAccount::new(
            Client::new(WechatConfig::default()).expect("client"),
            Platform::OfficialAccount,
        );
        let config = account.build_jsapi_config(
            "wxappid",
            "ticket",
            "https://example.com/page?a=1",
            vec!["chooseImage".to_string()],
        );

        assert_eq!(config.app_id, "wxappid");
        assert_eq!(config.nonce_str.len(), 16);
        assert_eq!(config.signature.len(), 40);
        assert_eq!(config.js_api_list, vec!["chooseImage"]);
    }

    #[test]
    fn serializes_jsapi_config_wire_names() {
        let value = serde_json::to_value(JsapiConfig {
            app_id: "wxappid".to_string(),
            timestamp: 1_800_000_000,
            nonce_str: "nonce".to_string(),
            signature: "signature".to_string(),
            js_api_list: vec!["chooseImage".to_string()],
        })
        .unwrap();

        assert_eq!(value["appId"], "wxappid");
        assert_eq!(value["timestamp"], 1_800_000_000);
        assert_eq!(value["nonceStr"], "nonce");
        assert_eq!(value["signature"], "signature");
        assert_eq!(value["jsApiList"][0], "chooseImage");
    }

    #[test]
    fn builds_oauth_authorize_url() {
        let url = OfficialAccount::oauth_authorize_url(OauthAuthorizeUrlRequest {
            app_id: "appid".to_string(),
            redirect_uri: "https://example.com/cb?a=1".to_string(),
            scope: Some("snsapi_userinfo".to_string()),
            state: Some("abc".to_string()),
        });

        assert!(url.starts_with("https://open.weixin.qq.com/connect/oauth2/authorize?"));
        assert!(url.contains("appid=appid"));
        assert!(url.ends_with("#wechat_redirect"));
    }

    #[test]
    fn serializes_template_message_request() {
        let value = serde_json::to_value(TemplateMessageRequest {
            touser: "openid".to_string(),
            template_id: "tpl".to_string(),
            url: None,
            miniprogram: Some(TemplateMiniProgram {
                appid: "mini".to_string(),
                pagepath: Some("pages/index".to_string()),
            }),
            data: json!({ "first": { "value": "hello" } }),
            client_msg_id: Some("msg-1".to_string()),
        })
        .unwrap();

        assert_eq!(value["touser"], "openid");
        assert_eq!(value["miniprogram"]["appid"], "mini");
        assert_eq!(value["data"]["first"]["value"], "hello");
        assert!(value.get("url").is_none());
    }

    #[test]
    fn serializes_mass_send_all_request() {
        let value = serde_json::to_value(MassSendAllRequest {
            filter: MassSendFilter {
                is_to_all: false,
                tag_id: Some(2),
            },
            msgtype: "mpnews".to_string(),
            message: json!({ "mpnews": { "media_id": "mid" } }),
            send_ignore_reprint: Some(0),
        })
        .unwrap();

        assert_eq!(value["filter"]["tag_id"], 2);
        assert_eq!(value["msgtype"], "mpnews");
        assert_eq!(value["mpnews"]["media_id"], "mid");
    }
}
