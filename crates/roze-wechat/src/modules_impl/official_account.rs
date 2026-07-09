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

    pub fn comment(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.comment")
    }

    pub async fn open_comment(
        &self,
        access_token: impl Into<String>,
        msg_data_id: impl Into<String>,
        index: i64,
    ) -> Result<WechatStatusResponse> {
        self.comment_action(
            "cgi-bin/comment/open",
            access_token,
            CommentArticleRequest {
                msg_data_id: msg_data_id.into(),
                index,
            },
        )
        .await
    }

    pub async fn close_comment(
        &self,
        access_token: impl Into<String>,
        msg_data_id: impl Into<String>,
        index: i64,
    ) -> Result<WechatStatusResponse> {
        self.comment_action(
            "cgi-bin/comment/close",
            access_token,
            CommentArticleRequest {
                msg_data_id: msg_data_id.into(),
                index,
            },
        )
        .await
    }

    pub async fn list_comments(
        &self,
        access_token: impl Into<String>,
        request: CommentListRequest,
    ) -> Result<CommentListResponse> {
        self.inner
            .post("cgi-bin/comment/list", Some(access_token.into()), request)
            .await
    }

    pub async fn mark_comment_elect(
        &self,
        access_token: impl Into<String>,
        request: CommentOperateRequest,
    ) -> Result<WechatStatusResponse> {
        self.comment_action("cgi-bin/comment/markelect", access_token, request)
            .await
    }

    pub async fn unmark_comment_elect(
        &self,
        access_token: impl Into<String>,
        request: CommentOperateRequest,
    ) -> Result<WechatStatusResponse> {
        self.comment_action("cgi-bin/comment/unmarkelect", access_token, request)
            .await
    }

    pub async fn delete_comment(
        &self,
        access_token: impl Into<String>,
        request: CommentOperateRequest,
    ) -> Result<WechatStatusResponse> {
        self.comment_action("cgi-bin/comment/delete", access_token, request)
            .await
    }

    pub async fn reply_comment(
        &self,
        access_token: impl Into<String>,
        request: CommentReplyRequest,
    ) -> Result<WechatStatusResponse> {
        self.comment_action("cgi-bin/comment/reply/add", access_token, request)
            .await
    }

    pub async fn delete_comment_reply(
        &self,
        access_token: impl Into<String>,
        request: CommentOperateRequest,
    ) -> Result<WechatStatusResponse> {
        self.comment_action("cgi-bin/comment/reply/delete", access_token, request)
            .await
    }

    pub fn data_cube(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.data_cube")
    }

    pub async fn data_cube_user_summary(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getusersummary", access_token, request)
            .await
    }

    pub async fn data_cube_user_cumulate(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getusercumulate", access_token, request)
            .await
    }

    pub async fn data_cube_article_summary(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getarticlesummary", access_token, request)
            .await
    }

    pub async fn data_cube_article_total(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getarticletotal", access_token, request)
            .await
    }

    pub async fn data_cube_user_read(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getuserread", access_token, request)
            .await
    }

    pub async fn data_cube_user_share(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getusershare", access_token, request)
            .await
    }

    pub async fn data_cube_user_share_hourly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getusersharehour", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsg", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_hourly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsghour", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_weekly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsgweek", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_monthly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsgmonth", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_dist(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsgdist", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_dist_weekly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsgdistweek", access_token, request)
            .await
    }

    pub async fn data_cube_upstream_message_dist_monthly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getupstreammsgdistmonth", access_token, request)
            .await
    }

    pub async fn data_cube_interface_summary(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getinterfacesummary", access_token, request)
            .await
    }

    pub async fn data_cube_interface_summary_hourly(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRangeRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getinterfacesummaryhour", access_token, request)
            .await
    }

    pub async fn data_cube_free_card_summary(
        &self,
        access_token: impl Into<String>,
        request: DataCubeCardSummaryRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getcardcardinfo", access_token, request)
            .await
    }

    pub async fn data_cube_member_card_summary(
        &self,
        access_token: impl Into<String>,
        request: DataCubeCardSummaryRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getcardmembercardinfo", access_token, request)
            .await
    }

    pub async fn data_cube_member_card_summary_by_id(
        &self,
        access_token: impl Into<String>,
        request: DataCubeCardDetailRequest,
    ) -> Result<DataCubeListResponse> {
        self.data_cube_query("datacube/getcardmembercarddetail", access_token, request)
            .await
    }

    async fn comment_action<B>(
        &self,
        path: &'static str,
        access_token: impl Into<String>,
        request: B,
    ) -> Result<WechatStatusResponse>
    where
        B: Serialize,
    {
        self.inner
            .post(path, Some(access_token.into()), request)
            .await
    }

    async fn data_cube_query<B>(
        &self,
        path: &'static str,
        access_token: impl Into<String>,
        request: B,
    ) -> Result<DataCubeListResponse>
    where
        B: Serialize,
    {
        self.inner
            .post(path, Some(access_token.into()), request)
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

    pub fn publish(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.publish")
    }

    pub async fn draft_add(
        &self,
        access_token: impl Into<String>,
        request: PublishDraftAddRequest,
    ) -> Result<PublishDraftAddResponse> {
        self.inner
            .post("cgi-bin/draft/add", Some(access_token.into()), request)
            .await
    }

    pub async fn draft_get(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<PublishDraftGetResponse> {
        self.inner
            .post(
                "cgi-bin/draft/get",
                Some(access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn draft_delete(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/draft/delete",
                Some(access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn draft_update(
        &self,
        access_token: impl Into<String>,
        request: PublishDraftUpdateRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("cgi-bin/draft/update", Some(access_token.into()), request)
            .await
    }

    pub async fn draft_count(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PublishDraftCountResponse> {
        self.inner
            .get("cgi-bin/draft/count", Some(access_token.into()))
            .await
    }

    pub async fn draft_batch_get(
        &self,
        access_token: impl Into<String>,
        request: PublishBatchGetRequest,
    ) -> Result<PublishBatchGetResponse> {
        self.inner
            .post("cgi-bin/draft/batchget", Some(access_token.into()), request)
            .await
    }

    pub async fn draft_switch(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("cgi-bin/draft/switch", Some(access_token.into()), json!({}))
            .await
    }

    pub async fn draft_check_switch(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PublishDraftSwitchStatusResponse> {
        self.inner
            .post_json_with_access_token_query(
                "cgi-bin/draft/switch",
                Some(access_token.into()),
                vec![("checkonly".to_string(), "1".to_string())],
                json!({}),
                Vec::new(),
            )
            .await
    }

    pub async fn publish_submit(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<PublishSubmitResponse> {
        self.inner
            .post(
                "cgi-bin/freepublish/submit",
                Some(access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn publish_get(
        &self,
        access_token: impl Into<String>,
        publish_id: u64,
    ) -> Result<PublishStatusResponse> {
        self.inner
            .post(
                "cgi-bin/freepublish/get",
                Some(access_token.into()),
                json!({ "publish_id": publish_id }),
            )
            .await
    }

    pub async fn publish_delete(
        &self,
        access_token: impl Into<String>,
        article_id: impl Into<String>,
        index: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/freepublish/delete",
                Some(access_token.into()),
                json!({ "article_id": article_id.into(), "index": index }),
            )
            .await
    }

    pub async fn publish_get_article(
        &self,
        access_token: impl Into<String>,
        article_id: impl Into<String>,
    ) -> Result<PublishArticleResponse> {
        self.inner
            .post(
                "cgi-bin/freepublish/getarticle",
                Some(access_token.into()),
                json!({ "article_id": article_id.into() }),
            )
            .await
    }

    pub async fn publish_batch_get(
        &self,
        access_token: impl Into<String>,
        request: PublishBatchGetRequest,
    ) -> Result<PublishBatchGetResponse> {
        self.inner
            .post(
                "cgi-bin/freepublish/batchget",
                Some(access_token.into()),
                request,
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
pub struct CommentArticleRequest {
    pub msg_data_id: String,
    pub index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentListRequest {
    pub msg_data_id: String,
    pub index: i64,
    pub begin: i64,
    pub count: i64,
    #[serde(rename = "type")]
    pub comment_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentOperateRequest {
    pub msg_data_id: String,
    pub index: i64,
    pub user_comment_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReplyRequest {
    pub msg_data_id: String,
    pub index: i64,
    pub user_comment_id: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentItem {
    #[serde(default)]
    pub user_comment_id: Option<i64>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub comment_type: Option<i64>,
    #[serde(default)]
    pub reply: Option<CommentReply>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub comment: Vec<CommentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeDateRangeRequest {
    pub begin_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeCardSummaryRequest {
    pub begin_date: String,
    pub end_date: String,
    pub cond_source: i64,
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeCardDetailRequest {
    pub begin_date: String,
    pub end_date: String,
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<Value>,
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
pub struct PublishArticle {
    pub title: String,
    pub author: String,
    pub digest: String,
    pub content: String,
    pub content_source_url: String,
    pub thumb_media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_open_comment: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fans_can_comment: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftAddRequest {
    pub articles: Vec<PublishArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftUpdateRequest {
    pub media_id: String,
    pub index: i64,
    pub articles: PublishArticle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchGetRequest {
    pub offset: i64,
    pub count: i64,
    pub no_content: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishNewsItem {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub content_source_url: Option<String>,
    #[serde(default)]
    pub thumb_media_id: Option<String>,
    #[serde(default)]
    pub thumb_url: Option<String>,
    #[serde(default)]
    pub show_cover_pic: Option<i64>,
    #[serde(default)]
    pub need_open_comment: Option<i64>,
    #[serde(default)]
    pub only_fans_can_comment: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftCountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishContent {
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchItem {
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub article_id: Option<String>,
    #[serde(default)]
    pub content: Option<PublishContent>,
    #[serde(default)]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub item_count: Option<i64>,
    #[serde(default)]
    pub item: Vec<PublishBatchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftSwitchStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub item_count: Option<i64>,
    #[serde(default)]
    pub is_open: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishSubmitResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub publish_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleItem {
    #[serde(default)]
    pub idx: Option<i64>,
    #[serde(default)]
    pub article_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleDetail {
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub item: Vec<PublishArticleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub publish_id: Option<u64>,
    #[serde(default)]
    pub publish_status: Option<i64>,
    #[serde(default)]
    pub article_id: Option<Value>,
    #[serde(default)]
    pub article_detail: Option<PublishArticleDetail>,
    #[serde(default)]
    pub fail_idx: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
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
        CardQrCodeResponse, CommentListRequest, CommentListResponse, CommentOperateRequest,
        CommentReplyRequest, CustomerServiceMessage, DataCubeCardDetailRequest,
        DataCubeCardSummaryRequest, DataCubeDateRangeRequest, DataCubeListResponse, JsapiConfig,
        MassSendAllRequest, MassSendFilter, MaterialListRequest, MenuButton,
        OauthAuthorizeUrlRequest, OfficialAccount, PublishArticle, PublishArticleResponse,
        PublishBatchGetRequest, PublishBatchGetResponse, PublishDraftAddRequest,
        PublishDraftAddResponse, PublishDraftGetResponse, PublishDraftSwitchStatusResponse,
        PublishDraftUpdateRequest, PublishStatusResponse, PublishSubmitResponse,
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
    fn serializes_publish_requests() {
        let article = PublishArticle {
            title: "Title".to_string(),
            author: "Roze".to_string(),
            digest: "Digest".to_string(),
            content: "<p>Hello</p>".to_string(),
            content_source_url: "https://example.com/source".to_string(),
            thumb_media_id: "thumb".to_string(),
            need_open_comment: Some(1),
            only_fans_can_comment: Some(0),
        };

        let add = serde_json::to_value(PublishDraftAddRequest {
            articles: vec![article.clone()],
        })
        .unwrap();
        assert_eq!(add["articles"][0]["title"], "Title");
        assert_eq!(add["articles"][0]["need_open_comment"], 1);

        let update = serde_json::to_value(PublishDraftUpdateRequest {
            media_id: "media".to_string(),
            index: 0,
            articles: article,
        })
        .unwrap();
        assert_eq!(update["media_id"], "media");
        assert_eq!(update["articles"]["thumb_media_id"], "thumb");

        let batch = serde_json::to_value(PublishBatchGetRequest {
            offset: 0,
            count: 20,
            no_content: 1,
        })
        .unwrap();
        assert_eq!(batch, json!({ "offset": 0, "count": 20, "no_content": 1 }));
    }

    #[test]
    fn deserializes_publish_responses() {
        let add: PublishDraftAddResponse =
            serde_json::from_value(json!({ "errcode": 0, "media_id": "media" })).unwrap();
        assert_eq!(add.media_id.as_deref(), Some("media"));

        let draft: PublishDraftGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "news_item": [{
                "title": "Title",
                "author": "Roze",
                "thumb_media_id": "thumb",
                "url": "https://example.com/article"
            }],
            "create_time": 1,
            "update_time": 2
        }))
        .unwrap();
        assert_eq!(draft.news_item[0].title.as_deref(), Some("Title"));
        assert_eq!(draft.update_time, Some(2));

        let list: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 1,
            "item_count": 1,
            "item": [{
                "media_id": "media",
                "article_id": "article",
                "content": { "news_item": [{ "title": "Title" }] },
                "update_time": 2
            }]
        }))
        .unwrap();
        assert_eq!(list.item[0].article_id.as_deref(), Some("article"));
        assert_eq!(
            list.item[0].content.as_ref().expect("content").news_item[0]
                .title
                .as_deref(),
            Some("Title")
        );

        let switch_status: PublishDraftSwitchStatusResponse = serde_json::from_value(json!({
            "is_open": 1,
            "total_count": 3,
            "item_count": 2
        }))
        .unwrap();
        assert_eq!(switch_status.is_open, Some(1));

        let submit: PublishSubmitResponse =
            serde_json::from_value(json!({ "publish_id": 10001 })).unwrap();
        assert_eq!(submit.publish_id, Some(10001));

        let status: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 10001,
            "publish_status": 0,
            "article_id": "article",
            "article_detail": {
                "count": 1,
                "item": [{ "idx": 1, "article_url": "https://example.com/article" }]
            },
            "fail_idx": []
        }))
        .unwrap();
        assert_eq!(status.publish_status, Some(0));
        assert_eq!(
            status.article_detail.expect("article_detail").item[0]
                .article_url
                .as_deref(),
            Some("https://example.com/article")
        );

        let article: PublishArticleResponse = serde_json::from_value(json!({
            "news_item": [{ "title": "Published", "url": "https://example.com/published" }]
        }))
        .unwrap();
        assert_eq!(article.news_item[0].title.as_deref(), Some("Published"));
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

    #[test]
    fn serializes_comment_requests() {
        let list = serde_json::to_value(CommentListRequest {
            msg_data_id: "msg".to_string(),
            index: 0,
            begin: 0,
            count: 20,
            comment_type: 1,
        })
        .unwrap();
        assert_eq!(list["msg_data_id"], "msg");
        assert_eq!(list["type"], 1);

        let operate = serde_json::to_value(CommentOperateRequest {
            msg_data_id: "msg".to_string(),
            index: 0,
            user_comment_id: 42,
        })
        .unwrap();
        assert_eq!(operate["user_comment_id"], 42);

        let reply = serde_json::to_value(CommentReplyRequest {
            msg_data_id: "msg".to_string(),
            index: 0,
            user_comment_id: 42,
            content: "thanks".to_string(),
        })
        .unwrap();
        assert_eq!(reply["content"], "thanks");
    }

    #[test]
    fn deserializes_comment_list_response() {
        let response: CommentListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "comment": [{
                "user_comment_id": 42,
                "openid": "openid",
                "create_time": 1_800_000_000,
                "content": "great",
                "comment_type": 0,
                "reply": { "content": "thanks", "create_time": 1_800_000_001 }
            }]
        }))
        .unwrap();

        assert_eq!(response.total, Some(1));
        assert_eq!(response.comment[0].openid.as_deref(), Some("openid"));
        assert_eq!(
            response.comment[0]
                .reply
                .as_ref()
                .expect("reply")
                .content
                .as_deref(),
            Some("thanks")
        );
    }

    #[test]
    fn serializes_data_cube_requests() {
        let range = serde_json::to_value(DataCubeDateRangeRequest {
            begin_date: "2026-07-01".to_string(),
            end_date: "2026-07-07".to_string(),
        })
        .unwrap();
        assert_eq!(range["begin_date"], "2026-07-01");
        assert_eq!(range["end_date"], "2026-07-07");

        let card = serde_json::to_value(DataCubeCardSummaryRequest {
            begin_date: "2026-07-01".to_string(),
            end_date: "2026-07-07".to_string(),
            cond_source: 0,
            card_id: "card".to_string(),
        })
        .unwrap();
        assert_eq!(card["cond_source"], 0);
        assert_eq!(card["card_id"], "card");

        let detail = serde_json::to_value(DataCubeCardDetailRequest {
            begin_date: "2026-07-01".to_string(),
            end_date: "2026-07-07".to_string(),
            card_id: "card".to_string(),
        })
        .unwrap();
        assert!(detail.get("cond_source").is_none());
        assert_eq!(detail["card_id"], "card");
    }

    #[test]
    fn deserializes_data_cube_list_response() {
        let response: DataCubeListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "list": [{
                "ref_date": "2026-07-01",
                "user_source": 0,
                "new_user": 10
            }]
        }))
        .unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.list[0]["ref_date"], "2026-07-01");
        assert_eq!(response.list[0]["new_user"], 10);
    }
}
