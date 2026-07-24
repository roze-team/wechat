use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    crypto,
    error::{Result, WechatError},
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
    ) -> Result<OauthUserInfoResponse> {
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

    pub async fn clear_quota(
        &self,
        access_token: impl Into<String>,
        app_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/clear_quota",
                Some(access_token.into()),
                json!({ "appid": app_id.into() }),
            )
            .await
    }

    pub async fn callback_ip(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialCallbackIpResponse> {
        self.inner
            .get("cgi-bin/getcallbackip", Some(access_token.into()))
            .await
    }

    pub async fn check_callback_url(
        &self,
        access_token: impl Into<String>,
        action: impl Into<String>,
        check_operator: impl Into<String>,
    ) -> Result<OfficialCallbackCheckResponse> {
        self.inner
            .post(
                "cgi-bin/callbacks/check",
                Some(access_token.into()),
                json!({
                    "action": action.into(),
                    "check_operator": check_operator.into(),
                }),
            )
            .await
    }

    pub fn broadcasting(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.broadcasting")
    }

    pub async fn mass_send_all(
        &self,
        access_token: impl Into<String>,
        request: MassSendAllRequest,
    ) -> Result<MassSendResponse> {
        request.validate()?;
        let response: MassSendResponse = self
            .inner
            .post(
                "cgi-bin/message/mass/sendall",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn mass_send_openids(
        &self,
        access_token: impl Into<String>,
        request: MassSendOpenIdsRequest,
    ) -> Result<MassSendResponse> {
        request.validate()?;
        let response: MassSendResponse = self
            .inner
            .post(
                "cgi-bin/message/mass/send",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn mass_delete(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
        article_idx: Option<i64>,
    ) -> Result<WechatStatusResponse> {
        let msg_id = msg_id.into();
        validate_official_identifier("mass message id", &msg_id)?;
        if article_idx.is_some_and(|index| !(0..=8).contains(&index)) {
            return Err(WechatError::Config(
                "official account mass delete article index must be between 0 and 8".to_string(),
            ));
        }
        let mut body = json!({ "msg_id": msg_id });
        if let Some(article_idx) = article_idx {
            body["article_idx"] = json!(article_idx);
        }
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/message/mass/delete",
                Some(access_token.into()),
                body,
            )
            .await?;
        response.validate_for("official account mass delete")?;
        Ok(response)
    }

    pub async fn mass_preview(
        &self,
        access_token: impl Into<String>,
        request: MassPreviewRequest,
    ) -> Result<MassPreviewResponse> {
        request.validate()?;
        let response: MassPreviewResponse = self
            .inner
            .post(
                "cgi-bin/message/mass/preview",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn mass_status(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Result<MassStatusResponse> {
        let msg_id = msg_id.into();
        validate_official_identifier("mass message id", &msg_id)?;
        let response: MassStatusResponse = self
            .inner
            .post(
                "cgi-bin/message/mass/get",
                Some(access_token.into()),
                json!({ "msg_id": msg_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
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

    pub async fn list_cards(
        &self,
        access_token: impl Into<String>,
        offset: i64,
        count: i64,
        status_list: Vec<String>,
    ) -> Result<CardListResponse> {
        self.inner
            .post(
                "card/batchget",
                Some(access_token.into()),
                json!({
                    "offset": offset,
                    "count": count,
                    "status_list": status_list,
                }),
            )
            .await
    }

    pub async fn update_card(
        &self,
        access_token: impl Into<String>,
        card_id: impl Into<String>,
        card_type: impl AsRef<str>,
        card: Value,
    ) -> Result<CardUpdateResponse> {
        let mut body = serde_json::Map::new();
        body.insert("card_id".to_string(), json!(card_id.into()));
        body.insert(card_type.as_ref().to_ascii_lowercase(), card);
        self.inner
            .post(
                "card/update",
                Some(access_token.into()),
                Value::Object(body),
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
    ) -> Result<CustomerServiceAccountListResponse> {
        self.inner
            .get("cgi-bin/customservice/getkflist", Some(access_token.into()))
            .await
    }

    pub async fn list_online_customer_service_accounts(
        &self,
        access_token: impl Into<String>,
    ) -> Result<CustomerServiceOnlineAccountListResponse> {
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
        let account = account.into();
        let nickname = nickname.into();
        validate_official_required("customer service account", &account)?;
        validate_official_required("customer service nickname", &nickname)?;
        self.inner
            .post(
                "customservice/kfaccount/add",
                Some(access_token.into()),
                json!({
                    "kf_account": account,
                    "nickname": nickname,
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
        let account = account.into();
        let nickname = nickname.into();
        validate_official_required("customer service account", &account)?;
        validate_official_required("customer service nickname", &nickname)?;
        self.inner
            .post(
                "customservice/kfaccount/update",
                Some(access_token.into()),
                json!({
                    "kf_account": account,
                    "nickname": nickname,
                }),
            )
            .await
    }

    pub async fn delete_customer_service_account(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let account = account.into();
        validate_official_required("customer service account", &account)?;
        self.inner
            .get_with_query(
                "customservice/kfaccount/del",
                Some(access_token.into()),
                vec![("kf_account".to_string(), account)],
            )
            .await
    }

    pub async fn invite_customer_service_worker(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        wechat_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let account = account.into();
        let wechat_id = wechat_id.into();
        validate_official_required("customer service account", &account)?;
        validate_official_required("customer service WeChat id", &wechat_id)?;
        self.inner
            .post(
                "customservice/kfaccount/inviteworker",
                Some(access_token.into()),
                json!({
                    "kf_account": account,
                    "invite_wx": wechat_id,
                }),
            )
            .await
    }

    pub async fn upload_customer_service_avatar_from_bytes(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WechatStatusResponse> {
        let account = account.into();
        let file_name = file_name.into();
        validate_official_required("customer service account", &account)?;
        validate_material_upload(&file_name, &data)?;
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name),
        );
        self.inner
            .post_multipart(
                "customservice/kfaccount/uploadheadimg",
                Some(access_token.into()),
                vec![("kf_account".to_string(), account)],
                form,
            )
            .await
    }

    pub async fn customer_service_sessions(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
    ) -> Result<CustomerServiceSessionListResponse> {
        let account = account.into();
        validate_official_required("customer service account", &account)?;
        self.inner
            .get_with_query(
                "customservice/kfsession/getsessionlist",
                Some(access_token.into()),
                vec![("kf_account".to_string(), account)],
            )
            .await
    }

    pub async fn waiting_customer_service_sessions(
        &self,
        access_token: impl Into<String>,
    ) -> Result<CustomerServiceWaitCaseListResponse> {
        self.inner
            .get(
                "customservice/kfsession/getwaitcase",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn create_customer_service_session(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let account = account.into();
        let openid = openid.into();
        validate_official_required("customer service account", &account)?;
        validate_official_required("customer service openid", &openid)?;
        self.inner
            .post(
                "customservice/kfsession/create",
                Some(access_token.into()),
                json!({ "kf_account": account, "openid": openid }),
            )
            .await
    }

    pub async fn close_customer_service_session(
        &self,
        access_token: impl Into<String>,
        account: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let account = account.into();
        let openid = openid.into();
        validate_official_required("customer service account", &account)?;
        validate_official_required("customer service openid", &openid)?;
        self.inner
            .post(
                "customservice/kfsession/close",
                Some(access_token.into()),
                json!({ "kf_account": account, "openid": openid }),
            )
            .await
    }

    pub async fn customer_service_session(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<CustomerServiceSessionResponse> {
        let openid = openid.into();
        validate_official_required("customer service openid", &openid)?;
        self.inner
            .get_with_query(
                "customservice/kfsession/getsession",
                Some(access_token.into()),
                vec![("openid".to_string(), openid)],
            )
            .await
    }

    pub async fn customer_service_message_records(
        &self,
        access_token: impl Into<String>,
        request: CustomerServiceMessageRecordRequest,
    ) -> Result<CustomerServiceMessageRecordResponse> {
        request.validate()?;
        self.inner
            .post(
                "customservice/msgrecord/getmsglist",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn send_customer_service_message(
        &self,
        access_token: impl Into<String>,
        message: CustomerServiceMessage,
    ) -> Result<WechatStatusResponse> {
        message.validate()?;
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
        let openid = openid.into();
        validate_official_required("customer service openid", &openid)?;
        self.inner
            .post(
                "cgi-bin/message/custom/typing",
                Some(access_token.into()),
                json!({
                    "touser": openid,
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
        kind: MaterialUploadKind,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        let file_name = file_name.into();
        validate_material_upload(&file_name, &data)?;
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name),
        );
        let response: MaterialMediaResponse = self
            .inner
            .post_multipart(
                "cgi-bin/material/add_material",
                Some(access_token.into()),
                vec![("type".to_string(), kind.as_code().to_string())],
                form,
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn upload_image_material_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_material_from_bytes(access_token, MaterialUploadKind::Image, file_name, data)
            .await
    }

    pub async fn upload_voice_material_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_material_from_bytes(access_token, MaterialUploadKind::Voice, file_name, data)
            .await
    }

    pub async fn upload_thumb_material_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_material_from_bytes(access_token, MaterialUploadKind::Thumb, file_name, data)
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
        let file_name = file_name.into();
        let title = title.into();
        let introduction = introduction.into();
        validate_material_upload(&file_name, &data)?;
        validate_material_required("video title", &title)?;
        validate_material_required("video introduction", &introduction)?;
        let description = json!({
            "title": title,
            "introduction": introduction,
        })
        .to_string();
        let form = reqwest::multipart::Form::new()
            .part(
                "media",
                reqwest::multipart::Part::bytes(data).file_name(file_name),
            )
            .text("description", description.clone())
            .text("Description", description);
        let response: MaterialMediaResponse = self
            .inner
            .post_multipart(
                "cgi-bin/material/add_material",
                Some(access_token.into()),
                vec![("type".to_string(), "video".to_string())],
                form,
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn upload_article_image_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        let file_name = file_name.into();
        validate_material_upload(&file_name, &data)?;
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name),
        );
        let response: MaterialMediaResponse = self
            .inner
            .post_multipart(
                "cgi-bin/media/uploadimg",
                Some(access_token.into()),
                Vec::new(),
                form,
            )
            .await?;
        response.require_url()?;
        Ok(response)
    }

    pub async fn add_news_material(
        &self,
        access_token: impl Into<String>,
        articles: Vec<Article>,
    ) -> Result<MaterialMediaResponse> {
        validate_material_articles(&articles)?;
        let response: MaterialMediaResponse = self
            .inner
            .post(
                "cgi-bin/material/add_news",
                Some(access_token.into()),
                json!({ "articles": articles }),
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn update_news_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
        index: i64,
        article: Article,
    ) -> Result<WechatStatusResponse> {
        let media_id = media_id.into();
        validate_material_identifier("media id", &media_id)?;
        if !(0..=7).contains(&index) {
            return Err(WechatError::Config(
                "official account material article index must be between 0 and 7".to_string(),
            ));
        }
        article.validate()?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/material/update_news",
                Some(access_token.into()),
                json!({
                    "media_id": media_id,
                    "index": index,
                    "articles": article,
                }),
            )
            .await?;
        response.validate_for("official account update news material")?;
        Ok(response)
    }

    pub async fn get_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<MaterialGetResponse> {
        let media_id = media_id.into();
        validate_material_identifier("media id", &media_id)?;
        let response: MaterialGetResponse = self
            .inner
            .post(
                "cgi-bin/material/get_material",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_material_bytes(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        let media_id = media_id.into();
        validate_material_identifier("media id", &media_id)?;
        self.inner
            .post_json_bytes(
                "cgi-bin/material/get_material",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await
    }

    pub async fn delete_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let media_id = media_id.into();
        validate_material_identifier("media id", &media_id)?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/material/del_material",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate_for("official account delete material")?;
        Ok(response)
    }

    pub async fn list_materials(
        &self,
        access_token: impl Into<String>,
        request: MaterialListRequest,
    ) -> Result<MaterialListResponse> {
        request.validate()?;
        let response: MaterialListResponse = self
            .inner
            .post(
                "cgi-bin/material/batchget_material",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn material_stats(
        &self,
        access_token: impl Into<String>,
    ) -> Result<MaterialStatsResponse> {
        let response: MaterialStatsResponse = self
            .inner
            .post(
                "cgi-bin/material/get_materialcount",
                Some(access_token.into()),
                json!({}),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub fn publish(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.publish")
    }

    pub async fn draft_add(
        &self,
        access_token: impl Into<String>,
        request: PublishDraftAddRequest,
    ) -> Result<PublishDraftAddResponse> {
        request.validate()?;
        let response: PublishDraftAddResponse = self
            .inner
            .post("cgi-bin/draft/add", Some(access_token.into()), request)
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn draft_get(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<PublishDraftGetResponse> {
        let media_id = media_id.into();
        validate_publish_identifier("draft media id", &media_id)?;
        let response: PublishDraftGetResponse = self
            .inner
            .post(
                "cgi-bin/draft/get",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn draft_delete(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let media_id = media_id.into();
        validate_publish_identifier("draft media id", &media_id)?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/draft/delete",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate_for("official account delete draft")?;
        Ok(response)
    }

    pub async fn draft_update(
        &self,
        access_token: impl Into<String>,
        request: PublishDraftUpdateRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        let response: WechatStatusResponse = self
            .inner
            .post("cgi-bin/draft/update", Some(access_token.into()), request)
            .await?;
        response.validate_for("official account update draft")?;
        Ok(response)
    }

    pub async fn draft_count(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PublishDraftCountResponse> {
        let response: PublishDraftCountResponse = self
            .inner
            .get("cgi-bin/draft/count", Some(access_token.into()))
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn draft_batch_get(
        &self,
        access_token: impl Into<String>,
        request: PublishBatchGetRequest,
    ) -> Result<PublishBatchGetResponse> {
        request.validate()?;
        let no_content = request.no_content == 1;
        let response: PublishBatchGetResponse = self
            .inner
            .post("cgi-bin/draft/batchget", Some(access_token.into()), request)
            .await?;
        response.validate_for(PublishBatchKind::Draft, no_content)?;
        Ok(response)
    }

    pub async fn draft_switch(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let response: WechatStatusResponse = self
            .inner
            .post("cgi-bin/draft/switch", Some(access_token.into()), json!({}))
            .await?;
        response.validate_for("official account switch draft mode")?;
        Ok(response)
    }

    pub async fn draft_check_switch(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PublishDraftSwitchStatusResponse> {
        let response: PublishDraftSwitchStatusResponse = self
            .inner
            .post_json_with_access_token_query(
                "cgi-bin/draft/switch",
                Some(access_token.into()),
                vec![("checkonly".to_string(), "1".to_string())],
                json!({}),
                Vec::new(),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn publish_submit(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<PublishSubmitResponse> {
        let media_id = media_id.into();
        validate_publish_identifier("publish media id", &media_id)?;
        let response: PublishSubmitResponse = self
            .inner
            .post(
                "cgi-bin/freepublish/submit",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.require_publish_id()?;
        Ok(response)
    }

    pub async fn publish_get(
        &self,
        access_token: impl Into<String>,
        publish_id: u64,
    ) -> Result<PublishStatusResponse> {
        if publish_id == 0 {
            return Err(WechatError::Config(
                "official account publish id must be positive".to_string(),
            ));
        }
        let response: PublishStatusResponse = self
            .inner
            .post(
                "cgi-bin/freepublish/get",
                Some(access_token.into()),
                json!({ "publish_id": publish_id }),
            )
            .await?;
        response.validate_for(publish_id)?;
        Ok(response)
    }

    pub async fn publish_delete(
        &self,
        access_token: impl Into<String>,
        article_id: impl Into<String>,
        index: i64,
    ) -> Result<WechatStatusResponse> {
        let article_id = article_id.into();
        validate_publish_identifier("published article id", &article_id)?;
        validate_publish_delete_index(index)?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/freepublish/delete",
                Some(access_token.into()),
                json!({ "article_id": article_id, "index": index }),
            )
            .await?;
        response.validate_for("official account delete published article")?;
        Ok(response)
    }

    pub async fn publish_get_article(
        &self,
        access_token: impl Into<String>,
        article_id: impl Into<String>,
    ) -> Result<PublishArticleResponse> {
        let article_id = article_id.into();
        validate_publish_identifier("published article id", &article_id)?;
        let response: PublishArticleResponse = self
            .inner
            .post(
                "cgi-bin/freepublish/getarticle",
                Some(access_token.into()),
                json!({ "article_id": article_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn publish_batch_get(
        &self,
        access_token: impl Into<String>,
        request: PublishBatchGetRequest,
    ) -> Result<PublishBatchGetResponse> {
        request.validate()?;
        let no_content = request.no_content == 1;
        let response: PublishBatchGetResponse = self
            .inner
            .post(
                "cgi-bin/freepublish/batchget",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate_for(PublishBatchKind::Published, no_content)?;
        Ok(response)
    }

    pub fn menu(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.menu")
    }

    pub async fn get_menu(&self, access_token: impl Into<String>) -> Result<MenuGetResponse> {
        let response: MenuGetResponse = self
            .inner
            .get("cgi-bin/menu/get", Some(access_token.into()))
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn current_self_menu(
        &self,
        access_token: impl Into<String>,
    ) -> Result<CurrentSelfMenuResponse> {
        let response: CurrentSelfMenuResponse = self
            .inner
            .get(
                "cgi-bin/get_current_selfmenu_info",
                Some(access_token.into()),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn create_menu(
        &self,
        access_token: impl Into<String>,
        request: CreateMenuRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/menu/create",
                Some(access_token.into()),
                json!({ "button": request.button }),
            )
            .await?;
        response.validate_for("official account menu create")?;
        Ok(response)
    }

    pub async fn create_conditional_menu(
        &self,
        access_token: impl Into<String>,
        request: CreateConditionalMenuRequest,
    ) -> Result<CreateConditionalMenuResponse> {
        request.validate()?;
        let response: CreateConditionalMenuResponse = self
            .inner
            .post(
                "cgi-bin/menu/addconditional",
                Some(access_token.into()),
                json!({
                    "button": request.button,
                    "matchrule": request.matchrule,
                }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn delete_menu(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let response: WechatStatusResponse = self
            .inner
            .get("cgi-bin/menu/delete", Some(access_token.into()))
            .await?;
        response.validate_for("official account menu delete")?;
        Ok(response)
    }

    pub async fn delete_conditional_menu(
        &self,
        access_token: impl Into<String>,
        menu_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let menu_id = menu_id.into();
        validate_menu_required("conditional menu id", &menu_id)?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/menu/delconditional",
                Some(access_token.into()),
                json!({ "menuid": menu_id }),
            )
            .await?;
        response.validate_for("official account conditional menu delete")?;
        Ok(response)
    }

    pub async fn try_match_menu(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<MenuTryMatchResponse> {
        let user_id = user_id.into();
        validate_menu_required("match user id", &user_id)?;
        let response: MenuTryMatchResponse = self
            .inner
            .post(
                "cgi-bin/menu/trymatch",
                Some(access_token.into()),
                json!({ "user_id": user_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub fn auto_reply(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.auto_reply")
    }

    pub async fn get_current_auto_reply_info(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialAutoReplyInfoResponse> {
        self.inner
            .get(
                "cgi-bin/get_current_autoreply_info",
                Some(access_token.into()),
            )
            .await
    }

    pub fn semantic(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.semantic")
    }

    pub async fn semantic_query(
        &self,
        access_token: impl Into<String>,
        request: OfficialSemanticQueryRequest,
    ) -> Result<OfficialSemanticQueryResponse> {
        self.inner
            .post(
                "semantic/semproxy/search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn poi(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.poi")
    }

    pub async fn poi_get(
        &self,
        access_token: impl Into<String>,
        poi_id: i64,
    ) -> Result<OfficialPoiGetResponse> {
        self.inner
            .post(
                "cgi-bin/poi/getpoi",
                Some(access_token.into()),
                json!({ "poi_id": poi_id }),
            )
            .await
    }

    pub async fn poi_list(
        &self,
        access_token: impl Into<String>,
        begin: i64,
        limit: i64,
    ) -> Result<OfficialPoiListResponse> {
        self.inner
            .post(
                "cgi-bin/poi/getpoilist",
                Some(access_token.into()),
                json!({ "begin": begin, "limit": limit }),
            )
            .await
    }

    pub async fn poi_create(
        &self,
        access_token: impl Into<String>,
        base_info: Value,
    ) -> Result<OfficialPoiMutationResponse> {
        self.inner
            .post(
                "cgi-bin/poi/addpoi",
                Some(access_token.into()),
                json!({ "business": { "base_info": base_info } }),
            )
            .await
    }

    pub async fn poi_update(
        &self,
        access_token: impl Into<String>,
        base_info: Value,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/poi/updatepoi",
                Some(access_token.into()),
                json!({ "business": { "base_info": base_info } }),
            )
            .await
    }

    pub async fn poi_delete(
        &self,
        access_token: impl Into<String>,
        poi_id: i64,
    ) -> Result<OfficialPoiGetResponse> {
        self.inner
            .post(
                "cgi-bin/poi/delpoi",
                Some(access_token.into()),
                json!({ "poi_id": poi_id }),
            )
            .await
    }

    pub fn device(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.device")
    }

    pub async fn device_message(
        &self,
        access_token: impl Into<String>,
        request: OfficialDeviceMessageRequest,
    ) -> Result<OfficialDeviceMessageResponse> {
        self.inner
            .post("device/transmsg", Some(access_token.into()), request)
            .await
    }

    pub async fn device_qrcode(
        &self,
        access_token: impl Into<String>,
        device_id_list: Vec<String>,
    ) -> Result<OfficialDeviceCreateQrCodeResponse> {
        let device_num = device_id_list.len();
        self.inner
            .post(
                "device/create_qrcode",
                Some(access_token.into()),
                json!({ "device_num": device_num, "device_id_list": device_id_list }),
            )
            .await
    }

    pub async fn device_authorize(
        &self,
        access_token: impl Into<String>,
        request: OfficialDeviceAuthorizeRequest,
    ) -> Result<OfficialDeviceAuthorizeResponse> {
        self.inner
            .post(
                "device/authorize_device",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn device_create_id(
        &self,
        access_token: impl Into<String>,
        product_id: impl Into<String>,
    ) -> Result<OfficialDeviceCreateIdResponse> {
        self.inner
            .post(
                "device/authorize_device",
                Some(access_token.into()),
                json!({ "product_id": product_id.into() }),
            )
            .await
    }

    pub async fn device_bind(
        &self,
        access_token: impl Into<String>,
        request: OfficialDeviceBindRequest,
    ) -> Result<OfficialDeviceBindResponse> {
        self.inner
            .post("device/bind", Some(access_token.into()), request)
            .await
    }

    pub async fn device_unbind(
        &self,
        access_token: impl Into<String>,
        request: OfficialDeviceBindRequest,
    ) -> Result<OfficialDeviceBindResponse> {
        self.inner
            .post("device/unbind", Some(access_token.into()), request)
            .await
    }

    pub async fn device_force_bind(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        device_id: impl Into<String>,
    ) -> Result<OfficialDeviceBindResponse> {
        self.inner
            .post(
                "device/compel_bind",
                Some(access_token.into()),
                json!({ "openid": openid.into(), "device_id": device_id.into() }),
            )
            .await
    }

    pub async fn device_force_unbind(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        device_id: impl Into<String>,
    ) -> Result<OfficialDeviceBindResponse> {
        self.inner
            .post(
                "device/compel_unbind",
                Some(access_token.into()),
                json!({ "openid": openid.into(), "device_id": device_id.into() }),
            )
            .await
    }

    pub fn goods(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.goods")
    }

    pub async fn goods_add(
        &self,
        access_token: impl Into<String>,
        request: OfficialGoodsProductRequest,
    ) -> Result<OfficialGoodsProductAddResponse> {
        self.inner
            .post("scan/product/v2/add", Some(access_token.into()), request)
            .await
    }

    pub async fn goods_update(
        &self,
        access_token: impl Into<String>,
        request: OfficialGoodsProductRequest,
    ) -> Result<OfficialGoodsProductAddResponse> {
        self.inner
            .post("scan/product/v2/add", Some(access_token.into()), request)
            .await
    }

    pub async fn goods_status(
        &self,
        access_token: impl Into<String>,
        status_ticket: impl Into<String>,
    ) -> Result<OfficialGoodsProductStatusResponse> {
        self.inner
            .post(
                "scan/product/v2/status",
                Some(access_token.into()),
                json!({ "status_ticket": status_ticket.into() }),
            )
            .await
    }

    pub async fn goods_get(
        &self,
        access_token: impl Into<String>,
        pid: impl Into<String>,
    ) -> Result<OfficialGoodsProductGetResponse> {
        self.inner
            .post(
                "scan/product/v2/getinfo",
                Some(access_token.into()),
                json!({ "product": { "pid": pid.into() } }),
            )
            .await
    }

    pub async fn goods_list(
        &self,
        access_token: impl Into<String>,
        page_context: impl Into<String>,
        page_num: i64,
        page_size: i64,
    ) -> Result<OfficialGoodsProductGetResponse> {
        self.inner
            .post(
                "scan/product/v2/getinfobypage",
                Some(access_token.into()),
                json!({
                    "page_context": page_context.into(),
                    "page_num": page_num,
                    "page_size": page_size
                }),
            )
            .await
    }

    pub fn ocr(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.ocr")
    }

    pub async fn ocr_id_card(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
        id_type: impl Into<String>,
    ) -> Result<OfficialOcrIdCardResponse> {
        self.inner
            .post(
                "cv/ocr/idcard",
                Some(access_token.into()),
                json!({ "img_url": img_url.into(), "type": id_type.into() }),
            )
            .await
    }

    pub async fn ocr_bank_card(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrBankCardResponse> {
        self.ocr_image(access_token, "cv/ocr/bankcard", img_url)
            .await
    }

    pub async fn ocr_vehicle_license(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrVehicleLicenseResponse> {
        self.ocr_image(access_token, "cv/ocr/drivinglicense", img_url)
            .await
    }

    pub async fn ocr_driving(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrVehicleLicenseResponse> {
        self.ocr_image(access_token, "cv/ocr/driving", img_url)
            .await
    }

    pub async fn ocr_biz_license(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrBizLicenseResponse> {
        self.ocr_image(access_token, "cv/ocr/bizlicense", img_url)
            .await
    }

    pub async fn ocr_common(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrCommonResponse> {
        self.ocr_image(access_token, "cv/ocr/comm", img_url).await
    }

    pub async fn ocr_plate_number(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OfficialOcrPlateNumberResponse> {
        self.ocr_image(access_token, "cv/ocr/platenum", img_url)
            .await
    }

    async fn ocr_image<R>(
        &self,
        access_token: impl Into<String>,
        path: &'static str,
        img_url: impl Into<String>,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        self.inner
            .post(
                path,
                Some(access_token.into()),
                json!({ "img_url": img_url.into() }),
            )
            .await
    }

    pub fn store(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.store")
    }

    pub async fn store_categories(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialStoreCategoryResponse> {
        self.inner
            .get("wxa/get_merchant_category", Some(access_token.into()))
            .await
    }

    pub async fn store_districts(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialStoreDistrictResponse> {
        self.inner
            .get("wxa/get_district", Some(access_token.into()))
            .await
    }

    pub async fn store_search_from_map(
        &self,
        access_token: impl Into<String>,
        district_id: i64,
        keyword: impl Into<String>,
    ) -> Result<OfficialStoreSearchMapResponse> {
        self.inner
            .post(
                "wxa/search_map_poi",
                Some(access_token.into()),
                json!({ "districtid": district_id, "keyword": keyword.into() }),
            )
            .await
    }

    pub async fn store_status(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialStoreStatusResponse> {
        self.inner
            .get("wxa/get_merchant_audit_info", Some(access_token.into()))
            .await
    }

    pub async fn store_update_merchant(
        &self,
        access_token: impl Into<String>,
        media_id: i64,
        intro: impl Into<String>,
    ) -> Result<OfficialStoreSearchMapResponse> {
        self.inner
            .post(
                "wxa/modify_merchant",
                Some(access_token.into()),
                json!({ "headimg_mediaid": media_id, "intro": intro.into() }),
            )
            .await
    }

    pub async fn store_create_from_map(
        &self,
        access_token: impl Into<String>,
        base_info: OfficialStoreBaseInfo,
    ) -> Result<OfficialStoreCreateFromMapResponse> {
        self.inner
            .post("wxa/create_map_poi", Some(access_token.into()), base_info)
            .await
    }

    pub async fn store_create(
        &self,
        access_token: impl Into<String>,
        request: OfficialStoreCreateRequest,
    ) -> Result<OfficialStoreCreateResponse> {
        self.inner
            .post("wxa/add_store", Some(access_token.into()), request)
            .await
    }

    pub async fn store_update(
        &self,
        access_token: impl Into<String>,
        request: OfficialStoreUpdateRequest,
    ) -> Result<OfficialStoreUpdateResponse> {
        self.inner
            .post("wxa/update_store", Some(access_token.into()), request)
            .await
    }

    pub async fn store_get(
        &self,
        access_token: impl Into<String>,
        poi_id: i64,
    ) -> Result<OfficialStoreInfoResponse> {
        self.inner
            .post(
                "wxa/get_store_info",
                Some(access_token.into()),
                json!({ "poi_id": poi_id }),
            )
            .await
    }

    pub async fn store_list(
        &self,
        access_token: impl Into<String>,
        offset: i64,
        limit: i64,
    ) -> Result<OfficialStoreListResponse> {
        self.inner
            .post(
                "wxa/get_store_list",
                Some(access_token.into()),
                json!({ "offset": offset, "limit": limit }),
            )
            .await
    }

    pub async fn store_delete(
        &self,
        access_token: impl Into<String>,
        poi_id: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/del_store",
                Some(access_token.into()),
                json!({ "poi_id": poi_id }),
            )
            .await
    }

    pub fn wifi(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.wifi")
    }

    pub async fn wifi_summary(
        &self,
        access_token: impl Into<String>,
        begin_date: impl Into<String>,
        end_date: impl Into<String>,
        shop_id: i64,
    ) -> Result<OfficialWifiSummaryResponse> {
        self.inner
            .post(
                "bizwifi/statistics/list",
                Some(access_token.into()),
                json!({ "begin_date": begin_date.into(), "end_date": end_date.into(), "shop_id": shop_id }),
            )
            .await
    }

    pub async fn wifi_qrcode_url(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        ssid: impl Into<String>,
        image_id: i64,
    ) -> Result<OfficialWifiQrCodeUrlResponse> {
        self.inner
            .post(
                "bizwifi/qrcode/get",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "ssid": ssid.into(), "img_id": image_id }),
            )
            .await
    }

    pub async fn wifi_set_finish_page(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        finish_page_url: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "bizwifi/finishpage/set",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "finishpage_url": finish_page_url.into() }),
            )
            .await
    }

    pub async fn wifi_set_home_page(
        &self,
        access_token: impl Into<String>,
        request: OfficialWifiSetHomePageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("bizwifi/homepage/set", Some(access_token.into()), request)
            .await
    }

    pub async fn wifi_shop_get(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
    ) -> Result<OfficialWifiShopGetResponse> {
        self.inner
            .post(
                "bizwifi/shop/get",
                Some(access_token.into()),
                json!({ "shop_id": shop_id }),
            )
            .await
    }

    pub async fn wifi_shop_list(
        &self,
        access_token: impl Into<String>,
        page: i64,
        size: i64,
    ) -> Result<OfficialWifiShopListResponse> {
        self.inner
            .post(
                "bizwifi/shop/List",
                Some(access_token.into()),
                json!({ "pageindex": page, "pagesize": size }),
            )
            .await
    }

    pub async fn wifi_shop_update(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        old_ssid: impl Into<String>,
        ssid: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "bizwifi/shop/update",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "old_ssid": old_ssid.into(), "ssid": ssid.into() }),
            )
            .await
    }

    pub async fn wifi_shop_clear_device(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        ssid: Option<String>,
    ) -> Result<WechatStatusResponse> {
        let mut body = json!({ "shop_id": shop_id });
        if let Some(ssid) = ssid {
            body["ssid"] = json!(ssid);
        }
        self.inner
            .post("bizwifi/shop/clean", Some(access_token.into()), body)
            .await
    }

    pub async fn wifi_device_add_password(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        ssid: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "bizwifi/device/add",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "ssid": ssid.into(), "password": password.into() }),
            )
            .await
    }

    pub async fn wifi_device_add_portal(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        ssid: impl Into<String>,
        reset: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "bizwifi/apportal/register",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "ssid": ssid.into(), "reset": reset.into() }),
            )
            .await
    }

    pub async fn wifi_device_delete(
        &self,
        access_token: impl Into<String>,
        mac_address: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "bizwifi/device/delete",
                Some(access_token.into()),
                json!({ "bssid": mac_address.into() }),
            )
            .await
    }

    pub async fn wifi_device_list(
        &self,
        access_token: impl Into<String>,
        page: i64,
        size: i64,
    ) -> Result<OfficialWifiDeviceListResponse> {
        self.inner
            .post(
                "bizwifi/device/List",
                Some(access_token.into()),
                json!({ "pageindex": page, "pagesize": size }),
            )
            .await
    }

    pub async fn wifi_device_list_by_shop(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
        page: i64,
        size: i64,
    ) -> Result<OfficialWifiDeviceListResponse> {
        self.inner
            .post(
                "bizwifi/device/List",
                Some(access_token.into()),
                json!({ "shop_id": shop_id, "pageindex": page, "pagesize": size }),
            )
            .await
    }

    pub async fn wifi_card_set(
        &self,
        access_token: impl Into<String>,
        request: OfficialWifiCardSetRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("bizwifi/couponput/set", Some(access_token.into()), request)
            .await
    }

    pub async fn wifi_card_get(
        &self,
        access_token: impl Into<String>,
        shop_id: i64,
    ) -> Result<OfficialWifiCardGetResponse> {
        self.inner
            .post(
                "bizwifi/couponput/get",
                Some(access_token.into()),
                json!({ "shop_id": shop_id }),
            )
            .await
    }

    pub fn guide(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.guide")
    }

    pub async fn guide_create_adviser(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAdviserRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguideacct",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_get_adviser(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAccountRequest,
    ) -> Result<OfficialGuideGetAdviserResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguideacct",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_update_adviser(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAdviserRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/updateguideacct",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_delete_adviser(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAccountRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguideacct",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_advisers(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAccountRequest,
    ) -> Result<OfficialGuideGetAdvisersResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguideacctlist",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_create_qrcode(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideQrcodeRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/guidecreateqrcode",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyer_chat_records(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerChatRecordRequest,
    ) -> Result<OfficialGuideChatRecordsResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyerchatrecord",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_set_config(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideConfigRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/setguideconfig",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_get_config(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAccountRequest,
    ) -> Result<OfficialGuideConfigResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguideconfig",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_set_adviser_config(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideAdviserConfigRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/setguideacctconfig",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_get_adviser_config(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialGuideAdviserConfigResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguideacctconfig",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn guide_allow_copy_mini_app_path(
        &self,
        access_token: impl Into<String>,
        wxa_appid: impl Into<String>,
        wx_username: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/pushshowwxapathmenu",
                Some(access_token.into()),
                json!({ "wxa_appid": wxa_appid.into(), "wx_username": wx_username.into() }),
            )
            .await
    }

    pub async fn guide_create_group(
        &self,
        access_token: impl Into<String>,
        group_name: impl Into<String>,
    ) -> Result<OfficialGuideCreateGroupResponse> {
        self.inner
            .post(
                "cgi-bin/guide/newguidegroup",
                Some(access_token.into()),
                json!({ "group_name": group_name.into() }),
            )
            .await
    }

    pub async fn guide_groups(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialGuideGroupListResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidegrouplist",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn guide_group(
        &self,
        access_token: impl Into<String>,
        group_name: impl Into<String>,
    ) -> Result<OfficialGuideGroupResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getgroupinfo",
                Some(access_token.into()),
                json!({ "group_name": group_name.into() }),
            )
            .await
    }

    pub async fn guide_add_group_guide(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        guide_account: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguide2guidegroup",
                Some(access_token.into()),
                json!({ "group_id": group_id, "guide_account": guide_account.into() }),
            )
            .await
    }

    pub async fn guide_delete_group_guide(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        guide_account: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguide2guidegroup",
                Some(access_token.into()),
                json!({ "group_id": group_id, "guide_account": guide_account.into() }),
            )
            .await
    }

    pub async fn guide_group_by_guide(
        &self,
        access_token: impl Into<String>,
        guide_account: impl Into<String>,
    ) -> Result<OfficialGuideGroupByGuideResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getgroupbyguide",
                Some(access_token.into()),
                json!({ "guide_account": guide_account.into() }),
            )
            .await
    }

    pub async fn guide_delete_group(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidegroup",
                Some(access_token.into()),
                json!({ "group_id": group_id }),
            )
            .await
    }

    pub async fn guide_create_buyer_relation(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerRelationRequest,
    ) -> Result<OfficialGuideBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguidebuyerrelation",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_delete_buyer_relation(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideDeleteBuyerRelationRequest,
    ) -> Result<OfficialGuideBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidebuyerrelation",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyer_relations(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerRelationsRequest,
    ) -> Result<OfficialGuideBuyerRelationListResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyerrelationlist",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_rebind_buyer(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideRebindBuyerRequest,
    ) -> Result<OfficialGuideBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/rebindguideacctforbuyer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_update_buyer_relation(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideUpdateBuyerRelationRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/updateguidebuyerrelation",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyer_relation(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<OfficialGuideGetBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyerrelationbybuyer",
                Some(access_token.into()),
                json!({ "openid": openid.into() }),
            )
            .await
    }

    pub async fn guide_buyer_relation_by_guide(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerRelationByGuideRequest,
    ) -> Result<OfficialGuideGetBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyerrelation",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_new_tag_option(
        &self,
        access_token: impl Into<String>,
        tag_name: impl Into<String>,
        tag_values: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/newguidetagoption",
                Some(access_token.into()),
                json!({ "tag_name": tag_name.into(), "tag_values": tag_values }),
            )
            .await
    }

    pub async fn guide_delete_tag_option(
        &self,
        access_token: impl Into<String>,
        tag_name: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidetagoption",
                Some(access_token.into()),
                json!({ "tag_name": tag_name.into() }),
            )
            .await
    }

    pub async fn guide_add_tag_option(
        &self,
        access_token: impl Into<String>,
        tag_name: impl Into<String>,
        tag_values: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguidetagoption",
                Some(access_token.into()),
                json!({ "tag_name": tag_name.into(), "tag_values": tag_values }),
            )
            .await
    }

    pub async fn guide_tag_options(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialGuideTagOptionResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidetagoption",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn guide_set_buyers_tag(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerTagRequest,
    ) -> Result<OfficialGuideBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguidebuyertag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyer_tags(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerTagRequest,
    ) -> Result<OfficialGuideBuyerTagsResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyertag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyers_by_tag(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyersByTagRequest,
    ) -> Result<OfficialGuideBuyersByTagResponse> {
        self.inner
            .post(
                "cgi-bin/guide/queryguidebuyerbytag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_delete_buyer_tag(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideDeleteBuyerTagRequest,
    ) -> Result<OfficialGuideBuyerRelationResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidebuyertag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_set_buyer_display_tags(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerDisplayTagRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/addguidebuyerdisplaytag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_buyer_display_tags(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideBuyerDisplayTagsRequest,
    ) -> Result<OfficialGuideBuyerDisplayTagsResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidebuyerdisplaytag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_create_card_material(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideCardMaterialRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/setguidecardmaterial",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_card_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
    ) -> Result<OfficialGuideCardMaterialResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidecardmaterial",
                Some(access_token.into()),
                json!({ "type": material_type }),
            )
            .await
    }

    pub async fn guide_delete_card_material(
        &self,
        access_token: impl Into<String>,
        request: OfficialGuideDeleteCardMaterialRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidecardmaterial",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn guide_create_image_material(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
        material_type: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/setguideimagematerial",
                Some(access_token.into()),
                json!({ "media_id": media_id.into(), "type": material_type }),
            )
            .await
    }

    pub async fn guide_image_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
    ) -> Result<OfficialGuideImageMaterialResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguideimagematerial",
                Some(access_token.into()),
                json!({ "type": material_type }),
            )
            .await
    }

    pub async fn guide_delete_image_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
        pic_url: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguideimagematerial",
                Some(access_token.into()),
                json!({ "type": material_type, "picurl": pic_url.into() }),
            )
            .await
    }

    pub async fn guide_create_word_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
        word: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/setguidewordmaterial",
                Some(access_token.into()),
                json!({ "type": material_type, "word": word.into() }),
            )
            .await
    }

    pub async fn guide_word_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
        start: i64,
        num: i64,
    ) -> Result<OfficialGuideWordMaterialResponse> {
        self.inner
            .post(
                "cgi-bin/guide/getguidewordmaterial",
                Some(access_token.into()),
                json!({ "type": material_type, "start": start, "num": num }),
            )
            .await
    }

    pub async fn guide_delete_word_material(
        &self,
        access_token: impl Into<String>,
        material_type: i64,
        word: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/guide/delguidewordmaterial",
                Some(access_token.into()),
                json!({ "type": material_type, "word": word.into() }),
            )
            .await
    }

    pub fn shake_around(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.shake_around")
    }

    pub async fn shake_around_register(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundAccountRegisterRequest,
    ) -> Result<OfficialShakeAroundAccountRegisterResponse> {
        self.inner
            .post(
                "shakearound/account/register",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn shake_around_status(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialShakeAroundAccountRegisterResponse> {
        self.inner
            .get("shakearound/account/auditstatus", Some(access_token.into()))
            .await
    }

    pub async fn shake_around_user(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundUserRequest,
    ) -> Result<OfficialShakeAroundUserResponse> {
        self.inner
            .post(
                "shakearound/user/getshakeinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn shake_around_user_with_poi(
        &self,
        access_token: impl Into<String>,
        ticket: impl Into<String>,
    ) -> Result<OfficialShakeAroundUserResponse> {
        self.shake_around_user(
            access_token,
            OfficialShakeAroundUserRequest {
                ticket: ticket.into(),
                need_poi: Some(1),
            },
        )
        .await
    }

    pub async fn shake_around_device_apply(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundDeviceApplyRequest,
    ) -> Result<OfficialShakeAroundDeviceApplyResponse> {
        self.inner
            .post(
                "shakearound/device/applyid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn shake_around_device_apply_status(
        &self,
        access_token: impl Into<String>,
        apply_id: i64,
    ) -> Result<OfficialShakeAroundDeviceApplyStatusResponse> {
        self.inner
            .post(
                "shakearound/device/applystatus",
                Some(access_token.into()),
                json!({ "apply_id": apply_id }),
            )
            .await
    }

    pub async fn shake_around_device_update(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
        comment: impl Into<String>,
    ) -> Result<OfficialShakeAroundDeviceApplyStatusResponse> {
        self.inner
            .post(
                "shakearound/device/update",
                Some(access_token.into()),
                json!({ "device_identifier": device_identifier, "comment": comment.into() }),
            )
            .await
    }

    pub async fn shake_around_device_bind_poi(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
        poi_id: i64,
    ) -> Result<OfficialShakeAroundDeviceBindPoiResponse> {
        self.inner
            .post(
                "shakearound/device/bindlocation",
                Some(access_token.into()),
                json!({ "device_identifier": device_identifier, "poi_id": poi_id }),
            )
            .await
    }

    pub async fn shake_around_device_bind_third_poi(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
        poi_id: i64,
        app_id: impl Into<String>,
    ) -> Result<OfficialShakeAroundDeviceBindPoiResponse> {
        self.inner
            .post(
                "shakearound/device/bindlocation",
                Some(access_token.into()),
                json!({ "device_identifier": device_identifier, "poi_id": poi_id, "type": 2, "poi_appid": app_id.into() }),
            )
            .await
    }

    pub async fn shake_around_devices_by_ids(
        &self,
        access_token: impl Into<String>,
        device_identifiers: Vec<OfficialShakeAroundDeviceIdentifier>,
    ) -> Result<OfficialShakeAroundDeviceSearchResponse> {
        self.shake_around_device_search(
            access_token,
            OfficialShakeAroundDeviceSearchRequest {
                search_type: 1,
                device_identifiers,
                apply_id: None,
                last_seen: None,
                count: None,
            },
        )
        .await
    }

    pub async fn shake_around_devices(
        &self,
        access_token: impl Into<String>,
        last_seen: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundDeviceSearchResponse> {
        self.shake_around_device_search(
            access_token,
            OfficialShakeAroundDeviceSearchRequest {
                search_type: 2,
                device_identifiers: Vec::new(),
                apply_id: None,
                last_seen: Some(last_seen),
                count: Some(count),
            },
        )
        .await
    }

    pub async fn shake_around_devices_by_apply_id(
        &self,
        access_token: impl Into<String>,
        apply_id: i64,
        last_seen: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundDeviceSearchResponse> {
        self.shake_around_device_search(
            access_token,
            OfficialShakeAroundDeviceSearchRequest {
                search_type: 3,
                device_identifiers: Vec::new(),
                apply_id: Some(apply_id),
                last_seen: Some(last_seen),
                count: Some(count),
            },
        )
        .await
    }

    pub async fn shake_around_device_search(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundDeviceSearchRequest,
    ) -> Result<OfficialShakeAroundDeviceSearchResponse> {
        self.inner
            .post(
                "shakearound/device/search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn shake_around_group_create(
        &self,
        access_token: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<OfficialShakeAroundGroupResponse> {
        self.inner
            .post(
                "shakearound/device/group/add",
                Some(access_token.into()),
                json!({ "group_name": name.into() }),
            )
            .await
    }

    pub async fn shake_around_group_update(
        &self,
        access_token: impl Into<String>,
        group_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<OfficialShakeAroundGroupResponse> {
        self.inner
            .post(
                "shakearound/device/group/update",
                Some(access_token.into()),
                json!({ "group_id": group_id.into(), "group_name": name.into() }),
            )
            .await
    }

    pub async fn shake_around_group_delete(
        &self,
        access_token: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Result<OfficialShakeAroundGroupResponse> {
        self.inner
            .post(
                "shakearound/device/group/delete",
                Some(access_token.into()),
                json!({ "group_id": group_id.into() }),
            )
            .await
    }

    pub async fn shake_around_group_list(
        &self,
        access_token: impl Into<String>,
        begin: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundGroupListResponse> {
        self.inner
            .post(
                "shakearound/device/group/getlist",
                Some(access_token.into()),
                json!({ "begin": begin, "count": count }),
            )
            .await
    }

    pub async fn shake_around_group_get(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        begin: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundGroupDetailResponse> {
        self.inner
            .post(
                "shakearound/device/group/getdetail",
                Some(access_token.into()),
                json!({ "group_id": group_id, "begin": begin, "count": count }),
            )
            .await
    }

    pub async fn shake_around_group_add_devices(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        device_identifiers: Vec<OfficialShakeAroundDeviceIdentifier>,
    ) -> Result<OfficialShakeAroundGroupResponse> {
        self.inner
            .post(
                "shakearound/device/group/adddevice",
                Some(access_token.into()),
                json!({ "group_id": group_id, "device_identifiers": device_identifiers }),
            )
            .await
    }

    pub async fn shake_around_group_remove_devices(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        device_identifiers: Vec<OfficialShakeAroundDeviceIdentifier>,
    ) -> Result<OfficialShakeAroundGroupResponse> {
        self.inner
            .post(
                "shakearound/device/group/deletedevice",
                Some(access_token.into()),
                json!({ "group_id": group_id, "device_identifiers": device_identifiers }),
            )
            .await
    }

    pub async fn shake_around_page_create(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundPageInfoRequest,
    ) -> Result<OfficialShakeAroundPageResponse> {
        self.inner
            .post("shakearound/page/add", Some(access_token.into()), request)
            .await
    }

    pub async fn shake_around_page_update(
        &self,
        access_token: impl Into<String>,
        request: OfficialShakeAroundPageUpdateRequest,
    ) -> Result<OfficialShakeAroundPageResponse> {
        self.inner
            .post(
                "shakearound/page/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn shake_around_page_list(
        &self,
        access_token: impl Into<String>,
        begin: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundPageListResponse> {
        self.inner
            .post(
                "shakearound/page/search",
                Some(access_token.into()),
                json!({ "type": 2, "begin": begin, "count": count }),
            )
            .await
    }

    pub async fn shake_around_page_delete(
        &self,
        access_token: impl Into<String>,
        page_id: impl Into<String>,
    ) -> Result<OfficialShakeAroundPageResponse> {
        self.inner
            .post(
                "shakearound/page/delete",
                Some(access_token.into()),
                json!({ "page_id": page_id.into() }),
            )
            .await
    }

    pub async fn shake_around_bind_pages(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
        page_ids: Vec<i64>,
    ) -> Result<OfficialShakeAroundPageResponse> {
        self.inner
            .post(
                "shakearound/device/bindpage",
                Some(access_token.into()),
                json!({ "device_identifier": device_identifier, "page_ids": page_ids }),
            )
            .await
    }

    pub async fn shake_around_relations_by_device(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
    ) -> Result<OfficialShakeAroundRelationSearchResponse> {
        self.inner
            .post(
                "shakearound/relation/search",
                Some(access_token.into()),
                json!({ "type": 1, "device_identifier": device_identifier }),
            )
            .await
    }

    pub async fn shake_around_relations_by_page(
        &self,
        access_token: impl Into<String>,
        page_id: i64,
        begin: i64,
        count: i64,
    ) -> Result<OfficialShakeAroundRelationSearchResponse> {
        self.inner
            .post(
                "shakearound/relation/search",
                Some(access_token.into()),
                json!({ "type": 2, "page_id": page_id, "begin": begin, "count": count }),
            )
            .await
    }

    pub async fn shake_around_material_upload_image(
        &self,
        access_token: impl Into<String>,
        media: Vec<u8>,
        file_name: impl Into<String>,
        image_type: impl Into<String>,
    ) -> Result<OfficialShakeAroundMaterialUploadResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(media).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "shakearound/material/add",
                Some(access_token.into()),
                vec![("type".to_string(), image_type.into().to_lowercase())],
                form,
            )
            .await
    }

    pub async fn shake_around_device_summary(
        &self,
        access_token: impl Into<String>,
        device_identifier: OfficialShakeAroundDeviceIdentifier,
        begin_date: i64,
        end_date: i64,
    ) -> Result<OfficialShakeAroundStatsSummaryResponse> {
        self.inner
            .post(
                "shakearound/statistics/device",
                Some(access_token.into()),
                json!({ "device_identifier": device_identifier, "begin_date": begin_date, "end_date": end_date }),
            )
            .await
    }

    pub async fn shake_around_devices_summary(
        &self,
        access_token: impl Into<String>,
        date: i64,
        page_index: i64,
    ) -> Result<OfficialShakeAroundStatsDeviceListResponse> {
        self.inner
            .post(
                "shakearound/statistics/devicelist",
                Some(access_token.into()),
                json!({ "date": date, "page_index": page_index }),
            )
            .await
    }

    pub async fn shake_around_page_summary(
        &self,
        access_token: impl Into<String>,
        page_id: i64,
        begin_date: i64,
        end_date: i64,
    ) -> Result<OfficialShakeAroundStatsPageResponse> {
        self.inner
            .post(
                "shakearound/statistics/page",
                Some(access_token.into()),
                json!({ "page_id": page_id, "begin_date": begin_date, "end_date": end_date }),
            )
            .await
    }

    pub async fn shake_around_pages_summary(
        &self,
        access_token: impl Into<String>,
        date: i64,
        page_index: i64,
    ) -> Result<OfficialShakeAroundStatsPageListResponse> {
        self.inner
            .post(
                "shakearound/statistics/pagelist",
                Some(access_token.into()),
                json!({ "date": date, "page_index": page_index }),
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
    ) -> Result<TemplateMessageSendResponse> {
        request.validate()?;
        let response: TemplateMessageSendResponse = self
            .inner
            .post(
                "cgi-bin/message/template/send",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn send_subscribe_template_message(
        &self,
        access_token: impl Into<String>,
        request: TemplateSubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/message/template/subscribe",
                Some(access_token.into()),
                request,
            )
            .await?;
        response.validate_for("official account subscribe template send")?;
        Ok(response)
    }

    pub async fn set_template_industry(
        &self,
        access_token: impl Into<String>,
        industry_id1: impl Into<String>,
        industry_id2: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let industry_id1 = industry_id1.into();
        let industry_id2 = industry_id2.into();
        validate_official_positive_identifier("primary template industry id", &industry_id1)?;
        validate_official_positive_identifier("secondary template industry id", &industry_id2)?;
        if industry_id1 == industry_id2 {
            return Err(WechatError::Config(
                "official account template industry ids must be different".to_string(),
            ));
        }
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/template/api_set_industry",
                Some(access_token.into()),
                json!({
                    "industry_id1": industry_id1,
                    "industry_id2": industry_id2,
                }),
            )
            .await?;
        response.validate_for("official account template industry set")?;
        Ok(response)
    }

    pub async fn get_template_industry(
        &self,
        access_token: impl Into<String>,
    ) -> Result<TemplateIndustryResponse> {
        let response: TemplateIndustryResponse = self
            .inner
            .get("cgi-bin/template/get_industry", Some(access_token.into()))
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn add_template(
        &self,
        access_token: impl Into<String>,
        template_id_short: impl Into<String>,
    ) -> Result<TemplateAddResponse> {
        self.add_template_with_keywords(access_token, template_id_short, Vec::new())
            .await
    }

    pub async fn add_template_with_keywords(
        &self,
        access_token: impl Into<String>,
        template_id_short: impl Into<String>,
        keyword_name_list: Vec<String>,
    ) -> Result<TemplateAddResponse> {
        let template_id_short = template_id_short.into();
        validate_official_identifier("short template id", &template_id_short)?;
        validate_template_keywords(&keyword_name_list)?;
        let mut body = json!({ "template_id_short": template_id_short });
        if !keyword_name_list.is_empty() {
            body["keyword_name_list"] = json!(keyword_name_list);
        }
        let response: TemplateAddResponse = self
            .inner
            .post(
                "cgi-bin/template/api_add_template",
                Some(access_token.into()),
                body,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn list_private_templates(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PrivateTemplateListResponse> {
        let response: PrivateTemplateListResponse = self
            .inner
            .get(
                "cgi-bin/template/get_all_private_template",
                Some(access_token.into()),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn delete_private_template(
        &self,
        access_token: impl Into<String>,
        template_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let template_id = template_id.into();
        validate_official_identifier("template id", &template_id)?;
        let response: WechatStatusResponse = self
            .inner
            .post(
                "cgi-bin/template/del_private_template",
                Some(access_token.into()),
                json!({ "template_id": template_id }),
            )
            .await?;
        response.validate_for("official account private template delete")?;
        Ok(response)
    }

    pub fn user(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "official_account.user")
    }

    pub async fn get_user_info(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        lang: impl Into<String>,
    ) -> Result<OfficialUserInfoResponse> {
        let openid = openid.into();
        let lang = lang.into();
        validate_official_required("user openid", &openid)?;
        validate_official_user_lang(&lang)?;
        self.inner
            .get_with_query(
                "cgi-bin/user/info",
                Some(access_token.into()),
                vec![("openid".to_string(), openid), ("lang".to_string(), lang)],
            )
            .await
    }

    pub async fn batch_get_user_info(
        &self,
        access_token: impl Into<String>,
        request: BatchGetUserInfoRequest,
    ) -> Result<OfficialBatchUserInfoResponse> {
        request.validate()?;
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
    ) -> Result<OfficialUserListResponse> {
        let next_openid = next_openid.into();
        validate_optional_official_identifier("next openid", &next_openid)?;
        self.inner
            .get_with_query(
                "cgi-bin/user/get",
                Some(access_token.into()),
                vec![("next_openid".to_string(), next_openid)],
            )
            .await
    }

    pub async fn update_user_remark(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        remark: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let openid = openid.into();
        let remark = remark.into();
        validate_official_required("user openid", &openid)?;
        if remark.chars().count() > 30 {
            return Err(WechatError::Config(
                "official account user remark must not exceed 30 characters".to_string(),
            ));
        }
        self.inner
            .post(
                "cgi-bin/user/info/updateremark",
                Some(access_token.into()),
                json!({
                    "openid": openid,
                    "remark": remark,
                }),
            )
            .await
    }

    pub async fn change_openid(
        &self,
        access_token: impl Into<String>,
        from_app_id: impl Into<String>,
        openid_list: Vec<String>,
    ) -> Result<OfficialChangeOpenidResponse> {
        let from_app_id = from_app_id.into();
        validate_official_required("source app id", &from_app_id)?;
        validate_official_identifier_batch("openid migration", &openid_list, 100)?;
        self.inner
            .post(
                "cgi-bin/changeopenid",
                Some(access_token.into()),
                json!({
                    "from_appid": from_app_id,
                    "openid_list": openid_list,
                }),
            )
            .await
    }

    pub async fn create_user_tag(
        &self,
        access_token: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<OfficialUserTagResponse> {
        let name = name.into();
        validate_official_tag_name(&name)?;
        self.inner
            .post(
                "cgi-bin/tags/create",
                Some(access_token.into()),
                json!({ "tag": { "name": name } }),
            )
            .await
    }

    pub async fn user_tags(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OfficialUserTagListResponse> {
        self.inner
            .get("cgi-bin/tags/get", Some(access_token.into()))
            .await
    }

    pub async fn update_user_tag(
        &self,
        access_token: impl Into<String>,
        tag_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let tag_id = tag_id.into();
        let name = name.into();
        validate_official_positive_identifier("tag id", &tag_id)?;
        validate_official_tag_name(&name)?;
        self.inner
            .post(
                "cgi-bin/tags/update",
                Some(access_token.into()),
                json!({ "tag": { "id": tag_id, "name": name } }),
            )
            .await
    }

    pub async fn delete_user_tag(
        &self,
        access_token: impl Into<String>,
        tag_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        let tag_id = tag_id.into();
        validate_official_positive_identifier("tag id", &tag_id)?;
        self.inner
            .post(
                "cgi-bin/tags/delete",
                Some(access_token.into()),
                json!({ "tag": { "id": tag_id } }),
            )
            .await
    }

    pub async fn get_user_tag_ids(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<OfficialUserTagIdsResponse> {
        let openid = openid.into();
        validate_official_required("user openid", &openid)?;
        self.inner
            .post(
                "cgi-bin/tags/getidlist",
                Some(access_token.into()),
                json!({ "openid": openid }),
            )
            .await
    }

    pub async fn users_of_tag(
        &self,
        access_token: impl Into<String>,
        tag_id: impl Into<String>,
        next_openid: impl Into<String>,
    ) -> Result<OfficialUsersOfTagResponse> {
        let tag_id = tag_id.into();
        let next_openid = next_openid.into();
        validate_official_positive_identifier("tag id", &tag_id)?;
        validate_optional_official_identifier("next openid", &next_openid)?;
        self.inner
            .post(
                "cgi-bin/user/tag/get",
                Some(access_token.into()),
                json!({ "tagid": tag_id, "next_openid": next_openid }),
            )
            .await
    }

    pub async fn tag_users(
        &self,
        access_token: impl Into<String>,
        openid_list: Vec<String>,
        tag_id: i64,
    ) -> Result<OfficialTagUsersResponse> {
        if tag_id <= 0 {
            return Err(WechatError::Config(
                "official account tag id must be positive".to_string(),
            ));
        }
        validate_official_identifier_batch("tagging", &openid_list, 50)?;
        self.inner
            .post(
                "cgi-bin/tags/members/batchtagging",
                Some(access_token.into()),
                json!({ "openid_list": openid_list, "tagid": tag_id }),
            )
            .await
    }

    pub async fn untag_users(
        &self,
        access_token: impl Into<String>,
        openid_list: Vec<String>,
        tag_id: i64,
    ) -> Result<OfficialTagUsersResponse> {
        if tag_id <= 0 {
            return Err(WechatError::Config(
                "official account tag id must be positive".to_string(),
            ));
        }
        validate_official_identifier_batch("untagging", &openid_list, 50)?;
        self.inner
            .post(
                "cgi-bin/tags/members/batchuntagging",
                Some(access_token.into()),
                json!({ "openid_list": openid_list, "tagid": tag_id }),
            )
            .await
    }

    pub async fn blacklist(
        &self,
        access_token: impl Into<String>,
        begin_openid: impl Into<String>,
    ) -> Result<OfficialBlacklistResponse> {
        let begin_openid = begin_openid.into();
        validate_optional_official_identifier("blacklist begin openid", &begin_openid)?;
        self.inner
            .post(
                "cgi-bin/tags/members/getblacklist",
                Some(access_token.into()),
                json!({ "begin_openid": begin_openid }),
            )
            .await
    }

    pub async fn block_users(
        &self,
        access_token: impl Into<String>,
        openid_list: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        validate_official_identifier_batch("blacklist", &openid_list, 20)?;
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
        validate_official_identifier_batch("blacklist removal", &openid_list, 20)?;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_button: Vec<MenuButton>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuButtonKind {
    Click,
    View,
    ScanCodePush,
    ScanCodeWaitMessage,
    PictureSystemPhoto,
    PicturePhotoOrAlbum,
    PictureWeixin,
    LocationSelect,
    MediaId,
    ViewLimited,
    MiniProgram,
    ArticleId,
    ArticleViewLimited,
}

impl MenuButtonKind {
    pub fn from_code(value: &str) -> Option<Self> {
        match value {
            "click" => Some(Self::Click),
            "view" => Some(Self::View),
            "scancode_push" => Some(Self::ScanCodePush),
            "scancode_waitmsg" => Some(Self::ScanCodeWaitMessage),
            "pic_sysphoto" => Some(Self::PictureSystemPhoto),
            "pic_photo_or_album" => Some(Self::PicturePhotoOrAlbum),
            "pic_weixin" => Some(Self::PictureWeixin),
            "location_select" => Some(Self::LocationSelect),
            "media_id" => Some(Self::MediaId),
            "view_limited" => Some(Self::ViewLimited),
            "miniprogram" => Some(Self::MiniProgram),
            "article_id" => Some(Self::ArticleId),
            "article_view_limited" => Some(Self::ArticleViewLimited),
            _ => None,
        }
    }

    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::View => "view",
            Self::ScanCodePush => "scancode_push",
            Self::ScanCodeWaitMessage => "scancode_waitmsg",
            Self::PictureSystemPhoto => "pic_sysphoto",
            Self::PicturePhotoOrAlbum => "pic_photo_or_album",
            Self::PictureWeixin => "pic_weixin",
            Self::LocationSelect => "location_select",
            Self::MediaId => "media_id",
            Self::ViewLimited => "view_limited",
            Self::MiniProgram => "miniprogram",
            Self::ArticleId => "article_id",
            Self::ArticleViewLimited => "article_view_limited",
        }
    }

    fn uses_key(self) -> bool {
        matches!(
            self,
            Self::Click
                | Self::ScanCodePush
                | Self::ScanCodeWaitMessage
                | Self::PictureSystemPhoto
                | Self::PicturePhotoOrAlbum
                | Self::PictureWeixin
                | Self::LocationSelect
        )
    }
}

impl MenuButton {
    pub fn leaf(
        name: impl Into<String>,
        kind: MenuButtonKind,
        value: impl Into<String>,
    ) -> Result<Self> {
        if kind == MenuButtonKind::MiniProgram {
            return Err(WechatError::Config(
                "official account mini-program menu must use the mini_program constructor"
                    .to_string(),
            ));
        }
        let value = value.into();
        let mut button = Self {
            name: name.into(),
            kind: Some(kind.as_code().to_string()),
            key: None,
            url: None,
            media_id: None,
            appid: None,
            pagepath: None,
            article_id: None,
            sub_button: Vec::new(),
        };
        if kind.uses_key() {
            button.key = Some(value);
        } else if kind == MenuButtonKind::View {
            button.url = Some(value);
        } else if matches!(kind, MenuButtonKind::MediaId | MenuButtonKind::ViewLimited) {
            button.media_id = Some(value);
        } else if matches!(
            kind,
            MenuButtonKind::ArticleId | MenuButtonKind::ArticleViewLimited
        ) {
            button.article_id = Some(value);
        }
        Ok(button)
    }

    pub fn parent(name: impl Into<String>, sub_button: Vec<MenuButton>) -> Self {
        Self {
            name: name.into(),
            kind: None,
            key: None,
            url: None,
            media_id: None,
            appid: None,
            pagepath: None,
            article_id: None,
            sub_button,
        }
    }

    pub fn mini_program(
        name: impl Into<String>,
        url: impl Into<String>,
        appid: impl Into<String>,
        pagepath: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            kind: Some(MenuButtonKind::MiniProgram.as_code().to_string()),
            key: None,
            url: Some(url.into()),
            media_id: None,
            appid: Some(appid.into()),
            pagepath: Some(pagepath.into()),
            article_id: None,
            sub_button: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_at_depth(0)
    }

    fn validate_at_depth(&self, depth: usize) -> Result<()> {
        validate_menu_required("button name", &self.name)?;
        if !self.sub_button.is_empty() {
            if depth > 0 {
                return Err(WechatError::Config(
                    "official account menu supports only one sub-button level".to_string(),
                ));
            }
            if self.sub_button.len() > 5 {
                return Err(WechatError::Config(
                    "official account menu parent supports at most 5 sub-buttons".to_string(),
                ));
            }
            if self.kind.is_some() || self.has_payload() {
                return Err(WechatError::Config(
                    "official account menu parent must not contain a type or action payload"
                        .to_string(),
                ));
            }
            for child in &self.sub_button {
                child.validate_at_depth(depth + 1)?;
            }
            return Ok(());
        }

        let kind = self
            .kind
            .as_deref()
            .and_then(MenuButtonKind::from_code)
            .ok_or_else(|| {
                WechatError::Config(
                    "official account menu leaf has an unsupported or missing type".to_string(),
                )
            })?;
        match kind {
            kind if kind.uses_key() => Self::require_only(
                "key",
                self.key.as_deref(),
                &[
                    self.url.as_ref(),
                    self.media_id.as_ref(),
                    self.appid.as_ref(),
                    self.pagepath.as_ref(),
                    self.article_id.as_ref(),
                ],
            ),
            MenuButtonKind::View => {
                Self::require_only(
                    "URL",
                    self.url.as_deref(),
                    &[
                        self.key.as_ref(),
                        self.media_id.as_ref(),
                        self.appid.as_ref(),
                        self.pagepath.as_ref(),
                        self.article_id.as_ref(),
                    ],
                )?;
                validate_material_http_url(
                    "menu view URL",
                    self.url.as_deref().expect("validated URL"),
                )
            }
            MenuButtonKind::MediaId | MenuButtonKind::ViewLimited => Self::require_only(
                "media id",
                self.media_id.as_deref(),
                &[
                    self.key.as_ref(),
                    self.url.as_ref(),
                    self.appid.as_ref(),
                    self.pagepath.as_ref(),
                    self.article_id.as_ref(),
                ],
            ),
            MenuButtonKind::ArticleId | MenuButtonKind::ArticleViewLimited => Self::require_only(
                "article id",
                self.article_id.as_deref(),
                &[
                    self.key.as_ref(),
                    self.url.as_ref(),
                    self.media_id.as_ref(),
                    self.appid.as_ref(),
                    self.pagepath.as_ref(),
                ],
            ),
            MenuButtonKind::MiniProgram => {
                validate_menu_required(
                    "mini-program fallback URL",
                    self.url.as_deref().unwrap_or_default(),
                )?;
                validate_material_http_url(
                    "menu mini-program fallback URL",
                    self.url.as_deref().expect("validated URL"),
                )?;
                validate_menu_required(
                    "mini-program appid",
                    self.appid.as_deref().unwrap_or_default(),
                )?;
                validate_menu_required(
                    "mini-program page path",
                    self.pagepath.as_deref().unwrap_or_default(),
                )?;
                if self.key.is_some() || self.media_id.is_some() || self.article_id.is_some() {
                    return Err(WechatError::Config(
                        "official account mini-program menu contains conflicting payload fields"
                            .to_string(),
                    ));
                }
                Ok(())
            }
            _ => unreachable!("all key-based menu kinds were handled by the guard"),
        }
    }

    fn has_payload(&self) -> bool {
        self.key.is_some()
            || self.url.is_some()
            || self.media_id.is_some()
            || self.appid.is_some()
            || self.pagepath.is_some()
            || self.article_id.is_some()
    }

    fn require_only<T>(
        kind: &str,
        required: Option<&str>,
        conflicting: &[Option<&T>],
    ) -> Result<()> {
        validate_menu_required(kind, required.unwrap_or_default())?;
        if conflicting.iter().any(Option::is_some) {
            return Err(WechatError::Config(format!(
                "official account menu {kind} action contains conflicting payload fields"
            )));
        }
        Ok(())
    }
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
pub struct OauthUserInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub sex: Option<i64>,
    #[serde(default)]
    pub province: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub headimgurl: Option<String>,
    #[serde(default)]
    pub privilege: Vec<String>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMenuRequest {
    pub button: Vec<MenuButton>,
}

impl CreateMenuRequest {
    pub fn new(button: Vec<MenuButton>) -> Self {
        Self { button }
    }

    pub fn validate(&self) -> Result<()> {
        validate_menu_buttons(&self.button)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalMenuRequest {
    pub button: Vec<MenuButton>,
    pub matchrule: MatchRule,
}

impl CreateConditionalMenuRequest {
    pub fn validate(&self) -> Result<()> {
        validate_menu_buttons(&self.button)?;
        self.matchrule.validate()
    }
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

impl MatchRule {
    pub fn validate(&self) -> Result<()> {
        let fields = [
            ("tag id", self.tag_id.as_deref()),
            ("sex", self.sex.as_deref()),
            ("country", self.country.as_deref()),
            ("province", self.province.as_deref()),
            ("city", self.city.as_deref()),
            ("client platform type", self.client_platform_type.as_deref()),
            ("language", self.language.as_deref()),
        ];
        if fields.iter().all(|(_, value)| value.is_none()) {
            return Err(WechatError::Config(
                "official account conditional menu requires at least one match rule".to_string(),
            ));
        }
        for (kind, value) in fields {
            if let Some(value) = value {
                validate_menu_required(kind, value)?;
            }
        }
        if self
            .sex
            .as_deref()
            .is_some_and(|value| !matches!(value, "1" | "2"))
        {
            return Err(WechatError::Config(
                "official account conditional menu sex must be 1 or 2".to_string(),
            ));
        }
        if self
            .client_platform_type
            .as_deref()
            .is_some_and(|value| !matches!(value, "1" | "2" | "3"))
        {
            return Err(WechatError::Config(
                "official account conditional menu client platform must be 1, 2, or 3".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_menu_buttons(buttons: &[MenuButton]) -> Result<()> {
    if buttons.is_empty() || buttons.len() > 3 {
        return Err(WechatError::Config(
            "official account menu must contain between 1 and 3 top-level buttons".to_string(),
        ));
    }
    for button in buttons {
        button.validate()?;
    }
    Ok(())
}

fn validate_menu_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "official account menu {kind} must not be blank"
        )));
    }
    Ok(())
}

fn ensure_official_response_success(
    operation: &str,
    errcode: Option<i64>,
    errmsg: Option<&str>,
) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg.unwrap_or(operation).to_string(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalMenuResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub menuid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl CreateConditionalMenuResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account conditional menu create",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_official_identifier(
            "conditional menu response id",
            self.menuid.as_deref().unwrap_or_default(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub menu: Option<OfficialMenuInfo>,
    #[serde(default)]
    pub conditionalmenu: Vec<OfficialConditionalMenuInfo>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MenuGetResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account menu get",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let menu = self.menu.as_ref().ok_or_else(|| {
            WechatError::Config("official account menu response requires menu".to_string())
        })?;
        validate_menu_response_buttons("menu response", &menu.button)?;
        for conditional in &self.conditionalmenu {
            validate_menu_response_buttons("conditional menu response", &conditional.button)?;
            if conditional.matchrule.is_none() {
                return Err(WechatError::Config(
                    "official account conditional menu response requires matchrule".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialMenuInfo {
    #[serde(default)]
    pub button: Vec<MenuResponseButton>,
    #[serde(default)]
    pub menuid: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponseButton {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub pagepath: Option<String>,
    #[serde(default)]
    pub article_id: Option<String>,
    #[serde(default)]
    pub sub_button: Vec<MenuResponseButton>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

fn validate_menu_response_buttons(kind: &str, buttons: &[MenuResponseButton]) -> Result<()> {
    if buttons.is_empty() || buttons.len() > 3 {
        return Err(WechatError::Config(format!(
            "official account {kind} must contain between 1 and 3 top-level buttons"
        )));
    }
    for button in buttons {
        menu_response_button_as_request(button)?.validate()?;
    }
    Ok(())
}

fn menu_response_button_as_request(button: &MenuResponseButton) -> Result<MenuButton> {
    let name = button.name.clone().ok_or_else(|| {
        WechatError::Config("official account menu response button requires name".to_string())
    })?;
    let sub_button = button
        .sub_button
        .iter()
        .map(menu_response_button_as_request)
        .collect::<Result<Vec<_>>>()?;
    Ok(MenuButton {
        name,
        kind: non_blank_menu_response_value(&button.kind),
        key: non_blank_menu_response_value(&button.key),
        url: non_blank_menu_response_value(&button.url),
        media_id: non_blank_menu_response_value(&button.media_id),
        appid: non_blank_menu_response_value(&button.appid),
        pagepath: non_blank_menu_response_value(&button.pagepath),
        article_id: non_blank_menu_response_value(&button.article_id),
        sub_button,
    })
}

fn non_blank_menu_response_value(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .filter(|value| !value.trim().is_empty())
        .cloned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuMatchRuleResponse {
    #[serde(default)]
    pub group_id: Option<Value>,
    #[serde(default)]
    pub tag_id: Option<Value>,
    #[serde(default)]
    pub sex: Option<Value>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub province: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub client_platform_type: Option<Value>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialConditionalMenuInfo {
    #[serde(default)]
    pub button: Vec<MenuResponseButton>,
    #[serde(default)]
    pub matchrule: Option<MenuMatchRuleResponse>,
    #[serde(default)]
    pub menuid: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub is_menu_open: Option<i64>,
    #[serde(default)]
    pub selfmenu_info: Option<CurrentSelfMenuInfo>,
    #[serde(flatten)]
    pub extra: Value,
}

impl CurrentSelfMenuResponse {
    pub fn is_open(&self) -> bool {
        self.is_menu_open == Some(1)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account current self menu get",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if !matches!(self.is_menu_open, Some(0 | 1)) {
            return Err(WechatError::Config(
                "official account current self menu response requires is_menu_open 0 or 1"
                    .to_string(),
            ));
        }
        if self.is_open() {
            let menu = self.selfmenu_info.as_ref().ok_or_else(|| {
                WechatError::Config(
                    "official account open current self menu requires selfmenu_info".to_string(),
                )
            })?;
            validate_current_self_menu_buttons(&menu.button)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuInfo {
    #[serde(default)]
    pub button: Vec<CurrentSelfMenuButton>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuButton {
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub pagepath: Option<String>,
    #[serde(default)]
    pub article_id: Option<String>,
    #[serde(default)]
    pub sub_button: Option<CurrentSelfMenuSubButtons>,
    #[serde(default)]
    pub news_info: Option<CurrentSelfMenuNewsInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuSubButtons {
    #[serde(default)]
    pub list: Vec<CurrentSelfMenuButton>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuNewsInfo {
    #[serde(default)]
    pub list: Vec<CurrentSelfMenuNewsItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSelfMenuNewsItem {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub show_cover: Option<i64>,
    #[serde(default)]
    pub cover_url: Option<String>,
    #[serde(default)]
    pub content_url: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

fn validate_current_self_menu_buttons(buttons: &[CurrentSelfMenuButton]) -> Result<()> {
    if buttons.is_empty() || buttons.len() > 3 {
        return Err(WechatError::Config(
            "official account current self menu must contain between 1 and 3 top-level buttons"
                .to_string(),
        ));
    }
    for button in buttons {
        validate_menu_required(
            "current self menu button name",
            button.name.as_deref().unwrap_or_default(),
        )?;
        if let Some(sub_buttons) = &button.sub_button {
            if sub_buttons.list.is_empty() || sub_buttons.list.len() > 5 {
                return Err(WechatError::Config(
                    "official account current self menu parent must contain between 1 and 5 sub-buttons"
                        .to_string(),
                ));
            }
            for child in &sub_buttons.list {
                validate_menu_required(
                    "current self menu sub-button name",
                    child.name.as_deref().unwrap_or_default(),
                )?;
                if child.sub_button.is_some() {
                    return Err(WechatError::Config(
                        "official account current self menu supports one sub-button level"
                            .to_string(),
                    ));
                }
            }
        } else {
            validate_menu_required(
                "current self menu button type",
                button.kind.as_deref().unwrap_or_default(),
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuTryMatchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub button: Vec<MenuResponseButton>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MenuTryMatchResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account menu try-match",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_menu_response_buttons("try-match response", &self.button)
    }
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

impl MassSendFilter {
    pub fn validate(&self) -> Result<()> {
        if self.is_to_all && self.tag_id.is_some() {
            return Err(WechatError::Config(
                "official account mass-send all filter must not include tag_id".to_string(),
            ));
        }
        if !self.is_to_all && self.tag_id.is_none_or(|tag_id| tag_id <= 0) {
            return Err(WechatError::Config(
                "official account tag-filtered mass send requires a positive tag_id".to_string(),
            ));
        }
        Ok(())
    }
}

impl MassSendAllRequest {
    pub fn validate(&self) -> Result<()> {
        self.filter.validate()?;
        validate_mass_message(&self.msgtype, &self.message)?;
        validate_binary_option("mass send_ignore_reprint", self.send_ignore_reprint)
    }
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

impl MassSendOpenIdsRequest {
    pub fn validate(&self) -> Result<()> {
        if !(2..=10_000).contains(&self.touser.len()) {
            return Err(WechatError::Config(
                "official account openid mass send requires between 2 and 10000 recipients"
                    .to_string(),
            ));
        }
        let mut unique = HashSet::with_capacity(self.touser.len());
        for openid in &self.touser {
            validate_official_identifier("mass-send recipient openid", openid)?;
            if !unique.insert(openid.trim()) {
                return Err(WechatError::Config(
                    "official account openid mass send contains duplicate recipients".to_string(),
                ));
            }
        }
        validate_mass_message(&self.msgtype, &self.message)?;
        validate_binary_option("mass send_ignore_reprint", self.send_ignore_reprint)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassPreviewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub towxname: Option<String>,
    pub msgtype: String,
    #[serde(flatten)]
    pub message: Value,
}

impl MassPreviewRequest {
    pub fn validate(&self) -> Result<()> {
        match (self.touser.as_deref(), self.towxname.as_deref()) {
            (Some(openid), None) => {
                validate_official_identifier("mass-preview recipient openid", openid)?
            }
            (None, Some(wxname)) => {
                validate_official_identifier("mass-preview recipient WeChat name", wxname)?
            }
            _ => {
                return Err(WechatError::Config(
                    "official account mass preview requires exactly one of touser or towxname"
                        .to_string(),
                ));
            }
        }
        validate_mass_message(&self.msgtype, &self.message)
    }
}

fn validate_binary_option(kind: &str, value: Option<i64>) -> Result<()> {
    if value.is_some_and(|value| !matches!(value, 0 | 1)) {
        return Err(WechatError::Config(format!(
            "official account {kind} must be 0 or 1"
        )));
    }
    Ok(())
}

fn validate_mass_message(msgtype: &str, message: &Value) -> Result<()> {
    let payload_kind = match msgtype {
        "text" => "content",
        "mpnews" | "voice" | "image" | "mpvideo" => "media_id",
        "wxcard" => "card_id",
        _ => {
            return Err(WechatError::Config(format!(
                "official account mass message type {msgtype:?} is unsupported"
            )));
        }
    };
    let message = message.as_object().ok_or_else(|| {
        WechatError::Config("official account mass message payload must be an object".to_string())
    })?;
    let typed_payload = message
        .get(msgtype)
        .and_then(Value::as_object)
        .ok_or_else(|| {
            WechatError::Config(format!(
                "official account mass message requires an object payload named {msgtype}"
            ))
        })?;
    let payload = typed_payload
        .get(payload_kind)
        .and_then(Value::as_str)
        .unwrap_or_default();
    validate_official_identifier(&format!("mass message {payload_kind}"), payload)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassSendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msg_id: Option<i64>,
    #[serde(default)]
    pub msg_data_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MassSendResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account mass send",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.msg_id.is_none_or(|msg_id| msg_id <= 0) {
            return Err(WechatError::Config(
                "official account mass-send response requires a positive msg_id".to_string(),
            ));
        }
        if self.msg_data_id.is_some_and(|msg_data_id| msg_data_id <= 0) {
            return Err(WechatError::Config(
                "official account mass-send response msg_data_id must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassPreviewResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msg_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MassPreviewResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account mass preview",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.msg_id.is_some_and(|msg_id| msg_id <= 0) {
            return Err(WechatError::Config(
                "official account mass-preview response msg_id must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msg_id: Option<i64>,
    #[serde(default)]
    pub msg_status: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassStatusKind {
    Sending,
    Success,
    Failed,
    Deleted,
    Other,
}

impl MassStatusKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("SENDING") {
            Self::Sending
        } else if value.eq_ignore_ascii_case("SEND_SUCCESS") {
            Self::Success
        } else if value.eq_ignore_ascii_case("SEND_FAIL") {
            Self::Failed
        } else if value.eq_ignore_ascii_case("DELETE") {
            Self::Deleted
        } else {
            Self::Other
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Success | Self::Failed | Self::Deleted)
    }
}

impl MassStatusResponse {
    pub fn status_kind(&self) -> Option<MassStatusKind> {
        self.msg_status.as_deref().map(MassStatusKind::from_code)
    }

    pub fn is_success(&self) -> bool {
        self.status_kind() == Some(MassStatusKind::Success)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account mass status",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.msg_id.is_none_or(|msg_id| msg_id <= 0) {
            return Err(WechatError::Config(
                "official account mass-status response requires a positive msg_id".to_string(),
            ));
        }
        validate_official_required(
            "mass-status response status",
            self.msg_status.as_deref().unwrap_or_default(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialAutoReplyInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub is_add_friend_reply_open: Option<i64>,
    #[serde(default)]
    pub is_autoreply_open: Option<i64>,
    #[serde(default)]
    pub add_friend_autoreply_info: Option<Value>,
    #[serde(default)]
    pub message_default_autoreply_info: Option<Value>,
    #[serde(default)]
    pub keyword_autoreply_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialSemanticQueryRequest {
    pub query: String,
    pub category: String,
    pub appid: String,
    #[serde(flatten)]
    pub optional: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialSemanticQueryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub semantic: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialPoiMutationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub poi_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialPoiGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub business: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialPoiListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub business_list: Vec<Value>,
    #[serde(default)]
    pub total_count: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceMessageRequest {
    pub device_type: String,
    pub device_id: String,
    pub open_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceAuthorizeRequest {
    pub device_num: String,
    pub device_list: Vec<Value>,
    pub op_type: String,
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceBindRequest {
    pub ticket: String,
    pub device_id: String,
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceMessageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ret: Option<i64>,
    #[serde(default)]
    pub ret_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceCreateQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub device_num: Option<i64>,
    #[serde(default)]
    pub code_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceAuthorizeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub resp: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceCreateIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub resp_msg: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialDeviceBindResponse {
    #[serde(default)]
    pub base_resp: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGoodsProductRequest {
    pub product: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGoodsProductAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status_ticket: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGoodsProductStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGoodsProductGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub product: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrIdCardResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "type")]
    pub id_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub addr: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub nationality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrBankCardResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrVehicleLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub id_num: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub sex: Option<String>,
    #[serde(default)]
    pub nationality: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub birth_date: Option<String>,
    #[serde(default)]
    pub issue_date: Option<String>,
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub official_seal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrBizLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub reg_num: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub legal_representative: Option<String>,
    #[serde(default)]
    pub enterprise_name: Option<String>,
    #[serde(default)]
    pub type_of_organization: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub type_of_enterprise: Option<String>,
    #[serde(default)]
    pub business_scope: Option<String>,
    #[serde(default)]
    pub registered_capital: Option<String>,
    #[serde(default)]
    pub paid_in_capital: Option<String>,
    #[serde(default)]
    pub valid_period: Option<String>,
    #[serde(default)]
    pub registered_date: Option<String>,
    #[serde(default)]
    pub cert_position: Option<Value>,
    #[serde(default)]
    pub img_size: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrCommonResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub img_size: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialOcrPlateNumberResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreBaseInfo {
    pub name: String,
    pub longitude: String,
    pub latitude: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    pub category: String,
    pub telephone: String,
    pub photo: String,
    pub license: String,
    #[serde(rename = "introduct")]
    pub introduction: String,
    pub districtid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreCreateRequest {
    pub poi_id: String,
    pub map_poi_id: String,
    pub pic_list: String,
    pub contract_phone: String,
    pub credential: String,
    pub qualification_list: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreUpdateRequest {
    pub map_poi_id: String,
    pub poi_id: String,
    pub hour: String,
    pub contract_phone: String,
    pub pic_list: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreCategoryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreDistrictResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data_version: Option<String>,
    #[serde(default)]
    pub result: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreSearchMapResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub first_catid: Option<i64>,
    #[serde(default)]
    pub second_catid: Option<i64>,
    #[serde(default)]
    pub qualification_list: Option<String>,
    #[serde(default)]
    pub headimg_mediaid: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub intro: Option<String>,
    #[serde(default)]
    pub org_code: Option<String>,
    #[serde(default)]
    pub other_files: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreCreateFromMapResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub error: Option<Value>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreUpdateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub business: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialStoreListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub business_list: Option<Vec<Value>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiHomePageStruct {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiSetHomePageRequest {
    pub shop_id: i64,
    pub template_id: i64,
    #[serde(rename = "struct")]
    pub struct_data: OfficialWifiHomePageStruct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiCardSetRequest {
    pub shop_id: i64,
    pub card_id: String,
    pub card_describe: String,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiSummaryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiQrCodeUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiCardGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiDeviceListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiShopGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialWifiShopListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialCallbackIpResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ip_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialCallbackCheckResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub dns: Vec<Value>,
    #[serde(default)]
    pub ping: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card_id_list: Vec<String>,
    #[serde(default)]
    pub total_num: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardUpdateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub send_check: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceSession {
    #[serde(default, rename = "createtime")]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceAccount {
    #[serde(default)]
    pub kf_account: Option<String>,
    #[serde(default)]
    pub kf_nick: Option<String>,
    #[serde(default)]
    pub kf_id: Option<String>,
    #[serde(default)]
    pub kf_headimgurl: Option<String>,
    #[serde(default)]
    pub kf_wx: Option<String>,
    #[serde(default)]
    pub invite_wx: Option<String>,
    #[serde(default)]
    pub invite_expire_time: Option<i64>,
    #[serde(default)]
    pub invite_status: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub accepted_case: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceAccountListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub kf_list: Vec<CustomerServiceAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceOnlineAccountListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub kf_online_list: Vec<CustomerServiceAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceSessionListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub sessionlist: Vec<CustomerServiceSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceWaitCase {
    #[serde(default)]
    pub latest_time: Option<i64>,
    #[serde(default)]
    pub openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceWaitCaseListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub waitcaselist: Vec<CustomerServiceWaitCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceSessionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "createtime")]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub kf_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessageRecordRequest {
    pub starttime: i64,
    pub endtime: i64,
    pub msgid: i64,
    pub number: i64,
}

impl CustomerServiceMessageRecordRequest {
    pub fn validate(&self) -> Result<()> {
        if self.starttime <= 0 || self.endtime <= 0 {
            return Err(WechatError::Config(
                "official account customer service record timestamps must be positive".to_string(),
            ));
        }
        if self.starttime > self.endtime {
            return Err(WechatError::Config(
                "official account customer service record start time must not exceed end time"
                    .to_string(),
            ));
        }
        if self.msgid < 0 {
            return Err(WechatError::Config(
                "official account customer service record message id must not be negative"
                    .to_string(),
            ));
        }
        if !(1..=10_000).contains(&self.number) {
            return Err(WechatError::Config(
                "official account customer service record page size must be between 1 and 10000"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessageRecord {
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub opercode: Option<i64>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub worker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessageRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub recordlist: Vec<CustomerServiceMessageRecord>,
    #[serde(default)]
    pub number: Option<i64>,
    #[serde(default)]
    pub msgid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<Value>,
    pub scene: String,
    pub title: String,
    pub data: Value,
}

impl TemplateSubscribeMessageRequest {
    pub fn validate(&self) -> Result<()> {
        validate_official_identifier("subscribe template recipient openid", &self.touser)?;
        validate_official_identifier("subscribe template id", &self.template_id)?;
        validate_material_http_url("subscribe template URL", &self.url)?;
        validate_official_identifier("subscribe template scene", &self.scene)?;
        validate_official_required("subscribe template title", &self.title)?;
        if let Some(miniprogram) = &self.miniprogram {
            let miniprogram: TemplateMiniProgram = serde_json::from_value(miniprogram.clone())
                .map_err(|error| {
                    WechatError::Config(format!(
                        "official account subscribe template miniprogram is invalid: {error}"
                    ))
                })?;
            miniprogram.validate()?;
        }
        validate_template_data(&self.data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialChangeOpenidResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result_list: Vec<OfficialChangeOpenidResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialChangeOpenidResult {
    #[serde(default)]
    pub ori_openid: Option<String>,
    #[serde(default)]
    pub new_openid: Option<String>,
    #[serde(default)]
    pub err_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserTag {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserTagResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag: Option<OfficialUserTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tags: Vec<OfficialUserTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserTagIdsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tagid_list: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUsersOfTagData {
    #[serde(default)]
    pub openid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUsersOfTagResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub data: Option<OfficialUsersOfTagData>,
    #[serde(default)]
    pub next_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialTagUsersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid_list: Vec<String>,
    #[serde(default)]
    pub tagid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub subscribe: Option<i64>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub sex: Option<i64>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub province: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub headimgurl: Option<String>,
    #[serde(default)]
    pub subscribe_time: Option<i64>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub tagid_list: Vec<i64>,
    #[serde(default)]
    pub subscribe_scene: Option<String>,
    #[serde(default)]
    pub qr_scene: Option<i64>,
    #[serde(default)]
    pub qr_scene_str: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialBatchUserInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub user_info_list: Vec<OfficialUserInfoResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserOpenidData {
    #[serde(default)]
    pub openid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialUserListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub data: Option<OfficialUserOpenidData>,
    #[serde(default)]
    pub next_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialBlacklistResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub data: Option<OfficialUserOpenidData>,
    #[serde(default)]
    pub next_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideAdviserRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_headimgurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideQrcodeRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrcode_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerChatRecordRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideConfigRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub is_delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_fast_reply_list: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_auto_reply: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_auto_reply_plus: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideAdviserConfigRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub is_delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_keyword: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_auto_reply: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyer {
    pub openid: String,
    pub buyer_nickname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerRelationRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub buyer_list: Vec<OfficialGuideBuyer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideDeleteBuyerRelationRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub openid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerRelationsRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub page: i64,
    pub num: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideRebindBuyerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_guide_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_guide_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_guide_openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_guide_openid: Option<String>,
    pub openid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideUpdateBuyerRelationRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub openid: String,
    pub buyer_nickname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerRelationByGuideRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerTagRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub tag_value: String,
    pub openid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyersByTagRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_count: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideDeleteBuyerTagRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub tag_value: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub openid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerDisplayTagRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub openid: String,
    pub display_tag_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideBuyerDisplayTagsRequest {
    #[serde(flatten)]
    pub account: OfficialGuideAccountRequest,
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideCardMaterialRequest {
    pub media_id: String,
    #[serde(rename = "type")]
    pub material_type: i64,
    pub title: String,
    pub path: String,
    pub appid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialGuideDeleteCardMaterialRequest {
    #[serde(rename = "type")]
    pub material_type: i64,
    pub title: String,
    pub path: String,
    pub appid: String,
}

macro_rules! official_value_response {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            #[serde(default)]
            pub errcode: Option<i64>,
            #[serde(default)]
            pub errmsg: Option<String>,
            $(
                #[serde(default)]
                pub $field: Option<$ty>,
            )*
        }
    };
}

official_value_response!(OfficialGuideGetAdviserResponse {
    guide_account: String,
    guide_headimgurl: String,
    guide_nickname: String,
    guide_openid: Value,
});
official_value_response!(OfficialGuideGetAdvisersResponse { total_num: i64, list: Vec<Value> });
official_value_response!(OfficialGuideChatRecordsResponse { total_num: i64, msg_list: Vec<Value> });
official_value_response!(OfficialGuideConfigResponse {
    guide_fast_reply_list: Vec<Value>,
    guide_auto_reply: Value,
    updatetime: i64,
    guide_auto_reply_plus: Value,
});
official_value_response!(OfficialGuideAdviserConfigResponse {
    black_keyword: Value,
    guide_auto_reply: Value,
});
official_value_response!(OfficialGuideCreateGroupResponse { group_id: Value });
official_value_response!(OfficialGuideGroupListResponse { group_list: Vec<Value> });
official_value_response!(OfficialGuideGroupResponse { guide_list: Vec<Value>, total_num: i64 });
official_value_response!(OfficialGuideGroupByGuideResponse { group_id_list: Vec<i64> });
official_value_response!(OfficialGuideBuyerRelationResponse { buyer_resp: Vec<Value> });
official_value_response!(OfficialGuideBuyerRelationListResponse { total_num: i64, list: Vec<Value> });
official_value_response!(OfficialGuideGetBuyerRelationResponse {
    openid: String,
    guide_account: String,
    guide_openid: String,
    buyer_nickname: String,
    create_time: i64,
});
official_value_response!(OfficialGuideTagOptionResponse { options: Vec<Value> });
official_value_response!(OfficialGuideBuyerTagsResponse { tag_values: Vec<String> });
official_value_response!(OfficialGuideBuyersByTagResponse { openid_list: Vec<String> });
official_value_response!(OfficialGuideBuyerDisplayTagsResponse { display_tag_list: Vec<String> });
official_value_response!(OfficialGuideCardMaterialResponse { card_list: Vec<Value> });
official_value_response!(OfficialGuideImageMaterialResponse { model_list: Vec<Value>, total_num: i64 });
official_value_response!(OfficialGuideWordMaterialResponse { word_list: Vec<Value>, total_num: i64 });

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundAccountRegisterRequest {
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub industry_id: String,
    pub qualification_cert_urls: Vec<String>,
    pub apply_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundUserRequest {
    pub ticket: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_poi: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundDeviceApplyRequest {
    pub quantity: i64,
    pub apply_reason: String,
    pub comment: String,
    pub poi_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundDeviceIdentifier {
    pub device_id: i64,
    pub uuid: String,
    pub major: i64,
    pub minor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundDeviceSearchRequest {
    #[serde(rename = "type")]
    pub search_type: i64,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub device_identifiers: Vec<OfficialShakeAroundDeviceIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundPageInfoRequest {
    pub title: String,
    pub description: String,
    pub page_url: String,
    pub comment: String,
    pub icon_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialShakeAroundPageUpdateRequest {
    #[serde(flatten)]
    pub page: OfficialShakeAroundPageInfoRequest,
    pub page_id: i64,
}

official_value_response!(OfficialShakeAroundAccountRegisterResponse { data: Value });
official_value_response!(OfficialShakeAroundUserResponse { data: Value });
official_value_response!(OfficialShakeAroundDeviceApplyResponse { data: Value });
official_value_response!(OfficialShakeAroundDeviceApplyStatusResponse { data: Value });
official_value_response!(OfficialShakeAroundDeviceBindPoiResponse { data: Value });
official_value_response!(OfficialShakeAroundDeviceSearchResponse { data: Value });
official_value_response!(OfficialShakeAroundGroupResponse { data: Value });
official_value_response!(OfficialShakeAroundGroupListResponse { data: Value });
official_value_response!(OfficialShakeAroundGroupDetailResponse { data: Value });
official_value_response!(OfficialShakeAroundPageResponse { data: Value });
official_value_response!(OfficialShakeAroundPageListResponse { data: Value });
official_value_response!(OfficialShakeAroundRelationSearchResponse { data: Value });
official_value_response!(OfficialShakeAroundMaterialUploadResponse { data: Value });
official_value_response!(OfficialShakeAroundStatsSummaryResponse { data: Vec<Value> });
official_value_response!(OfficialShakeAroundStatsDeviceListResponse {
    data: Value,
    date: i64,
    total_count: i64,
    page_index: i64,
});
official_value_response!(OfficialShakeAroundStatsPageResponse { data: Vec<Value> });
official_value_response!(OfficialShakeAroundStatsPageListResponse {
    data: Value,
    date: i64,
    total_count: i64,
    page_index: i64,
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

impl WechatStatusResponse {
    pub fn validate_for(&self, operation: &str) -> Result<()> {
        if let Some(code) = self.errcode.filter(|code| *code != 0) {
            return Err(WechatError::Api {
                code,
                message: self.errmsg.as_deref().unwrap_or(operation).to_string(),
            });
        }
        Ok(())
    }
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
pub struct CardDateInfo {
    #[serde(default)]
    pub date_type: Option<String>,
    #[serde(default)]
    pub begin_timestamp: Option<i64>,
    #[serde(default)]
    pub end_timestamp: Option<i64>,
    #[serde(default)]
    pub fixed_term: Option<i64>,
    #[serde(default)]
    pub fixed_begin_term: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSku {
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub total_quantity: Option<i64>,
    #[serde(default)]
    pub use_custom_code: Option<bool>,
    #[serde(default)]
    pub bind_openid: Option<bool>,
    #[serde(default)]
    pub can_share: Option<bool>,
    #[serde(default)]
    pub can_give_friend: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardBaseInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub logo_url: Option<String>,
    #[serde(default)]
    pub code_type: Option<String>,
    #[serde(default)]
    pub brand_name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub sub_title: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub notice: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub date_info: Option<CardDateInfo>,
    #[serde(default)]
    pub sku: Option<CardSku>,
    #[serde(default)]
    pub get_limit: Option<i64>,
    #[serde(default)]
    pub use_limit: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardStatusKind {
    PendingReview,
    ReviewFailed,
    Approved,
    MerchantDeleted,
    Dispatched,
    UserDispatched,
    Other,
}

impl CardBaseInfo {
    pub fn status_kind(&self) -> Option<CardStatusKind> {
        self.status.as_deref().map(|status| match status {
            "CARD_STATUS_NOT_VERIFY" => CardStatusKind::PendingReview,
            "CARD_STATUS_VERIFY_FAIL" | "CARD_STATUS_VERIFY_FALL" => CardStatusKind::ReviewFailed,
            "CARD_STATUS_VERIFY_OK" => CardStatusKind::Approved,
            "CARD_STATUS_DELETE" | "CARD_STATUS_USER_DELETE" => CardStatusKind::MerchantDeleted,
            "CARD_STATUS_DISPATCH" => CardStatusKind::Dispatched,
            "CARD_STATUS_USER_DISPATCH" => CardStatusKind::UserDispatched,
            _ => CardStatusKind::Other,
        })
    }

    pub fn is_approved(&self) -> bool {
        self.status_kind() == Some(CardStatusKind::Approved)
    }

    pub fn needs_review(&self) -> bool {
        self.status_kind() == Some(CardStatusKind::PendingReview)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTypeDetail {
    #[serde(default)]
    pub base_info: Option<CardBaseInfo>,
    #[serde(default)]
    pub advanced_info: Option<Value>,
    #[serde(default)]
    pub deal_detail: Option<String>,
    #[serde(default)]
    pub least_cost: Option<i64>,
    #[serde(default)]
    pub reduce_cost: Option<i64>,
    #[serde(default)]
    pub discount: Option<i64>,
    #[serde(default)]
    pub gift: Option<String>,
    #[serde(default)]
    pub default_detail: Option<String>,
    #[serde(default)]
    pub supply_bonus: Option<bool>,
    #[serde(default)]
    pub supply_balance: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardInfo {
    #[serde(default)]
    pub card_type: Option<String>,
    #[serde(default)]
    pub groupon: Option<CardTypeDetail>,
    #[serde(default)]
    pub cash: Option<CardTypeDetail>,
    #[serde(default)]
    pub discount: Option<CardTypeDetail>,
    #[serde(default)]
    pub gift: Option<CardTypeDetail>,
    #[serde(default)]
    pub general_coupon: Option<CardTypeDetail>,
    #[serde(default)]
    pub member_card: Option<CardTypeDetail>,
    #[serde(default)]
    pub scenic_ticket: Option<CardTypeDetail>,
    #[serde(default)]
    pub movie_ticket: Option<CardTypeDetail>,
    #[serde(default)]
    pub boarding_pass: Option<CardTypeDetail>,
    #[serde(default)]
    pub meeting_ticket: Option<CardTypeDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCodeCardInfo {
    #[serde(default)]
    pub card_id: Option<String>,
    #[serde(default)]
    pub begin_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card: Option<CardInfo>,
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
    pub card: Option<CardCodeCardInfo>,
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

    pub fn media(
        touser: impl Into<String>,
        kind: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<Self> {
        let kind = kind.into();
        if !matches!(kind.as_str(), "image" | "voice" | "mpnews") {
            return Err(WechatError::Config(
                "official account customer service media type must be image, voice, or mpnews"
                    .to_string(),
            ));
        }
        let payload = json!({ "media_id": media_id.into() });
        let mut message = Self {
            touser: touser.into(),
            msgtype: kind.clone(),
            text: None,
            image: None,
            voice: None,
            video: None,
            music: None,
            news: None,
            mpnews: None,
            wxcard: None,
            miniprogrampage: None,
            customservice: None,
        };
        match kind.as_str() {
            "image" => message.image = Some(payload),
            "voice" => message.voice = Some(payload),
            "mpnews" => message.mpnews = Some(payload),
            _ => unreachable!("customer service media type was checked"),
        }
        message.validate()?;
        Ok(message)
    }

    pub fn with_customer_service_account(mut self, account: impl Into<String>) -> Result<Self> {
        self.customservice = Some(json!({ "kf_account": account.into() }));
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<()> {
        validate_official_required("customer service recipient", &self.touser)?;
        let payloads = [
            ("text", &self.text),
            ("image", &self.image),
            ("voice", &self.voice),
            ("video", &self.video),
            ("music", &self.music),
            ("news", &self.news),
            ("mpnews", &self.mpnews),
            ("wxcard", &self.wxcard),
            ("miniprogrampage", &self.miniprogrampage),
        ];
        let Some((_, payload)) = payloads
            .iter()
            .find(|(kind, _)| *kind == self.msgtype.as_str())
        else {
            return Err(WechatError::Config(format!(
                "unsupported official account customer service message type `{}`",
                self.msgtype
            )));
        };
        if payload.is_none() {
            return Err(WechatError::Config(format!(
                "official account customer service `{}` payload is required",
                self.msgtype
            )));
        }
        if payloads
            .iter()
            .filter(|(_, payload)| payload.is_some())
            .count()
            != 1
        {
            return Err(WechatError::Config(
                "official account customer service message must contain exactly one payload"
                    .to_string(),
            ));
        }
        validate_customer_service_payload(&self.msgtype, payload.as_ref().expect("checked"))?;
        if let Some(customservice) = &self.customservice {
            let account = customservice
                .as_object()
                .and_then(|value| value.get("kf_account"))
                .and_then(Value::as_str)
                .unwrap_or_default();
            validate_official_required("customer service sender account", account)?;
        }
        Ok(())
    }
}

fn validate_customer_service_payload(kind: &str, payload: &Value) -> Result<()> {
    let Some(payload) = payload.as_object() else {
        return Err(WechatError::Config(format!(
            "official account customer service `{kind}` payload must be an object"
        )));
    };
    if payload.is_empty() {
        return Err(WechatError::Config(format!(
            "official account customer service `{kind}` payload must not be empty"
        )));
    }
    let required = match kind {
        "text" => Some("content"),
        "image" | "voice" | "video" | "mpnews" => Some("media_id"),
        "wxcard" => Some("card_id"),
        "miniprogrampage" => Some("pagepath"),
        "music" => Some("musicurl"),
        "news" => Some("articles"),
        _ => None,
    };
    if let Some(field) = required {
        let present = match payload.get(field) {
            Some(Value::String(value)) => !value.trim().is_empty(),
            Some(Value::Array(value)) => !value.is_empty(),
            Some(Value::Object(value)) => !value.is_empty(),
            Some(Value::Null) | None => false,
            Some(_) => true,
        };
        if !present {
            return Err(WechatError::Config(format!(
                "official account customer service `{kind}` payload requires `{field}`"
            )));
        }
    }
    Ok(())
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

impl Article {
    pub fn validate(&self) -> Result<()> {
        validate_material_required("article title", &self.title)?;
        validate_material_identifier("article thumbnail media id", &self.thumb_media_id)?;
        validate_material_required("article content", &self.content)?;
        if !matches!(self.show_cover_pic, 0 | 1) {
            return Err(WechatError::Config(
                "official account material show-cover flag must be 0 or 1".to_string(),
            ));
        }
        for (kind, value) in [
            ("open-comment flag", self.need_open_comment),
            ("fans-only-comment flag", self.only_fans_can_comment),
        ] {
            if value.is_some_and(|value| !matches!(value, 0 | 1)) {
                return Err(WechatError::Config(format!(
                    "official account material {kind} must be 0 or 1"
                )));
            }
        }
        if self.only_fans_can_comment == Some(1) && self.need_open_comment != Some(1) {
            return Err(WechatError::Config(
                "official account material fans-only comments require comments to be enabled"
                    .to_string(),
            ));
        }
        if !self.content_source_url.trim().is_empty() {
            validate_material_http_url("article source URL", &self.content_source_url)?;
        }
        Ok(())
    }
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

impl PublishArticle {
    pub fn validate(&self) -> Result<()> {
        validate_publish_required("article title", &self.title)?;
        validate_publish_identifier("article thumbnail media id", &self.thumb_media_id)?;
        validate_publish_required("article content", &self.content)?;
        for (kind, value) in [
            ("open-comment flag", self.need_open_comment),
            ("fans-only-comment flag", self.only_fans_can_comment),
        ] {
            if value.is_some_and(|value| !matches!(value, 0 | 1)) {
                return Err(WechatError::Config(format!(
                    "official account publish {kind} must be 0 or 1"
                )));
            }
        }
        if self.only_fans_can_comment == Some(1) && self.need_open_comment != Some(1) {
            return Err(WechatError::Config(
                "official account publish fans-only comments require comments to be enabled"
                    .to_string(),
            ));
        }
        if !self.content_source_url.trim().is_empty() {
            validate_material_http_url("publish article source URL", &self.content_source_url)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftAddRequest {
    pub articles: Vec<PublishArticle>,
}

impl PublishDraftAddRequest {
    pub fn new(articles: Vec<PublishArticle>) -> Self {
        Self { articles }
    }

    pub fn validate(&self) -> Result<()> {
        validate_publish_articles(&self.articles)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftUpdateRequest {
    pub media_id: String,
    pub index: i64,
    pub articles: PublishArticle,
}

impl PublishDraftUpdateRequest {
    pub fn validate(&self) -> Result<()> {
        validate_publish_identifier("draft media id", &self.media_id)?;
        validate_publish_index(self.index)?;
        self.articles.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchGetRequest {
    pub offset: i64,
    pub count: i64,
    pub no_content: i64,
}

impl PublishBatchGetRequest {
    pub fn new(offset: i64, count: i64, no_content: bool) -> Self {
        Self {
            offset,
            count,
            no_content: i64::from(no_content),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.offset < 0 {
            return Err(WechatError::Config(
                "official account publish list offset must not be negative".to_string(),
            ));
        }
        if !(1..=20).contains(&self.count) {
            return Err(WechatError::Config(
                "official account publish list count must be between 1 and 20".to_string(),
            ));
        }
        if !matches!(self.no_content, 0 | 1) {
            return Err(WechatError::Config(
                "official account publish no-content flag must be 0 or 1".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_publish_articles(articles: &[PublishArticle]) -> Result<()> {
    if articles.is_empty() || articles.len() > 8 {
        return Err(WechatError::Config(
            "official account publish articles must contain between 1 and 8 entries".to_string(),
        ));
    }
    for article in articles {
        article.validate()?;
    }
    Ok(())
}

fn validate_publish_index(index: i64) -> Result<()> {
    if !(0..=7).contains(&index) {
        return Err(WechatError::Config(
            "official account publish article index must be between 0 and 7".to_string(),
        ));
    }
    Ok(())
}

fn validate_publish_delete_index(index: i64) -> Result<()> {
    if !(0..=8).contains(&index) {
        return Err(WechatError::Config(
            "official account published article delete index must be between 0 and 8".to_string(),
        ));
    }
    Ok(())
}

fn validate_publish_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "official account {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_publish_identifier(kind: &str, value: &str) -> Result<()> {
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "official account {kind} must contain 1 to 512 printable UTF-8 bytes"
        )));
    }
    Ok(())
}

fn ensure_publish_response_success(
    operation: &str,
    errcode: Option<i64>,
    errmsg: Option<&str>,
) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg.unwrap_or(operation).to_string(),
        });
    }
    Ok(())
}

fn validate_publish_timestamps(
    kind: &str,
    create_time: Option<i64>,
    update_time: Option<i64>,
) -> Result<()> {
    if create_time.is_some_and(|timestamp| timestamp < 0)
        || update_time.is_some_and(|timestamp| timestamp < 0)
    {
        return Err(WechatError::Config(format!(
            "official account {kind} timestamps cannot be negative"
        )));
    }
    if create_time
        .zip(update_time)
        .is_some_and(|(created, updated)| updated < created)
    {
        return Err(WechatError::Config(format!(
            "official account {kind} update time cannot precede create time"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishDraftAddResponse {
    pub fn require_media_id(&self) -> Result<&str> {
        ensure_publish_response_success(
            "official account add draft",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let media_id = self.media_id.as_deref().ok_or_else(|| {
            WechatError::Config("official account draft add response requires media_id".to_string())
        })?;
        validate_publish_identifier("draft media id", media_id)?;
        Ok(media_id)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_media_id().map(|_| ())
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishNewsItem {
    fn validate_with_content(&self, require_content: bool) -> Result<()> {
        let deleted = self.is_deleted == Some(true);
        if !deleted {
            validate_publish_required(
                "publish response article title",
                self.title.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "official account publish response article requires title".to_string(),
                    )
                })?,
            )?;
            if require_content {
                validate_publish_required(
                    "publish response article content",
                    self.content.as_deref().ok_or_else(|| {
                        WechatError::Config(
                            "official account publish response article requires content"
                                .to_string(),
                        )
                    })?,
                )?;
            }
        }
        for (kind, value) in [
            ("show-cover flag", self.show_cover_pic),
            ("open-comment flag", self.need_open_comment),
            ("fans-only-comment flag", self.only_fans_can_comment),
        ] {
            if value.is_some_and(|value| !matches!(value, 0 | 1)) {
                return Err(WechatError::Config(format!(
                    "official account publish response {kind} must be 0 or 1"
                )));
            }
        }
        if self.only_fans_can_comment == Some(1) && self.need_open_comment != Some(1) {
            return Err(WechatError::Config(
                "official account publish response fans-only comments require comments to be enabled"
                    .to_string(),
            ));
        }
        if let Some(thumb_media_id) = self
            .thumb_media_id
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            validate_publish_identifier(
                "publish response article thumbnail media id",
                thumb_media_id,
            )?;
        }
        for (kind, value) in [
            ("article source URL", self.content_source_url.as_deref()),
            ("article thumbnail URL", self.thumb_url.as_deref()),
            ("article URL", self.url.as_deref()),
        ] {
            if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
                validate_material_http_url(kind, value)?;
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_with_content(true)
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishDraftGetResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_publish_response_success(
            "official account get draft",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.news_item.is_empty() || self.news_item.len() > 8 {
            return Err(WechatError::Config(
                "official account draft response must contain 1 to 8 articles".to_string(),
            ));
        }
        for item in &self.news_item {
            item.validate()?;
        }
        validate_publish_timestamps("draft response", self.create_time, self.update_time)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDraftCountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishDraftCountResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_publish_response_success(
            "official account count drafts",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.total_count.is_none_or(|count| count < 0) {
            return Err(WechatError::Config(
                "official account draft count response requires a non-negative total_count"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishContent {
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishContent {
    fn validate(&self, no_content: bool) -> Result<()> {
        if !no_content && (self.news_item.is_empty() || self.news_item.len() > 8) {
            return Err(WechatError::Config(
                "official account publish content must contain 1 to 8 articles".to_string(),
            ));
        }
        if self.news_item.len() > 8 {
            return Err(WechatError::Config(
                "official account publish content cannot exceed 8 articles".to_string(),
            ));
        }
        for item in &self.news_item {
            item.validate_with_content(!no_content)?;
        }
        validate_publish_timestamps("publish content", self.create_time, self.update_time)
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishBatchKind {
    Draft,
    Published,
}

impl PublishBatchItem {
    fn validate(&self, kind: PublishBatchKind, no_content: bool) -> Result<()> {
        match kind {
            PublishBatchKind::Draft => validate_publish_identifier(
                "draft list media id",
                self.media_id.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "official account draft list item requires media_id".to_string(),
                    )
                })?,
            )?,
            PublishBatchKind::Published => validate_publish_identifier(
                "published list article id",
                self.article_id.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "official account published list item requires article_id".to_string(),
                    )
                })?,
            )?,
        }
        if let Some(media_id) = self.media_id.as_deref() {
            validate_publish_identifier("publish list media id", media_id)?;
        }
        if let Some(article_id) = self.article_id.as_deref() {
            validate_publish_identifier("publish list article id", article_id)?;
        }
        if self.update_time.is_some_and(|timestamp| timestamp < 0) {
            return Err(WechatError::Config(
                "official account publish list update_time cannot be negative".to_string(),
            ));
        }
        match self.content.as_ref() {
            Some(content) => content.validate(no_content),
            None if no_content => Ok(()),
            None => Err(WechatError::Config(
                "official account publish list item requires content".to_string(),
            )),
        }
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishBatchGetResponse {
    pub fn validate_for(&self, kind: PublishBatchKind, no_content: bool) -> Result<()> {
        ensure_publish_response_success(
            "official account list publish records",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let total_count = self.total_count.ok_or_else(|| {
            WechatError::Config(
                "official account publish list response requires total_count".to_string(),
            )
        })?;
        let item_count = self.item_count.ok_or_else(|| {
            WechatError::Config(
                "official account publish list response requires item_count".to_string(),
            )
        })?;
        if total_count < 0 || item_count < 0 {
            return Err(WechatError::Config(
                "official account publish list counts cannot be negative".to_string(),
            ));
        }
        if item_count > total_count || item_count > 20 {
            return Err(WechatError::Config(
                "official account publish list item_count cannot exceed total_count or 20"
                    .to_string(),
            ));
        }
        let actual_count = i64::try_from(self.item.len()).map_err(|_| {
            WechatError::Config("official account publish list is too large".to_string())
        })?;
        if item_count != actual_count {
            return Err(WechatError::Config(format!(
                "official account publish list item_count {item_count} does not match {} decoded items",
                self.item.len()
            )));
        }
        let mut identities = HashSet::with_capacity(self.item.len());
        for item in &self.item {
            item.validate(kind, no_content)?;
            let identity = match kind {
                PublishBatchKind::Draft => item.media_id.as_deref(),
                PublishBatchKind::Published => item.article_id.as_deref(),
            }
            .expect("validated publish list identity")
            .trim();
            if !identities.insert(identity) {
                return Err(WechatError::Config(
                    "official account publish list contains duplicate item identities".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn next_offset(
        &self,
        current_offset: i64,
        kind: PublishBatchKind,
        no_content: bool,
    ) -> Result<Option<i64>> {
        self.validate_for(kind, no_content)?;
        if current_offset < 0 {
            return Err(WechatError::Config(
                "official account publish list current offset cannot be negative".to_string(),
            ));
        }
        let page_count = self.item_count.expect("validated publish list item_count");
        let total_count = self
            .total_count
            .expect("validated publish list total_count");
        let next_offset = current_offset.checked_add(page_count).ok_or_else(|| {
            WechatError::Config("official account publish list next offset overflowed".to_string())
        })?;
        if next_offset < total_count && page_count == 0 {
            return Err(WechatError::Config(
                "official account publish list cannot advance an empty non-terminal page"
                    .to_string(),
            ));
        }
        Ok((next_offset < total_count).then_some(next_offset))
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishDraftSwitchStatusResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_publish_response_success(
            "official account check draft switch",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.is_open.is_none_or(|value| !matches!(value, 0 | 1)) {
            return Err(WechatError::Config(
                "official account draft switch response requires is_open 0 or 1".to_string(),
            ));
        }
        if self.total_count.is_some_and(|count| count < 0)
            || self.item_count.is_some_and(|count| count < 0)
            || self
                .total_count
                .zip(self.item_count)
                .is_some_and(|(total, page)| page > total)
        {
            return Err(WechatError::Config(
                "official account draft switch counts must be non-negative and consistent"
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.is_open == Some(1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishSubmitResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub publish_id: Option<u64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishSubmitResponse {
    pub fn require_publish_id(&self) -> Result<u64> {
        ensure_publish_response_success(
            "official account submit publish",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        self.publish_id
            .filter(|publish_id| *publish_id != 0)
            .ok_or_else(|| {
                WechatError::Config(
                    "official account publish submit response requires a positive publish_id"
                        .to_string(),
                )
            })
    }

    pub fn validate(&self) -> Result<()> {
        self.require_publish_id().map(|_| ())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleItem {
    #[serde(default)]
    pub idx: Option<i64>,
    #[serde(default)]
    pub article_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishArticleItem {
    fn validate(&self) -> Result<()> {
        if self.idx.is_none_or(|index| !(1..=8).contains(&index)) {
            return Err(WechatError::Config(
                "official account publish article detail index must be between 1 and 8".to_string(),
            ));
        }
        let article_url = self.article_url.as_deref().ok_or_else(|| {
            WechatError::Config(
                "official account publish article detail requires article_url".to_string(),
            )
        })?;
        validate_material_http_url("publish article detail URL", article_url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleDetail {
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub item: Vec<PublishArticleItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishArticleDetail {
    fn validate(&self) -> Result<()> {
        let count = self.count.ok_or_else(|| {
            WechatError::Config(
                "official account publish article detail requires count".to_string(),
            )
        })?;
        let actual_count = i64::try_from(self.item.len()).map_err(|_| {
            WechatError::Config("official account publish article detail is too large".to_string())
        })?;
        if !(0..=8).contains(&count) || count != actual_count {
            return Err(WechatError::Config(
                "official account publish article detail count must match 0 to 8 items".to_string(),
            ));
        }
        let mut indices = HashSet::with_capacity(self.item.len());
        for item in &self.item {
            item.validate()?;
            if !indices.insert(item.idx.expect("validated publish article index")) {
                return Err(WechatError::Config(
                    "official account publish article detail indices must be unique".to_string(),
                ));
            }
        }
        Ok(())
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishStatusKind {
    Success,
    Publishing,
    OriginalFailed,
    Failed,
    AuditRefused,
    UserDeleted,
    SystemBanned,
    Other,
}

impl PublishStatusResponse {
    pub fn require_publish_id(&self) -> Result<u64> {
        ensure_publish_response_success(
            "official account get publish status",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        self.publish_id
            .filter(|publish_id| *publish_id != 0)
            .ok_or_else(|| {
                WechatError::Config(
                    "official account publish status response requires a positive publish_id"
                        .to_string(),
                )
            })
    }

    pub fn checked_article_ids(&self) -> Result<Vec<&str>> {
        let values = match &self.article_id {
            None | Some(Value::Null) => Vec::new(),
            Some(Value::String(article_id)) => vec![article_id.as_str()],
            Some(Value::Array(article_ids)) => article_ids
                .iter()
                .map(|article_id| {
                    article_id.as_str().ok_or_else(|| {
                        WechatError::Config(
                            "official account publish article_id array must contain strings"
                                .to_string(),
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()?,
            Some(_) => {
                return Err(WechatError::Config(
                    "official account publish article_id must be a string or string array"
                        .to_string(),
                ));
            }
        };
        if values.len() > 8 {
            return Err(WechatError::Config(
                "official account publish status cannot contain more than 8 article ids"
                    .to_string(),
            ));
        }
        let mut unique = HashSet::with_capacity(values.len());
        for article_id in &values {
            validate_publish_identifier("published article id", article_id)?;
            if !unique.insert(article_id.trim()) {
                return Err(WechatError::Config(
                    "official account publish article ids must be unique".to_string(),
                ));
            }
        }
        Ok(values)
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_inner(None)
    }

    pub fn validate_for(&self, expected_publish_id: u64) -> Result<()> {
        if expected_publish_id == 0 {
            return Err(WechatError::Config(
                "official account expected publish id must be positive".to_string(),
            ));
        }
        self.validate_inner(Some(expected_publish_id))
    }

    fn validate_inner(&self, expected_publish_id: Option<u64>) -> Result<()> {
        let publish_id = self.require_publish_id()?;
        if expected_publish_id.is_some_and(|expected| publish_id != expected) {
            return Err(WechatError::Config(format!(
                "official account publish status id {publish_id} does not match requested id {}",
                expected_publish_id.expect("checked expected publish id")
            )));
        }
        let status = self.publish_status.ok_or_else(|| {
            WechatError::Config(
                "official account publish status response requires publish_status".to_string(),
            )
        })?;
        if status < 0 {
            return Err(WechatError::Config(
                "official account publish status cannot be negative".to_string(),
            ));
        }
        let article_ids = self.checked_article_ids()?;
        if self.status_kind() == Some(PublishStatusKind::Success) && article_ids.is_empty() {
            return Err(WechatError::Config(
                "successful official account publishing requires at least one article id"
                    .to_string(),
            ));
        }
        if let Some(detail) = &self.article_detail {
            detail.validate()?;
        }
        let mut failed_indices = HashSet::with_capacity(self.fail_idx.len());
        for index in &self.fail_idx {
            if !(1..=8).contains(index) || !failed_indices.insert(*index) {
                return Err(WechatError::Config(
                    "official account publish failed indices must be unique values from 1 to 8"
                        .to_string(),
                ));
            }
        }
        if matches!(
            self.status_kind(),
            Some(PublishStatusKind::Success | PublishStatusKind::Publishing)
        ) && !self.fail_idx.is_empty()
        {
            return Err(WechatError::Config(
                "successful or pending official account publishing cannot contain failed indices"
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn status_kind(&self) -> Option<PublishStatusKind> {
        self.publish_status.map(|status| match status {
            0 => PublishStatusKind::Success,
            1 => PublishStatusKind::Publishing,
            2 => PublishStatusKind::OriginalFailed,
            3 => PublishStatusKind::Failed,
            4 => PublishStatusKind::AuditRefused,
            5 => PublishStatusKind::UserDeleted,
            6 => PublishStatusKind::SystemBanned,
            _ => PublishStatusKind::Other,
        })
    }

    pub fn is_success(&self) -> bool {
        self.status_kind() == Some(PublishStatusKind::Success)
    }

    pub fn is_pending(&self) -> bool {
        self.status_kind() == Some(PublishStatusKind::Publishing)
    }

    pub fn is_failed(&self) -> bool {
        matches!(
            self.status_kind(),
            Some(
                PublishStatusKind::OriginalFailed
                    | PublishStatusKind::Failed
                    | PublishStatusKind::AuditRefused
                    | PublishStatusKind::UserDeleted
                    | PublishStatusKind::SystemBanned
            )
        )
    }

    pub fn is_terminal(&self) -> bool {
        self.status_kind()
            .is_some_and(PublishStatusKind::is_terminal)
    }

    pub fn article_ids(&self) -> Vec<&str> {
        self.checked_article_ids().unwrap_or_default()
    }
}

impl PublishStatusKind {
    pub fn is_terminal(self) -> bool {
        !matches!(self, Self::Publishing | Self::Other)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishArticleResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl PublishArticleResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_publish_response_success(
            "official account get published article",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.news_item.is_empty() || self.news_item.len() > 8 {
            return Err(WechatError::Config(
                "official account published article response must contain 1 to 8 articles"
                    .to_string(),
            ));
        }
        for item in &self.news_item {
            item.validate()?;
        }
        Ok(())
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MaterialMediaResponse {
    pub fn require_media_id(&self) -> Result<&str> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        let media_id = self.media_id.as_deref().ok_or_else(|| {
            WechatError::Config("material upload response is missing media id".to_string())
        })?;
        validate_material_identifier("response media id", media_id)?;
        Ok(media_id)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        if let Some(media_id) = self.media_id.as_deref() {
            validate_material_identifier("response media id", media_id)?;
        }
        if let Some(url) = self.url.as_deref() {
            validate_material_http_url("response URL", url)?;
        }
        Ok(())
    }

    pub fn require_url(&self) -> Result<&str> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        let url = self.url.as_deref().ok_or_else(|| {
            WechatError::Config("material upload response is missing URL".to_string())
        })?;
        validate_material_http_url("response URL", url)?;
        Ok(url)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialUploadKind {
    Image,
    Voice,
    Thumb,
}

impl MaterialUploadKind {
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Voice => "voice",
            Self::Thumb => "thumb",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialListKind {
    Image,
    Voice,
    Video,
    News,
}

impl MaterialListKind {
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Voice => "voice",
            Self::Video => "video",
            Self::News => "news",
        }
    }

    pub fn from_code(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("image") {
            Some(Self::Image)
        } else if value.eq_ignore_ascii_case("voice") {
            Some(Self::Voice)
        } else if value.eq_ignore_ascii_case("video") {
            Some(Self::Video)
        } else if value.eq_ignore_ascii_case("news") {
            Some(Self::News)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub down_url: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MaterialGetResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.news_item.is_empty()
            && self.title.is_none()
            && self.description.is_none()
            && self.down_url.is_none()
            && self.url.is_none()
        {
            return Err(WechatError::Config(
                "material response does not contain news, video, or media data".to_string(),
            ));
        }
        if let Some(title) = self.title.as_deref() {
            validate_material_required("video title", title)?;
        }
        if let Some(description) = self.description.as_deref() {
            validate_material_required("video description", description)?;
        }
        if let Some(url) = self.down_url.as_deref() {
            validate_material_http_url("video download URL", url)?;
        }
        if let Some(url) = self.url.as_deref() {
            validate_material_http_url("media URL", url)?;
        }
        if self.news_item.len() > 8 {
            return Err(WechatError::Config(
                "material response cannot contain more than 8 news items".to_string(),
            ));
        }
        for item in &self.news_item {
            item.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialListRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub offset: i64,
    pub count: i64,
}

impl MaterialListRequest {
    pub fn new(kind: MaterialListKind, offset: i64, count: i64) -> Self {
        Self {
            kind: kind.as_code().to_string(),
            offset,
            count,
        }
    }

    pub fn kind(&self) -> Option<MaterialListKind> {
        MaterialListKind::from_code(&self.kind)
    }

    pub fn validate(&self) -> Result<()> {
        if self.kind().is_none() {
            return Err(WechatError::Config(
                "official account material list type must be image, voice, video, or news"
                    .to_string(),
            ));
        }
        if self.offset < 0 {
            return Err(WechatError::Config(
                "official account material list offset must not be negative".to_string(),
            ));
        }
        if !(1..=20).contains(&self.count) {
            return Err(WechatError::Config(
                "official account material list count must be between 1 and 20".to_string(),
            ));
        }
        Ok(())
    }
}

fn ensure_material_response_success(errcode: Option<i64>, errmsg: Option<&str>) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg.unwrap_or("material operation failed").to_string(),
        });
    }
    Ok(())
}

pub(crate) fn validate_material_articles(articles: &[Article]) -> Result<()> {
    if articles.is_empty() || articles.len() > 8 {
        return Err(WechatError::Config(
            "official account material articles must contain between 1 and 8 entries".to_string(),
        ));
    }
    for article in articles {
        article.validate()?;
    }
    Ok(())
}

pub(crate) fn validate_material_upload(file_name: &str, data: &[u8]) -> Result<()> {
    validate_material_required("file name", file_name)?;
    if file_name.len() > 255
        || file_name.chars().any(char::is_control)
        || file_name.contains(['/', '\\'])
    {
        return Err(WechatError::Config(
            "official account material file name must contain at most 255 printable UTF-8 bytes without path separators"
                .to_string(),
        ));
    }
    if data.is_empty() {
        return Err(WechatError::Config(
            "official account material upload data must not be empty".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_material_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "official account material {kind} must not be blank"
        )));
    }
    Ok(())
}

pub(crate) fn validate_material_identifier(kind: &str, value: &str) -> Result<()> {
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "official account material {kind} must contain 1 to 512 printable UTF-8 bytes"
        )));
    }
    Ok(())
}

fn validate_material_http_url(kind: &str, value: &str) -> Result<()> {
    let url = url::Url::parse(value).map_err(|error| {
        WechatError::Config(format!(
            "official account material {kind} is invalid: {error}"
        ))
    })?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(WechatError::Config(format!(
            "official account material {kind} must be an absolute HTTP(S) URL without credentials or fragments"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialListItem {
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MaterialListItem {
    pub fn news_content(&self) -> Result<Option<MaterialNewsContent>> {
        self.content
            .clone()
            .map(serde_json::from_value)
            .transpose()
            .map_err(WechatError::from)
    }

    pub fn require_media_id(&self) -> Result<&str> {
        let media_id = self.media_id.as_deref().ok_or_else(|| {
            WechatError::Config("material list item is missing media id".to_string())
        })?;
        validate_material_identifier("list item media id", media_id)?;
        Ok(media_id)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_media_id()?;
        if self.update_time.is_some_and(|update_time| update_time < 0) {
            return Err(WechatError::Config(
                "material list item update time cannot be negative".to_string(),
            ));
        }
        if let Some(url) = self.url.as_deref() {
            validate_material_http_url("list item URL", url)?;
        }
        if let Some(content) = self.news_content()? {
            content.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialNewsContent {
    #[serde(default)]
    pub news_item: Vec<PublishNewsItem>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MaterialNewsContent {
    pub fn validate(&self) -> Result<()> {
        if self.news_item.is_empty() || self.news_item.len() > 8 {
            return Err(WechatError::Config(
                "material news content must contain 1 to 8 articles".to_string(),
            ));
        }
        if self.create_time.is_some_and(|create_time| create_time < 0)
            || self.update_time.is_some_and(|update_time| update_time < 0)
        {
            return Err(WechatError::Config(
                "material news timestamps cannot be negative".to_string(),
            ));
        }
        if self
            .create_time
            .zip(self.update_time)
            .is_some_and(|(created, updated)| updated < created)
        {
            return Err(WechatError::Config(
                "material news update time cannot precede create time".to_string(),
            ));
        }
        for item in &self.news_item {
            item.validate_with_content(false)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub item_count: Option<i64>,
    #[serde(default)]
    pub item: Vec<MaterialListItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MaterialListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.total_count.is_some_and(|count| count < 0)
            || self.item_count.is_some_and(|count| count < 0)
        {
            return Err(WechatError::Config(
                "material list counts cannot be negative".to_string(),
            ));
        }
        if let Some(item_count) = self.item_count {
            let actual_count = i64::try_from(self.item.len()).map_err(|_| {
                WechatError::Config("material list item count exceeds i64".to_string())
            })?;
            if item_count != actual_count {
                return Err(WechatError::Config(format!(
                    "material list item_count {item_count} does not match {} decoded items",
                    self.item.len()
                )));
            }
        }
        if self
            .total_count
            .zip(self.item_count)
            .is_some_and(|(total, page)| page > total)
        {
            return Err(WechatError::Config(
                "material list item_count cannot exceed total_count".to_string(),
            ));
        }
        let mut media_ids = HashSet::with_capacity(self.item.len());
        for item in &self.item {
            item.validate()?;
            let media_id = item.require_media_id()?;
            if !media_ids.insert(media_id.trim()) {
                return Err(WechatError::Config(
                    "material list contains duplicate media ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find(&self, media_id: &str) -> Option<&MaterialListItem> {
        self.item
            .iter()
            .find(|item| item.media_id.as_deref() == Some(media_id))
    }

    pub fn next_offset(&self, current_offset: i64) -> Result<Option<i64>> {
        self.validate()?;
        if current_offset < 0 {
            return Err(WechatError::Config(
                "material list current offset cannot be negative".to_string(),
            ));
        }
        let page_count = match self.item_count {
            Some(item_count) => item_count,
            None => i64::try_from(self.item.len()).map_err(|_| {
                WechatError::Config("material list item count exceeds i64".to_string())
            })?,
        };
        let next_offset = current_offset.checked_add(page_count).ok_or_else(|| {
            WechatError::Config("material list next offset overflowed".to_string())
        })?;
        match self.total_count {
            Some(total_count) if next_offset < total_count && page_count == 0 => {
                Err(WechatError::Config(
                    "material list cannot advance an empty non-terminal page".to_string(),
                ))
            }
            Some(total_count) if next_offset < total_count => Ok(Some(next_offset)),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialStatsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub voice_count: Option<i64>,
    #[serde(default)]
    pub video_count: Option<i64>,
    #[serde(default)]
    pub image_count: Option<i64>,
    #[serde(default)]
    pub news_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

impl MaterialStatsResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_material_response_success(self.errcode, self.errmsg.as_deref())?;
        if [
            self.voice_count,
            self.video_count,
            self.image_count,
            self.news_count,
        ]
        .into_iter()
        .flatten()
        .any(|count| count < 0)
        {
            return Err(WechatError::Config(
                "material statistics counts cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    pub fn total_count(&self) -> Result<i64> {
        self.validate()?;
        [
            self.voice_count,
            self.video_count,
            self.image_count,
            self.news_count,
        ]
        .into_iter()
        .flatten()
        .try_fold(0_i64, |total, count| {
            total.checked_add(count).ok_or_else(|| {
                WechatError::Config("material statistics total count overflowed".to_string())
            })
        })
    }
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

impl TemplateMessageRequest {
    pub fn validate(&self) -> Result<()> {
        validate_official_identifier("template recipient openid", &self.touser)?;
        validate_official_identifier("template id", &self.template_id)?;
        if let Some(url) = &self.url {
            validate_material_http_url("template message URL", url)?;
        }
        if let Some(miniprogram) = &self.miniprogram {
            miniprogram.validate()?;
        }
        if let Some(client_msg_id) = &self.client_msg_id {
            validate_official_identifier("template client message id", client_msg_id)?;
        }
        validate_template_data(&self.data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMiniProgram {
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

impl TemplateMiniProgram {
    pub fn validate(&self) -> Result<()> {
        validate_official_identifier("template mini-program appid", &self.appid)?;
        if let Some(pagepath) = &self.pagepath {
            validate_official_identifier("template mini-program page path", pagepath)?;
        }
        Ok(())
    }
}

fn validate_template_data(data: &Value) -> Result<()> {
    let fields = data
        .as_object()
        .filter(|fields| !fields.is_empty())
        .ok_or_else(|| {
            WechatError::Config(
                "official account template message data must be a non-empty object".to_string(),
            )
        })?;
    for (name, field) in fields {
        validate_official_identifier("template data field name", name)?;
        let field = field.as_object().ok_or_else(|| {
            WechatError::Config(format!(
                "official account template data field {name:?} must be an object"
            ))
        })?;
        let value = field
            .get("value")
            .and_then(Value::as_str)
            .unwrap_or_default();
        validate_official_required(&format!("template data field {name:?} value"), value)?;
        if let Some(color) = field.get("color") {
            let color = color.as_str().ok_or_else(|| {
                WechatError::Config(format!(
                    "official account template data field {name:?} color must be a string"
                ))
            })?;
            if !(color.is_empty()
                || color.len() == 7
                    && color.starts_with('#')
                    && color[1..]
                        .chars()
                        .all(|character| character.is_ascii_hexdigit()))
            {
                return Err(WechatError::Config(format!(
                    "official account template data field {name:?} color must use #RRGGBB"
                )));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMessageSendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msgid: Option<i64>,
}

impl TemplateMessageSendResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account template message send",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.msgid.is_none_or(|msgid| msgid <= 0) {
            return Err(WechatError::Config(
                "official account template send response requires a positive msgid".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateIndustryInfo {
    #[serde(default)]
    pub first_class: Option<String>,
    #[serde(default)]
    pub second_class: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateIndustryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub primary_industry: Option<TemplateIndustryInfo>,
    #[serde(default)]
    pub secondary_industry: Option<TemplateIndustryInfo>,
}

impl TemplateIndustryResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account template industry get",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        for (kind, industry) in [
            ("primary", self.primary_industry.as_ref()),
            ("secondary", self.secondary_industry.as_ref()),
        ] {
            let industry = industry.ok_or_else(|| {
                WechatError::Config(format!(
                    "official account template industry response requires {kind}_industry"
                ))
            })?;
            validate_official_required(
                &format!("{kind} template industry first class"),
                industry.first_class.as_deref().unwrap_or_default(),
            )?;
            validate_official_required(
                &format!("{kind} template industry second class"),
                industry.second_class.as_deref().unwrap_or_default(),
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
}

impl TemplateAddResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account template add",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_official_identifier(
            "template add response id",
            self.template_id.as_deref().unwrap_or_default(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateTemplateInfo {
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub primary_industry: Option<String>,
    #[serde(default)]
    pub deputy_industry: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateTemplateListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_list: Vec<PrivateTemplateInfo>,
}

impl PrivateTemplateListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_official_response_success(
            "official account private template list",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let mut template_ids = HashSet::with_capacity(self.template_list.len());
        for template in &self.template_list {
            let template_id = template.template_id.as_deref().unwrap_or_default();
            validate_official_identifier("private template response id", template_id)?;
            if !template_ids.insert(template_id) {
                return Err(WechatError::Config(
                    "official account private template response contains duplicate template ids"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

fn validate_template_keywords(keywords: &[String]) -> Result<()> {
    if keywords.len() > 5 {
        return Err(WechatError::Config(
            "official account template keyword list supports at most 5 names".to_string(),
        ));
    }
    let mut unique = HashSet::with_capacity(keywords.len());
    for keyword in keywords {
        validate_official_identifier("template keyword name", keyword)?;
        if !unique.insert(keyword.trim()) {
            return Err(WechatError::Config(
                "official account template keyword list contains duplicate names".to_string(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUserInfoRequest {
    pub user_list: Vec<UserInfoQuery>,
}

impl BatchGetUserInfoRequest {
    pub fn validate(&self) -> Result<()> {
        if self.user_list.is_empty() || self.user_list.len() > 100 {
            return Err(WechatError::Config(
                "official account batch user query must contain between 1 and 100 users"
                    .to_string(),
            ));
        }
        let mut openids = HashSet::with_capacity(self.user_list.len());
        for user in &self.user_list {
            validate_official_required("batch user openid", &user.openid)?;
            validate_official_user_lang(&user.lang)?;
            if !openids.insert(user.openid.trim()) {
                return Err(WechatError::Config(
                    "official account batch user query contains duplicate openids".to_string(),
                ));
            }
        }
        Ok(())
    }
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

fn validate_official_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "official account {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_official_identifier(kind: &str, value: &str) -> Result<()> {
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "official account {kind} must contain 1 to 512 printable UTF-8 bytes"
        )));
    }
    Ok(())
}

fn validate_optional_official_identifier(kind: &str, value: &str) -> Result<()> {
    if !value.is_empty() && value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "official account {kind} must not contain only whitespace"
        )));
    }
    Ok(())
}

fn validate_official_positive_identifier(kind: &str, value: &str) -> Result<()> {
    validate_official_required(kind, value)?;
    if !value.parse::<i64>().is_ok_and(|id| id > 0) {
        return Err(WechatError::Config(format!(
            "official account {kind} must be a positive integer"
        )));
    }
    Ok(())
}

fn validate_official_user_lang(lang: &str) -> Result<()> {
    if !matches!(lang, "zh_CN" | "zh_TW" | "en") {
        return Err(WechatError::Config(
            "official account user language must be zh_CN, zh_TW, or en".to_string(),
        ));
    }
    Ok(())
}

fn validate_official_identifier_batch(
    kind: &str,
    identifiers: &[String],
    maximum: usize,
) -> Result<()> {
    if identifiers.is_empty() || identifiers.len() > maximum {
        return Err(WechatError::Config(format!(
            "official account {kind} batch must contain between 1 and {maximum} openids"
        )));
    }
    let mut unique = HashSet::with_capacity(identifiers.len());
    for identifier in identifiers {
        validate_official_required(&format!("{kind} openid"), identifier)?;
        if !unique.insert(identifier.trim()) {
            return Err(WechatError::Config(format!(
                "official account {kind} batch contains duplicate openids"
            )));
        }
    }
    Ok(())
}

fn validate_official_tag_name(name: &str) -> Result<()> {
    validate_official_required("tag name", name)?;
    if name.chars().count() > 30 {
        return Err(WechatError::Config(
            "official account tag name must not exceed 30 characters".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{config::Platform, Client, WechatConfig};

    use super::*;

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
            article_id: None,
            sub_button: Vec::new(),
        })
        .unwrap();

        assert_eq!(value["type"], "view");
        assert_eq!(value["url"], "https://example.com");
    }

    #[test]
    fn validates_menu_button_and_match_rule_workflows() {
        let menu = CreateMenuRequest::new(vec![
            MenuButton::leaf("Website", MenuButtonKind::View, "https://example.com")
                .expect("view button"),
            MenuButton::parent(
                "Services",
                vec![
                    MenuButton::leaf("Scan", MenuButtonKind::ScanCodePush, "scan")
                        .expect("scan button"),
                    MenuButton::mini_program(
                        "Mini Program",
                        "https://example.com/fallback",
                        "wx-mini-program",
                        "pages/index/index",
                    ),
                ],
            ),
        ]);
        assert!(menu.validate().is_ok());

        assert!(MenuButton::leaf("Mini", MenuButtonKind::MiniProgram, "ignored").is_err());
        let mut conflicting =
            MenuButton::leaf("Website", MenuButtonKind::View, "https://example.com")
                .expect("view button");
        conflicting.key = Some("unexpected".to_string());
        assert!(conflicting.validate().is_err());
        assert!(CreateMenuRequest::new(Vec::new()).validate().is_err());

        let conditional = CreateConditionalMenuRequest {
            button: vec![
                MenuButton::leaf("VIP", MenuButtonKind::Click, "vip").expect("click button")
            ],
            matchrule: MatchRule {
                tag_id: Some("2".to_string()),
                ..MatchRule::default()
            },
        };
        assert!(conditional.validate().is_ok());
        assert!(MatchRule::default().validate().is_err());
        assert!(MatchRule {
            sex: Some("3".to_string()),
            ..MatchRule::default()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn serializes_batch_user_query() {
        let request = BatchGetUserInfoRequest {
            user_list: vec![UserInfoQuery {
                openid: "openid".to_string(),
                lang: "zh_CN".to_string(),
            }],
        };
        assert!(request.validate().is_ok());
        let value = serde_json::to_value(request).unwrap();

        assert_eq!(
            value,
            json!({ "user_list": [{ "openid": "openid", "lang": "zh_CN" }] })
        );
    }

    #[test]
    fn validates_official_user_and_tag_batches() {
        let duplicate = BatchGetUserInfoRequest {
            user_list: vec![
                UserInfoQuery {
                    openid: "openid".to_string(),
                    lang: "zh_CN".to_string(),
                },
                UserInfoQuery {
                    openid: "openid".to_string(),
                    lang: "en".to_string(),
                },
            ],
        };
        assert!(duplicate.validate().is_err());

        let invalid_lang = BatchGetUserInfoRequest {
            user_list: vec![UserInfoQuery {
                openid: "openid".to_string(),
                lang: "fr".to_string(),
            }],
        };
        assert!(invalid_lang.validate().is_err());
        assert!(validate_official_identifier_batch(
            "tagging",
            &["one".to_string(), "two".to_string()],
            50
        )
        .is_ok());
        assert!(validate_official_identifier_batch(
            "blacklist",
            &["same".to_string(), "same".to_string()],
            20
        )
        .is_err());
        assert!(validate_official_tag_name(&"x".repeat(31)).is_err());
        assert!(validate_official_positive_identifier("tag id", "0").is_err());
    }

    #[test]
    fn validates_customer_service_messages() {
        let message = CustomerServiceMessage::text("openid", "hello");
        assert!(message.validate().is_ok());
        let value = serde_json::to_value(message).unwrap();
        assert_eq!(value["touser"], "openid");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");

        let media = CustomerServiceMessage::media("openid", "image", "media")
            .expect("valid media message")
            .with_customer_service_account("support@account")
            .expect("valid sender account");
        assert_eq!(media.image.as_ref().unwrap()["media_id"], "media");
        assert!(media.validate().is_ok());

        assert!(CustomerServiceMessage::text("", "hello")
            .validate()
            .is_err());
        assert!(CustomerServiceMessage::text("openid", "")
            .validate()
            .is_err());
        assert!(CustomerServiceMessage::media("openid", "video", "media").is_err());

        let mut conflicting = CustomerServiceMessage::text("openid", "hello");
        conflicting.image = Some(json!({ "media_id": "media" }));
        assert!(conflicting.validate().is_err());
    }

    #[test]
    fn deserializes_official_base_and_card_depth_responses() {
        let callback: OfficialCallbackIpResponse = serde_json::from_value(json!({
            "ip_list": ["127.0.0.1", "127.0.0.2"]
        }))
        .unwrap();
        assert_eq!(callback.ip_list[0], "127.0.0.1");

        let check: OfficialCallbackCheckResponse = serde_json::from_value(json!({
            "dns": [{ "ip": "127.0.0.1", "real_operator": "UNICOM" }],
            "ping": [{ "ip": "127.0.0.1", "package_loss": "0%" }]
        }))
        .unwrap();
        assert_eq!(check.dns[0]["real_operator"], "UNICOM");

        let cards: CardListResponse = serde_json::from_value(json!({
            "card_id_list": ["card-1", "card-2"],
            "total_num": 2
        }))
        .unwrap();
        assert_eq!(cards.card_id_list[1], "card-2");
        assert_eq!(cards.total_num, Some(2));

        let update: CardUpdateResponse =
            serde_json::from_value(json!({ "send_check": true })).unwrap();
        assert_eq!(update.send_check, Some(true));
    }

    #[test]
    fn deserializes_oauth_and_mass_message_responses() {
        let user: OauthUserInfoResponse = serde_json::from_value(json!({
            "openid": "openid",
            "nickname": "Roze",
            "sex": 1,
            "province": "Guangdong",
            "city": "Shenzhen",
            "country": "CN",
            "headimgurl": "https://example.com/avatar.png",
            "privilege": ["chinaunicom"],
            "unionid": "unionid"
        }))
        .unwrap();
        assert_eq!(user.openid.as_deref(), Some("openid"));
        assert_eq!(user.privilege[0], "chinaunicom");

        let send: MassSendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "msg_id": 1000000001,
            "msg_data_id": 2247483650i64
        }))
        .unwrap();
        assert_eq!(send.msg_id, Some(1000000001));
        assert_eq!(send.msg_data_id, Some(2247483650));

        let preview_request = serde_json::to_value(MassPreviewRequest {
            touser: Some("openid".to_string()),
            towxname: None,
            msgtype: "mpnews".to_string(),
            message: json!({ "mpnews": { "media_id": "media" } }),
        })
        .unwrap();
        assert_eq!(preview_request["touser"], "openid");
        assert_eq!(preview_request["msgtype"], "mpnews");
        assert_eq!(preview_request["mpnews"]["media_id"], "media");
        assert!(preview_request.get("towxname").is_none());

        let preview: MassPreviewResponse =
            serde_json::from_value(json!({ "errcode": 0, "msg_id": 1001 })).unwrap();
        assert_eq!(preview.msg_id, Some(1001));

        let status: MassStatusResponse = serde_json::from_value(json!({
            "msg_id": 1001,
            "msg_status": "SEND_SUCCESS"
        }))
        .unwrap();
        assert_eq!(status.msg_status.as_deref(), Some("SEND_SUCCESS"));
        assert_eq!(status.status_kind(), Some(MassStatusKind::Success));
        assert!(status.is_success());
        assert!(status.status_kind().expect("status").is_terminal());
        assert_eq!(
            MassStatusKind::from_code("sending"),
            MassStatusKind::Sending
        );
        assert!(!MassStatusKind::Sending.is_terminal());
        assert_eq!(
            MassStatusKind::from_code("SEND_FAIL"),
            MassStatusKind::Failed
        );
        assert_eq!(MassStatusKind::from_code("DELETE"), MassStatusKind::Deleted);
        assert_eq!(MassStatusKind::from_code("UNKNOWN"), MassStatusKind::Other);
    }

    #[test]
    fn serializes_customer_service_session_and_record_requests() {
        let session = serde_json::to_value(json!({
            "kf_account": "test@test",
            "openid": "openid"
        }))
        .unwrap();
        assert_eq!(session["kf_account"], "test@test");

        let record_request = CustomerServiceMessageRecordRequest {
            starttime: 1,
            endtime: 2,
            msgid: 10,
            number: 20,
        };
        assert!(record_request.validate().is_ok());
        let request = serde_json::to_value(record_request).unwrap();
        assert_eq!(
            request,
            json!({ "starttime": 1, "endtime": 2, "msgid": 10, "number": 20 })
        );
        assert!(CustomerServiceMessageRecordRequest {
            starttime: 2,
            endtime: 1,
            msgid: 0,
            number: 20,
        }
        .validate()
        .is_err());
        assert!(CustomerServiceMessageRecordRequest {
            starttime: 1,
            endtime: 2,
            msgid: 0,
            number: 10_001,
        }
        .validate()
        .is_err());
    }

    #[test]
    fn deserializes_customer_service_session_and_record_responses() {
        let accounts: CustomerServiceAccountListResponse = serde_json::from_value(json!({
            "kf_list": [{
                "kf_account": "test@test",
                "kf_nick": "Roze Support",
                "kf_id": "1001",
                "kf_headimgurl": "https://example.com/kf.png"
            }]
        }))
        .unwrap();
        assert_eq!(accounts.kf_list[0].kf_account.as_deref(), Some("test@test"));

        let online: CustomerServiceOnlineAccountListResponse = serde_json::from_value(json!({
            "kf_online_list": [{
                "kf_account": "test@test",
                "status": 1,
                "accepted_case": 2
            }]
        }))
        .unwrap();
        assert_eq!(online.kf_online_list[0].accepted_case, Some(2));

        let sessions: CustomerServiceSessionListResponse = serde_json::from_value(json!({
            "sessionlist": [{ "createtime": 1800000000, "openid": "openid" }]
        }))
        .unwrap();
        assert_eq!(sessions.sessionlist[0].create_time, Some(1800000000));

        let waiting: CustomerServiceWaitCaseListResponse = serde_json::from_value(json!({
            "count": 1,
            "waitcaselist": [{ "latest_time": 1800000001, "openid": "openid" }]
        }))
        .unwrap();
        assert_eq!(waiting.count, Some(1));
        assert_eq!(waiting.waitcaselist[0].openid.as_deref(), Some("openid"));

        let current: CustomerServiceSessionResponse = serde_json::from_value(json!({
            "createtime": 1800000002,
            "kf_account": "test@test"
        }))
        .unwrap();
        assert_eq!(current.kf_account.as_deref(), Some("test@test"));

        let records: CustomerServiceMessageRecordResponse = serde_json::from_value(json!({
            "recordlist": [{
                "openid": "openid",
                "opercode": 2002,
                "text": "hello",
                "time": 1800000003,
                "worker": "test@test"
            }],
            "number": 1,
            "msgid": 11
        }))
        .unwrap();
        assert_eq!(records.recordlist[0].text.as_deref(), Some("hello"));
        assert_eq!(records.msgid, Some(11));
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
    fn rejects_inconsistent_material_responses() {
        let upload: MaterialMediaResponse =
            serde_json::from_value(json!({ "media_id": "media" })).unwrap();
        assert_eq!(upload.require_media_id().unwrap(), "media");
        assert!(upload.require_url().is_err());

        let article_image: MaterialMediaResponse =
            serde_json::from_value(json!({ "url": "https://example.com/article.png" })).unwrap();
        assert_eq!(
            article_image.require_url().unwrap(),
            "https://example.com/article.png"
        );
        assert!(article_image.require_media_id().is_err());
        let credential_url: MaterialMediaResponse = serde_json::from_value(json!({
            "url": "https://user:secret@example.com/article.png"
        }))
        .unwrap();
        assert!(credential_url.require_url().is_err());
        let fragment_url: MaterialMediaResponse = serde_json::from_value(json!({
            "url": "https://example.com/article.png#local"
        }))
        .unwrap();
        assert!(fragment_url.require_url().is_err());
        let malformed_media: MaterialMediaResponse =
            serde_json::from_value(json!({ "media_id": "media\nid" })).unwrap();
        assert!(malformed_media.require_media_id().is_err());

        let api_error: MaterialMediaResponse = serde_json::from_value(json!({
            "errcode": 40007,
            "errmsg": "invalid media_id"
        }))
        .unwrap();
        assert!(matches!(
            api_error.validate(),
            Err(WechatError::Api { code: 40007, .. })
        ));

        let empty_material: MaterialGetResponse = serde_json::from_value(json!({})).unwrap();
        assert!(empty_material.validate().is_err());

        let unsafe_video: MaterialGetResponse = serde_json::from_value(json!({
            "title": "video",
            "description": "description",
            "down_url": "file:///tmp/video.mp4"
        }))
        .unwrap();
        assert!(unsafe_video.validate().is_err());

        let inconsistent_list: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 1,
            "item_count": 2,
            "item": [{ "media_id": "media" }]
        }))
        .unwrap();
        assert!(inconsistent_list.validate().is_err());

        let invalid_stats: MaterialStatsResponse =
            serde_json::from_value(json!({ "voice_count": -1 })).unwrap();
        assert!(invalid_stats.validate().is_err());
        assert!(validate_material_upload("folder/image.png", b"image").is_err());
        assert!(validate_material_upload("image\n.png", b"image").is_err());
        assert!(validate_material_upload(&format!("{}.png", "x".repeat(252)), b"image").is_err());
    }

    #[test]
    fn deserializes_material_menu_template_and_user_depth_responses() {
        let material: MaterialGetResponse = serde_json::from_value(json!({
            "news_item": [{
                "title": "title",
                "author": "author",
                "digest": "digest",
                "content": "content",
                "content_source_url": "https://example.com",
                "thumb_media_id": "thumb"
            }]
        }))
        .unwrap();
        assert_eq!(material.news_item[0].title.as_deref(), Some("title"));
        material.validate().unwrap();

        let materials: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 1,
            "item": [{ "media_id": "media", "name": "image.png", "url": "https://example.com/i.png" }]
        }))
        .unwrap();
        assert_eq!(materials.total_count, Some(2));
        assert_eq!(materials.item[0].media_id.as_deref(), Some("media"));
        materials.validate().unwrap();
        assert_eq!(materials.next_offset(0).unwrap(), Some(1));

        let stats: MaterialStatsResponse = serde_json::from_value(json!({
            "voice_count": 1,
            "video_count": 2,
            "image_count": 3,
            "news_count": 4
        }))
        .unwrap();
        assert_eq!(stats.news_count, Some(4));
        assert_eq!(stats.total_count().unwrap(), 10);

        let menu: MenuGetResponse = serde_json::from_value(json!({
            "menu": {
                "button": [{
                    "name": "Open",
                    "type": "view",
                    "url": "https://example.com",
                    "button_extra": "kept"
                }],
                "menuid": 10,
                "menu_extra": "kept"
            },
            "conditionalmenu": [{
                "button": [{ "name": "VIP", "type": "click", "key": "vip" }],
                "matchrule": {
                    "tag_id": 2,
                    "sex": 1,
                    "client_platform_type": 3,
                    "rule_extra": "kept"
                },
                "menuid": "menu-1"
            }]
        }))
        .unwrap();
        assert_eq!(menu.conditionalmenu.len(), 1);
        assert_eq!(
            menu.menu.as_ref().unwrap().button[0].url.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(
            menu.menu.as_ref().unwrap().button[0].extra["button_extra"],
            "kept"
        );
        assert_eq!(menu.menu.as_ref().unwrap().menuid, Some(json!(10)));
        assert_eq!(
            menu.conditionalmenu[0].button[0].key.as_deref(),
            Some("vip")
        );
        assert_eq!(
            menu.conditionalmenu[0]
                .matchrule
                .as_ref()
                .and_then(|rule| rule.tag_id.as_ref())
                .and_then(Value::as_i64),
            Some(2)
        );
        assert_eq!(
            menu.conditionalmenu[0].matchrule.as_ref().unwrap().extra["rule_extra"],
            "kept"
        );

        let current: CurrentSelfMenuResponse = serde_json::from_value(json!({
            "is_menu_open": 1,
            "selfmenu_info": {
                "button": [{
                    "name": "Current",
                    "sub_button": {
                        "list": [{
                            "name": "Article",
                            "type": "news",
                            "news_info": {
                                "list": [{
                                    "title": "Published",
                                    "content_url": "https://example.com/article",
                                    "news_extra": "kept"
                                }]
                            }
                        }]
                    },
                    "button_extra": "kept"
                }],
                "menu_extra": "kept"
            }
        }))
        .unwrap();
        assert_eq!(current.is_menu_open, Some(1));
        assert!(current.is_open());
        assert_eq!(
            current.selfmenu_info.as_ref().unwrap().button[0]
                .name
                .as_deref(),
            Some("Current")
        );
        let current_button = &current.selfmenu_info.as_ref().unwrap().button[0];
        assert_eq!(current_button.extra["button_extra"], "kept");
        let child = &current_button.sub_button.as_ref().unwrap().list[0];
        assert_eq!(
            child.news_info.as_ref().unwrap().list[0]
                .content_url
                .as_deref(),
            Some("https://example.com/article")
        );
        assert_eq!(
            child.news_info.as_ref().unwrap().list[0].extra["news_extra"],
            "kept"
        );

        let matched: MenuTryMatchResponse =
            serde_json::from_value(json!({ "button": [{ "name": "Open" }] })).unwrap();
        assert_eq!(matched.button[0].name.as_deref(), Some("Open"));

        let semantic: OfficialSemanticQueryResponse = serde_json::from_value(json!({
            "query": "weather",
            "result": { "answer": "sunny" }
        }))
        .unwrap();
        assert_eq!(semantic.query.as_deref(), Some("weather"));

        let send: TemplateMessageSendResponse =
            serde_json::from_value(json!({ "errcode": 0, "msgid": 10001 })).unwrap();
        assert_eq!(send.msgid, Some(10001));

        let industry: TemplateIndustryResponse = serde_json::from_value(json!({
            "primary_industry": { "first_class": "IT", "second_class": "Software" }
        }))
        .unwrap();
        assert_eq!(
            industry.primary_industry.unwrap().second_class.as_deref(),
            Some("Software")
        );

        let added: TemplateAddResponse =
            serde_json::from_value(json!({ "template_id": "template" })).unwrap();
        assert_eq!(added.template_id.as_deref(), Some("template"));

        let templates: PrivateTemplateListResponse = serde_json::from_value(json!({
            "template_list": [{ "template_id": "template", "title": "Notice" }]
        }))
        .unwrap();
        assert_eq!(templates.template_list[0].title.as_deref(), Some("Notice"));

        let user: OfficialUserInfoResponse = serde_json::from_value(json!({
            "subscribe": 1,
            "openid": "openid",
            "tagid_list": [1, 2],
            "subscribe_scene": "ADD_SCENE_SEARCH"
        }))
        .unwrap();
        assert_eq!(user.tagid_list, vec![1, 2]);

        let batch: OfficialBatchUserInfoResponse = serde_json::from_value(json!({
            "user_info_list": [{ "openid": "openid" }]
        }))
        .unwrap();
        assert_eq!(batch.user_info_list[0].openid.as_deref(), Some("openid"));

        let users: OfficialUserListResponse = serde_json::from_value(json!({
            "total": 2,
            "count": 1,
            "data": { "openid": ["openid"] },
            "next_openid": "next"
        }))
        .unwrap();
        assert_eq!(users.data.unwrap().openid[0], "openid");

        let blacklist: OfficialBlacklistResponse = serde_json::from_value(json!({
            "total": 1,
            "count": 1,
            "data": { "openid": ["blocked"] }
        }))
        .unwrap();
        assert_eq!(blacklist.data.unwrap().openid[0], "blocked");
    }

    #[test]
    fn deserializes_card_create_response() {
        let response: CardCreateResponse =
            serde_json::from_value(json!({ "errcode": 0, "card_id": "card" })).unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.card_id.as_deref(), Some("card"));
    }

    #[test]
    fn deserializes_card_get_response() {
        let response: CardGetResponse = serde_json::from_value(json!({
            "card": {
                "card_type": "GROUPON",
                "groupon": {
                    "base_info": {
                        "id": "card",
                        "brand_name": "brand",
                        "title": "title",
                        "status": "CARD_STATUS_VERIFY_OK",
                        "date_info": {
                            "date_type": "DATE_TYPE_FIX_TIME_RANGE",
                            "begin_timestamp": 1800000000,
                            "end_timestamp": 1800100000
                        },
                        "sku": { "quantity": 100 }
                    },
                    "deal_detail": "deal",
                    "custom_field": "kept"
                }
            }
        }))
        .unwrap();

        let card = response.card.expect("card");
        assert_eq!(card.card_type.as_deref(), Some("GROUPON"));
        let groupon = card.groupon.expect("groupon");
        assert_eq!(groupon.deal_detail.as_deref(), Some("deal"));
        assert_eq!(groupon.extra["custom_field"], "kept");
        let base = groupon.base_info.expect("base_info");
        assert_eq!(base.id.as_deref(), Some("card"));
        assert_eq!(base.brand_name.as_deref(), Some("brand"));
        assert_eq!(base.status_kind(), Some(CardStatusKind::Approved));
        assert!(base.is_approved());
        assert!(!base.needs_review());
        assert_eq!(
            base.date_info
                .as_ref()
                .and_then(|info| info.begin_timestamp),
            Some(1800000000)
        );
        assert_eq!(base.sku.as_ref().and_then(|sku| sku.quantity), Some(100));
        let pending_base = CardBaseInfo {
            id: None,
            logo_url: None,
            code_type: None,
            brand_name: None,
            title: None,
            sub_title: None,
            color: None,
            notice: None,
            description: None,
            date_info: None,
            sku: None,
            get_limit: None,
            use_limit: None,
            status: Some("CARD_STATUS_NOT_VERIFY".to_string()),
            extra: Value::Null,
        };
        assert_eq!(
            pending_base.status_kind(),
            Some(CardStatusKind::PendingReview)
        );
        assert!(pending_base.needs_review());
        let failed_base = CardBaseInfo {
            status: Some("CARD_STATUS_VERIFY_FALL".to_string()),
            ..pending_base
        };
        assert_eq!(
            failed_base.status_kind(),
            Some(CardStatusKind::ReviewFailed)
        );
        let unknown_base = CardBaseInfo {
            status: Some("CARD_STATUS_UNKNOWN".to_string()),
            ..failed_base
        };
        assert_eq!(unknown_base.status_kind(), Some(CardStatusKind::Other));
    }

    #[test]
    fn serializes_template_subscribe_and_user_tag_requests() {
        let subscribe = serde_json::to_value(TemplateSubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "template".to_string(),
            url: "https://example.com".to_string(),
            miniprogram: None,
            scene: "1000".to_string(),
            title: "subscribe".to_string(),
            data: json!({ "content": { "value": "hello" } }),
        })
        .unwrap();
        assert_eq!(subscribe["touser"], "openid");
        assert_eq!(subscribe["scene"], "1000");
        assert!(subscribe.get("miniprogram").is_none());

        let tag_create = json!({ "tag": { "name": "vip" } });
        assert_eq!(tag_create["tag"]["name"], "vip");

        let tag_users = json!({ "openid_list": ["openid"], "tagid": 2 });
        assert_eq!(tag_users["openid_list"][0], "openid");
        assert_eq!(tag_users["tagid"], 2);
    }

    #[test]
    fn deserializes_user_tag_and_change_openid_responses() {
        let created: OfficialUserTagResponse = serde_json::from_value(json!({
            "tag": { "id": 2, "name": "vip", "count": 10 }
        }))
        .unwrap();
        assert_eq!(created.tag.as_ref().map(|tag| tag.id), Some(2));

        let tags: OfficialUserTagListResponse = serde_json::from_value(json!({
            "tags": [{ "id": 2, "name": "vip", "count": 10 }]
        }))
        .unwrap();
        assert_eq!(tags.tags[0].name, "vip");

        let tag_ids: OfficialUserTagIdsResponse =
            serde_json::from_value(json!({ "tagid_list": [2, 3] })).unwrap();
        assert_eq!(tag_ids.tagid_list, vec![2, 3]);

        let users: OfficialUsersOfTagResponse = serde_json::from_value(json!({
            "count": 1,
            "data": { "openid": ["openid"] },
            "next_openid": "next"
        }))
        .unwrap();
        assert_eq!(users.data.as_ref().unwrap().openid[0], "openid");
        assert_eq!(users.next_openid.as_deref(), Some("next"));

        let tagged: OfficialTagUsersResponse = serde_json::from_value(json!({
            "openid_list": ["openid"],
            "tagid": 2
        }))
        .unwrap();
        assert_eq!(tagged.tagid, Some(2));

        let changed: OfficialChangeOpenidResponse = serde_json::from_value(json!({
            "result_list": [{
                "ori_openid": "old",
                "new_openid": "new",
                "err_msg": "ok"
            }]
        }))
        .unwrap();
        assert_eq!(changed.result_list[0].new_openid.as_deref(), Some("new"));
    }

    #[test]
    fn serializes_official_semantic_query_request() {
        let value = serde_json::to_value(OfficialSemanticQueryRequest {
            query: "nearby coffee".to_string(),
            category: "map".to_string(),
            appid: "wx123".to_string(),
            optional: json!({
                "uid": "openid",
                "city": "Shanghai",
                "latitude": 31.2,
                "longitude": 121.5
            }),
        })
        .unwrap();

        assert_eq!(value["query"], "nearby coffee");
        assert_eq!(value["category"], "map");
        assert_eq!(value["appid"], "wx123");
        assert_eq!(value["uid"], "openid");
        assert_eq!(value["city"], "Shanghai");
    }

    #[test]
    fn deserializes_official_auto_reply_response() {
        let response: OfficialAutoReplyInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "is_add_friend_reply_open": 1,
            "is_autoreply_open": 1,
            "add_friend_autoreply_info": { "type": "text", "content": "hello" },
            "message_default_autoreply_info": { "type": "text", "content": "default" },
            "keyword_autoreply_info": {
                "list": [{
                    "rule_name": "rule",
                    "reply_mode": "reply_all",
                    "keyword_list_info": [{ "type": "text", "match_mode": "contain", "content": "hi" }],
                    "reply_list_info": [{ "type": "text", "content": "hello" }]
                }]
            }
        }))
        .unwrap();

        assert_eq!(response.is_autoreply_open, Some(1));
        assert_eq!(
            response.add_friend_autoreply_info.unwrap()["content"],
            "hello"
        );
        assert_eq!(
            response.keyword_autoreply_info.unwrap()["list"][0]["rule_name"],
            "rule"
        );
    }

    #[test]
    fn deserializes_official_poi_responses() {
        let created: OfficialPoiMutationResponse =
            serde_json::from_value(json!({ "errcode": 0, "poi_id": 100 })).unwrap();
        assert_eq!(created.poi_id, Some(100));

        let get: OfficialPoiGetResponse = serde_json::from_value(json!({
            "business": {
                "base_info": {
                    "poi_id": "100",
                    "business_name": "Roze Store",
                    "city": "Shanghai"
                }
            }
        }))
        .unwrap();
        assert_eq!(
            get.business.unwrap()["base_info"]["business_name"],
            "Roze Store"
        );

        let list: OfficialPoiListResponse = serde_json::from_value(json!({
            "business_list": [{
                "base_info": { "poi_id": "100", "business_name": "Roze Store" }
            }],
            "total_count": "1"
        }))
        .unwrap();
        assert_eq!(list.total_count.as_deref(), Some("1"));
        assert_eq!(
            list.business_list[0]["base_info"]["business_name"],
            "Roze Store"
        );
    }

    #[test]
    fn serializes_official_device_and_goods_requests() {
        let device_message = serde_json::to_value(OfficialDeviceMessageRequest {
            device_type: "gh_device".to_string(),
            device_id: "device".to_string(),
            open_id: "openid".to_string(),
            content: "hello".to_string(),
        })
        .unwrap();
        assert_eq!(device_message["device_id"], "device");
        assert_eq!(device_message["open_id"], "openid");

        let authorize = serde_json::to_value(OfficialDeviceAuthorizeRequest {
            device_num: "1".to_string(),
            device_list: vec![json!({
                "id": "device",
                "mac": "00:11",
                "connect_protocol": "3",
                "auth_key": "key"
            })],
            op_type: "1".to_string(),
            product_id: "product".to_string(),
        })
        .unwrap();
        assert_eq!(authorize["device_num"], "1");
        assert_eq!(authorize["device_list"][0]["id"], "device");

        let bind = serde_json::to_value(OfficialDeviceBindRequest {
            ticket: "ticket".to_string(),
            device_id: "device".to_string(),
            openid: "openid".to_string(),
        })
        .unwrap();
        assert_eq!(bind["ticket"], "ticket");

        let goods = serde_json::to_value(OfficialGoodsProductRequest {
            product: vec![json!({
                "pid": "pid",
                "title": "Product",
                "sku_info": { "sku_item": [] }
            })],
        })
        .unwrap();
        assert_eq!(goods["product"][0]["pid"], "pid");
        assert_eq!(goods["product"][0]["title"], "Product");
    }

    #[test]
    fn deserializes_official_device_goods_and_ocr_responses() {
        let message: OfficialDeviceMessageResponse =
            serde_json::from_value(json!({ "ret": 0, "ret_info": "ok" })).unwrap();
        assert_eq!(message.ret, Some(0));
        assert_eq!(message.ret_info.as_deref(), Some("ok"));

        let qrcode: OfficialDeviceCreateQrCodeResponse = serde_json::from_value(json!({
            "device_num": 1,
            "code_list": [{ "device_id": "device", "ticket": "ticket" }]
        }))
        .unwrap();
        assert_eq!(qrcode.device_num, Some(1));
        assert_eq!(qrcode.code_list[0]["ticket"], "ticket");

        let authorized: OfficialDeviceAuthorizeResponse = serde_json::from_value(json!({
            "resp": [{ "base_info": { "device_id": "device" }, "errcode": 0 }]
        }))
        .unwrap();
        assert_eq!(authorized.resp[0]["base_info"]["device_id"], "device");

        let create_id: OfficialDeviceCreateIdResponse = serde_json::from_value(json!({
            "resp_msg": { "ret_code": 0, "error_info": "ok" }
        }))
        .unwrap();
        assert_eq!(create_id.resp_msg.unwrap()["ret_code"], 0);

        let bind: OfficialDeviceBindResponse =
            serde_json::from_value(json!({ "base_resp": { "errcode": 0, "errmsg": "ok" } }))
                .unwrap();
        assert_eq!(bind.base_resp.unwrap()["errmsg"], "ok");

        let add: OfficialGoodsProductAddResponse =
            serde_json::from_value(json!({ "status_ticket": "ticket" })).unwrap();
        assert_eq!(add.status_ticket.as_deref(), Some("ticket"));

        let status: OfficialGoodsProductStatusResponse = serde_json::from_value(json!({
            "result": { "succ_cnt": 1, "fail_cnt": 0, "progress": "100%" }
        }))
        .unwrap();
        assert_eq!(status.result.unwrap()["succ_cnt"], 1);

        let product: OfficialGoodsProductGetResponse = serde_json::from_value(json!({
            "product": { "pid": "pid", "title": "Product" }
        }))
        .unwrap();
        assert_eq!(product.product.unwrap()["pid"], "pid");

        let id_card: OfficialOcrIdCardResponse = serde_json::from_value(json!({
            "type": "Front",
            "name": "Alice",
            "id": "123",
            "addr": "Shanghai"
        }))
        .unwrap();
        assert_eq!(id_card.id_type.as_deref(), Some("Front"));
        assert_eq!(id_card.name.as_deref(), Some("Alice"));

        let bank: OfficialOcrBankCardResponse =
            serde_json::from_value(json!({ "number": "6222" })).unwrap();
        assert_eq!(bank.number.as_deref(), Some("6222"));

        let vehicle: OfficialOcrVehicleLicenseResponse = serde_json::from_value(json!({
            "id_num": "id",
            "name": "Alice",
            "car_class": "C1"
        }))
        .unwrap();
        assert_eq!(vehicle.car_class.as_deref(), Some("C1"));

        let biz: OfficialOcrBizLicenseResponse = serde_json::from_value(json!({
            "reg_num": "reg",
            "enterprise_name": "Roze",
            "img_size": { "w": 100, "h": 80 }
        }))
        .unwrap();
        assert_eq!(biz.enterprise_name.as_deref(), Some("Roze"));
        assert_eq!(biz.img_size.unwrap()["w"], 100);

        let common: OfficialOcrCommonResponse = serde_json::from_value(json!({
            "items": [{ "text": "hello" }],
            "img_size": { "w": 100, "h": 80 }
        }))
        .unwrap();
        assert_eq!(common.items[0]["text"], "hello");

        let plate: OfficialOcrPlateNumberResponse =
            serde_json::from_value(json!({ "number": "PLATE123" })).unwrap();
        assert_eq!(plate.number.as_deref(), Some("PLATE123"));
    }

    #[test]
    fn serializes_official_store_wifi_guide_and_shake_requests() {
        let store = serde_json::to_value(OfficialStoreBaseInfo {
            name: "Roze Store".to_string(),
            longitude: "121.5".to_string(),
            latitude: "31.2".to_string(),
            province: "Shanghai".to_string(),
            city: "Shanghai".to_string(),
            district: "Pudong".to_string(),
            address: "Road".to_string(),
            category: "Food".to_string(),
            telephone: "10086".to_string(),
            photo: "media".to_string(),
            license: "license".to_string(),
            introduction: "intro".to_string(),
            districtid: "310000".to_string(),
        })
        .unwrap();
        assert_eq!(store["introduct"], "intro");
        assert_eq!(store["districtid"], "310000");

        let wifi_home = serde_json::to_value(OfficialWifiSetHomePageRequest {
            shop_id: 1,
            template_id: 2,
            struct_data: OfficialWifiHomePageStruct {
                url: "https://example.com".to_string(),
            },
        })
        .unwrap();
        assert_eq!(wifi_home["struct"]["url"], "https://example.com");
        assert_eq!(wifi_home["template_id"], 2);

        let guide = serde_json::to_value(OfficialGuideAdviserRequest {
            account: OfficialGuideAccountRequest {
                guide_account: Some("guide".to_string()),
                guide_openid: None,
            },
            guide_headimgurl: Some("https://example.com/avatar.png".to_string()),
            guide_nickname: None,
        })
        .unwrap();
        assert_eq!(guide["guide_account"], "guide");
        assert!(guide.get("guide_openid").is_none());
        assert!(guide.get("guide_nickname").is_none());

        let tag = serde_json::to_value(OfficialGuideBuyersByTagRequest {
            account: OfficialGuideAccountRequest {
                guide_account: None,
                guide_openid: Some("openid".to_string()),
            },
            push_count: Some(10),
            tag_values: vec!["vip".to_string()],
        })
        .unwrap();
        assert_eq!(tag["guide_openid"], "openid");
        assert_eq!(tag["tag_values"][0], "vip");

        let search = serde_json::to_value(OfficialShakeAroundDeviceSearchRequest {
            search_type: 3,
            device_identifiers: Vec::new(),
            apply_id: Some(10),
            last_seen: Some(0),
            count: Some(20),
        })
        .unwrap();
        assert_eq!(search["type"], 3);
        assert_eq!(search["apply_id"], 10);
        assert!(search.get("device_identifiers").is_none());

        let page = serde_json::to_value(OfficialShakeAroundPageUpdateRequest {
            page: OfficialShakeAroundPageInfoRequest {
                title: "Title".to_string(),
                description: "Desc".to_string(),
                page_url: "https://example.com/page".to_string(),
                comment: "Comment".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
            },
            page_id: 7,
        })
        .unwrap();
        assert_eq!(page["page_id"], 7);
        assert_eq!(page["page_url"], "https://example.com/page");
    }

    #[test]
    fn deserializes_official_store_wifi_guide_and_shake_responses() {
        let categories: OfficialStoreCategoryResponse = serde_json::from_value(json!({
            "data": { "all_category_info": { "categories": [{ "id": 1, "name": "Food" }] } }
        }))
        .unwrap();
        assert_eq!(
            categories.data.unwrap()["all_category_info"]["categories"][0]["name"],
            "Food"
        );

        let districts: OfficialStoreDistrictResponse = serde_json::from_value(json!({
            "status": 0,
            "message": "query ok",
            "data_version": "v1",
            "result": [[{ "id": "310000", "fullname": "Shanghai" }]]
        }))
        .unwrap();
        assert_eq!(districts.status, Some(0));
        assert_eq!(districts.result.unwrap()[0][0]["id"], "310000");

        let store_list: OfficialStoreListResponse = serde_json::from_value(json!({
            "business_list": [{ "base_info": { "poi_id": "poi" } }],
            "total_count": 1
        }))
        .unwrap();
        assert_eq!(store_list.total_count, Some(1));
        assert_eq!(
            store_list.business_list.unwrap()[0]["base_info"]["poi_id"],
            "poi"
        );

        let wifi: OfficialWifiSummaryResponse = serde_json::from_value(json!({
            "data": [{ "shop_id": "1", "total_user": 10 }]
        }))
        .unwrap();
        assert_eq!(wifi.data.unwrap()[0]["total_user"], 10);

        let shop: OfficialWifiShopGetResponse = serde_json::from_value(json!({
            "data": { "shop_name": "Roze Store", "ssid": "RozeWiFi" }
        }))
        .unwrap();
        assert_eq!(shop.data.unwrap()["ssid"], "RozeWiFi");

        let adviser: OfficialGuideGetAdviserResponse = serde_json::from_value(json!({
            "guide_account": "guide",
            "guide_nickname": "Alice"
        }))
        .unwrap();
        assert_eq!(adviser.guide_account.as_deref(), Some("guide"));
        assert_eq!(adviser.guide_nickname.as_deref(), Some("Alice"));

        let buyers: OfficialGuideBuyerRelationListResponse = serde_json::from_value(json!({
            "total_num": 1,
            "list": [{ "openid": "buyer", "guide_account": "guide" }]
        }))
        .unwrap();
        assert_eq!(buyers.total_num, Some(1));
        assert_eq!(buyers.list.unwrap()[0]["openid"], "buyer");

        let material: OfficialGuideCardMaterialResponse = serde_json::from_value(json!({
            "card_list": [{ "title": "Card", "appid": "app" }]
        }))
        .unwrap();
        assert_eq!(material.card_list.unwrap()[0]["title"], "Card");

        let register: OfficialShakeAroundAccountRegisterResponse = serde_json::from_value(json!({
            "data": { "audit_status": 1, "audit_comment": "ok" }
        }))
        .unwrap();
        assert_eq!(register.data.unwrap()["audit_status"], 1);

        let search: OfficialShakeAroundDeviceSearchResponse = serde_json::from_value(json!({
            "data": {
                "devices": [{ "device_id": 1, "uuid": "uuid" }],
                "total_count": 1
            }
        }))
        .unwrap();
        assert_eq!(search.data.unwrap()["devices"][0]["uuid"], "uuid");

        let stats: OfficialShakeAroundStatsDeviceListResponse = serde_json::from_value(json!({
            "data": { "devices": [{ "device_id": 1, "shake_pv": 2 }] },
            "date": 20260709,
            "total_count": 1,
            "page_index": 1
        }))
        .unwrap();
        assert_eq!(stats.date, Some(20260709));
        assert_eq!(stats.data.unwrap()["devices"][0]["shake_pv"], 2);
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
        assert_eq!(
            response.card.expect("card").card_id.as_deref(),
            Some("card")
        );
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
        let value =
            serde_json::to_value(MaterialListRequest::new(MaterialListKind::News, 0, 20)).unwrap();
        assert_eq!(value, json!({ "type": "news", "offset": 0, "count": 20 }));
    }

    #[test]
    fn validates_material_requests_and_typed_news_content() {
        let article = Article {
            title: "Release notes".to_string(),
            thumb_media_id: "thumb-media".to_string(),
            author: "Roze".to_string(),
            digest: "Production update".to_string(),
            show_cover_pic: 1,
            content: "<p>Ready</p>".to_string(),
            content_source_url: "https://example.com/releases/1".to_string(),
            need_open_comment: Some(1),
            only_fans_can_comment: Some(0),
        };
        assert!(article.validate().is_ok());

        let mut invalid_article = article;
        invalid_article.only_fans_can_comment = Some(1);
        invalid_article.need_open_comment = Some(0);
        assert!(invalid_article.validate().is_err());
        assert!(validate_material_articles(&[]).is_err());
        assert!(validate_material_upload("image.png", b"image").is_ok());
        assert!(validate_material_upload("", b"image").is_err());
        assert!(validate_material_upload("image.png", b"").is_err());

        assert!(MaterialListRequest::new(MaterialListKind::Image, 0, 20)
            .validate()
            .is_ok());
        assert!(MaterialListRequest {
            kind: "unknown".to_string(),
            offset: -1,
            count: 21,
        }
        .validate()
        .is_err());

        let item: MaterialListItem = serde_json::from_value(json!({
            "media_id": "news-media",
            "content": {
                "news_item": [{
                    "title": "Release notes",
                    "thumb_media_id": "thumb-media",
                    "content": "<p>Ready</p>",
                    "article_revision": 2
                }],
                "create_time": 1,
                "update_time": 2,
                "content_revision": 3
            }
        }))
        .unwrap();
        let content = item.news_content().unwrap().unwrap();
        assert_eq!(content.news_item[0].title.as_deref(), Some("Release notes"));
        assert_eq!(content.news_item[0].extra["article_revision"], 2);
        assert_eq!(content.update_time, Some(2));
        assert_eq!(content.extra["content_revision"], 3);
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

        let add = serde_json::to_value(PublishDraftAddRequest::new(vec![article.clone()])).unwrap();
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

        let batch = serde_json::to_value(PublishBatchGetRequest::new(0, 20, true)).unwrap();
        assert_eq!(batch, json!({ "offset": 0, "count": 20, "no_content": 1 }));
    }

    #[test]
    fn validates_publish_workflow_requests() {
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
        assert!(PublishDraftAddRequest::new(vec![article.clone()])
            .validate()
            .is_ok());
        assert!(PublishDraftUpdateRequest {
            media_id: "draft".to_string(),
            index: 0,
            articles: article.clone(),
        }
        .validate()
        .is_ok());
        assert!(PublishDraftAddRequest::new(Vec::new()).validate().is_err());

        let mut invalid_article = article;
        invalid_article.content_source_url = "javascript:alert(1)".to_string();
        assert!(invalid_article.validate().is_err());
        assert!(PublishBatchGetRequest::new(0, 20, true).validate().is_ok());
        assert!(PublishBatchGetRequest {
            offset: -1,
            count: 21,
            no_content: 2,
        }
        .validate()
        .is_err());
        assert!(validate_publish_index(7).is_ok());
        assert!(validate_publish_index(8).is_err());
        assert!(validate_publish_delete_index(0).is_ok());
        assert!(validate_publish_delete_index(8).is_ok());
        assert!(validate_publish_delete_index(9).is_err());
    }

    #[test]
    fn deserializes_publish_responses() {
        let add: PublishDraftAddResponse = serde_json::from_value(json!({
            "errcode": 0,
            "media_id": "media",
            "request_id": "draft-add"
        }))
        .unwrap();
        assert_eq!(add.media_id.as_deref(), Some("media"));
        assert_eq!(add.extra["request_id"], "draft-add");
        assert!(add.validate().is_ok());

        let draft: PublishDraftGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "news_item": [{
                "title": "Title",
                "author": "Roze",
                "thumb_media_id": "thumb",
                "content": "<p>Hello</p>",
                "url": "https://example.com/article",
                "article_extra": "kept"
            }],
            "create_time": 1,
            "update_time": 2,
            "request_id": "draft-get"
        }))
        .unwrap();
        assert_eq!(draft.news_item[0].title.as_deref(), Some("Title"));
        assert_eq!(draft.update_time, Some(2));
        assert_eq!(draft.extra["request_id"], "draft-get");
        assert_eq!(draft.news_item[0].extra["article_extra"], "kept");
        assert!(draft.validate().is_ok());

        let list: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 1,
            "item_count": 1,
            "request_id": "batch-get",
            "item": [{
                "media_id": "media",
                "article_id": "article",
                "content": {
                    "news_item": [{
                        "title": "Title",
                        "thumb_media_id": "thumb",
                        "content": "<p>Hello</p>",
                        "article_extra": "kept"
                    }],
                    "create_time": 1,
                    "update_time": 2,
                    "content_extra": "kept"
                },
                "update_time": 2,
                "item_extra": "kept"
            }]
        }))
        .unwrap();
        assert_eq!(list.extra["request_id"], "batch-get");
        assert_eq!(list.item[0].article_id.as_deref(), Some("article"));
        assert_eq!(list.item[0].extra["item_extra"], "kept");
        assert_eq!(
            list.item[0].content.as_ref().expect("content").extra["content_extra"],
            "kept"
        );
        assert_eq!(
            list.item[0].content.as_ref().expect("content").news_item[0]
                .title
                .as_deref(),
            Some("Title")
        );
        assert_eq!(
            list.item[0].content.as_ref().expect("content").news_item[0].extra["article_extra"],
            "kept"
        );
        assert!(list
            .validate_for(PublishBatchKind::Published, false)
            .is_ok());
        assert_eq!(
            list.next_offset(0, PublishBatchKind::Published, false)
                .unwrap(),
            None
        );

        let switch_status: PublishDraftSwitchStatusResponse = serde_json::from_value(json!({
            "is_open": 1,
            "total_count": 3,
            "item_count": 2,
            "request_id": "switch"
        }))
        .unwrap();
        assert_eq!(switch_status.is_open, Some(1));
        assert!(switch_status.is_open());
        assert_eq!(switch_status.extra["request_id"], "switch");
        assert!(switch_status.validate().is_ok());

        let submit: PublishSubmitResponse = serde_json::from_value(json!({
            "publish_id": 10001,
            "request_id": "submit"
        }))
        .unwrap();
        assert_eq!(submit.publish_id, Some(10001));
        assert_eq!(submit.extra["request_id"], "submit");
        assert_eq!(submit.require_publish_id().unwrap(), 10001);

        let status: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 10001,
            "publish_status": 0,
            "article_id": "article",
            "article_detail": {
                "count": 1,
                "item": [{
                    "idx": 1,
                    "article_url": "https://example.com/article",
                    "article_item_extra": "kept"
                }],
                "detail_extra": "kept"
            },
            "fail_idx": [],
            "request_id": "status"
        }))
        .unwrap();
        assert_eq!(status.publish_status, Some(0));
        assert_eq!(status.status_kind(), Some(PublishStatusKind::Success));
        assert!(status.is_success());
        assert!(status.is_terminal());
        assert_eq!(status.article_ids(), vec!["article"]);
        assert!(!status.is_pending());
        assert!(!status.is_failed());
        assert!(status.validate_for(10001).is_ok());
        assert_eq!(status.extra["request_id"], "status");
        assert_eq!(
            status
                .article_detail
                .as_ref()
                .expect("article_detail")
                .extra["detail_extra"],
            "kept"
        );
        assert_eq!(
            status.article_detail.as_ref().expect("article_detail").item[0]
                .article_url
                .as_deref(),
            Some("https://example.com/article")
        );
        assert_eq!(
            status.article_detail.expect("article_detail").item[0].extra["article_item_extra"],
            "kept"
        );
        let publishing: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 1 })).unwrap();
        assert_eq!(
            publishing.status_kind(),
            Some(PublishStatusKind::Publishing)
        );
        assert!(publishing.is_pending());
        let original_failed: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 2, "fail_idx": [1] })).unwrap();
        assert_eq!(
            original_failed.status_kind(),
            Some(PublishStatusKind::OriginalFailed)
        );
        assert!(original_failed.is_failed());
        let failed: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 3 })).unwrap();
        assert_eq!(failed.status_kind(), Some(PublishStatusKind::Failed));
        assert!(failed.is_failed());
        let audit_refused: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 4 })).unwrap();
        assert_eq!(
            audit_refused.status_kind(),
            Some(PublishStatusKind::AuditRefused)
        );
        assert!(audit_refused.is_failed());
        let user_deleted: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 5 })).unwrap();
        assert_eq!(
            user_deleted.status_kind(),
            Some(PublishStatusKind::UserDeleted)
        );
        assert!(user_deleted.is_failed());
        assert!(user_deleted.is_terminal());
        let system_banned: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 6 })).unwrap();
        assert_eq!(
            system_banned.status_kind(),
            Some(PublishStatusKind::SystemBanned)
        );
        assert!(system_banned.is_failed());
        let unknown_status: PublishStatusResponse =
            serde_json::from_value(json!({ "publish_status": 99 })).unwrap();
        assert_eq!(unknown_status.status_kind(), Some(PublishStatusKind::Other));

        let article: PublishArticleResponse = serde_json::from_value(json!({
            "news_item": [{
                "title": "Published",
                "thumb_media_id": "thumb",
                "content": "<p>Published</p>",
                "url": "https://example.com/published",
                "article_extra": "kept"
            }],
            "request_id": "article"
        }))
        .unwrap();
        assert_eq!(article.news_item[0].title.as_deref(), Some("Published"));
        assert_eq!(article.news_item[0].extra["article_extra"], "kept");
        assert_eq!(article.extra["request_id"], "article");
        assert!(article.validate().is_ok());
    }

    #[test]
    fn rejects_inconsistent_publish_responses() {
        let api_error: PublishDraftAddResponse = serde_json::from_value(json!({
            "errcode": 40007,
            "errmsg": "invalid media"
        }))
        .unwrap();
        assert!(matches!(
            api_error.validate(),
            Err(WechatError::Api { code: 40007, .. })
        ));
        let missing_media: PublishDraftAddResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_media.validate().is_err());
        let malformed_media: PublishDraftAddResponse =
            serde_json::from_value(json!({ "media_id": "media\nid" })).unwrap();
        assert!(malformed_media.validate().is_err());

        let empty_draft: PublishDraftGetResponse =
            serde_json::from_value(json!({ "create_time": 1, "update_time": 2 })).unwrap();
        assert!(empty_draft.validate().is_err());
        let reversed_draft: PublishDraftGetResponse = serde_json::from_value(json!({
            "news_item": [{
                "title": "Title",
                "thumb_media_id": "thumb",
                "content": "<p>Hello</p>"
            }],
            "create_time": 3,
            "update_time": 2
        }))
        .unwrap();
        assert!(reversed_draft.validate().is_err());
        let malformed_article: PublishDraftGetResponse = serde_json::from_value(json!({
            "news_item": [{
                "title": "Title",
                "thumb_media_id": "thumb",
                "content": "<p>Hello</p>",
                "only_fans_can_comment": 1,
                "need_open_comment": 0
            }]
        }))
        .unwrap();
        assert!(malformed_article.validate().is_err());

        let missing_count: PublishDraftCountResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_count.validate().is_err());
        let negative_count: PublishDraftCountResponse =
            serde_json::from_value(json!({ "total_count": -1 })).unwrap();
        assert!(negative_count.validate().is_err());

        let draft_page: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 1,
            "item": [{ "media_id": "draft-1" }]
        }))
        .unwrap();
        assert!(draft_page
            .validate_for(PublishBatchKind::Draft, true)
            .is_ok());
        assert_eq!(
            draft_page
                .next_offset(0, PublishBatchKind::Draft, true)
                .unwrap(),
            Some(1)
        );
        assert!(draft_page
            .validate_for(PublishBatchKind::Published, true)
            .is_err());

        let mismatched_page: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 2,
            "item": [{ "media_id": "draft-1" }]
        }))
        .unwrap();
        assert!(mismatched_page
            .validate_for(PublishBatchKind::Draft, true)
            .is_err());
        let duplicate_page: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 2,
            "item": [
                { "media_id": "draft-1" },
                { "media_id": "draft-1" }
            ]
        }))
        .unwrap();
        assert!(duplicate_page
            .validate_for(PublishBatchKind::Draft, true)
            .is_err());
        let missing_content_page: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 1,
            "item_count": 1,
            "item": [{ "article_id": "article-1" }]
        }))
        .unwrap();
        assert!(missing_content_page
            .validate_for(PublishBatchKind::Published, false)
            .is_err());
        let stalled_page: PublishBatchGetResponse = serde_json::from_value(json!({
            "total_count": 1,
            "item_count": 0,
            "item": []
        }))
        .unwrap();
        assert!(stalled_page
            .next_offset(0, PublishBatchKind::Draft, true)
            .is_err());

        let invalid_switch: PublishDraftSwitchStatusResponse =
            serde_json::from_value(json!({ "is_open": 2 })).unwrap();
        assert!(invalid_switch.validate().is_err());
        let inconsistent_switch: PublishDraftSwitchStatusResponse = serde_json::from_value(json!({
            "is_open": 1,
            "total_count": 1,
            "item_count": 2
        }))
        .unwrap();
        assert!(inconsistent_switch.validate().is_err());

        let missing_publish_id: PublishSubmitResponse =
            serde_json::from_value(json!({ "publish_id": 0 })).unwrap();
        assert!(missing_publish_id.validate().is_err());
        let publish_api_error: PublishSubmitResponse = serde_json::from_value(json!({
            "errcode": 53503,
            "errmsg": "publish rejected"
        }))
        .unwrap();
        assert!(matches!(
            publish_api_error.validate(),
            Err(WechatError::Api { code: 53503, .. })
        ));

        let wrong_publish_id: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 2,
            "publish_status": 1
        }))
        .unwrap();
        assert!(wrong_publish_id.validate_for(1).is_err());
        let success_without_article: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 0
        }))
        .unwrap();
        assert!(success_without_article.validate().is_err());
        let malformed_article_ids: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 3,
            "article_id": ["article", 2]
        }))
        .unwrap();
        assert!(malformed_article_ids.validate().is_err());
        let duplicate_article_ids: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 3,
            "article_id": ["article", "article"]
        }))
        .unwrap();
        assert!(duplicate_article_ids.validate().is_err());
        let inconsistent_detail: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 3,
            "article_detail": {
                "count": 2,
                "item": [{
                    "idx": 1,
                    "article_url": "https://example.com/article"
                }]
            }
        }))
        .unwrap();
        assert!(inconsistent_detail.validate().is_err());
        let unsafe_detail: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 3,
            "article_detail": {
                "count": 1,
                "item": [{
                    "idx": 1,
                    "article_url": "https://user:secret@example.com/article"
                }]
            }
        }))
        .unwrap();
        assert!(unsafe_detail.validate().is_err());
        let pending_with_failures: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 1,
            "fail_idx": [1]
        }))
        .unwrap();
        assert!(pending_with_failures.validate().is_err());
        let duplicate_failures: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 3,
            "fail_idx": [1, 1]
        }))
        .unwrap();
        assert!(duplicate_failures.validate().is_err());
        let future_status: PublishStatusResponse = serde_json::from_value(json!({
            "publish_id": 1,
            "publish_status": 99
        }))
        .unwrap();
        assert!(future_status.validate().is_ok());

        let empty_published_article: PublishArticleResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(empty_published_article.validate().is_err());
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
    fn validates_mass_message_request_and_response_matrix() {
        let send_all = MassSendAllRequest {
            filter: MassSendFilter {
                is_to_all: true,
                tag_id: None,
            },
            msgtype: "text".to_string(),
            message: json!({ "text": { "content": "hello" } }),
            send_ignore_reprint: Some(1),
        };
        assert!(send_all.validate().is_ok());

        let mut invalid_filter = send_all.clone();
        invalid_filter.filter.tag_id = Some(2);
        assert!(invalid_filter.validate().is_err());
        let mut invalid_payload = send_all.clone();
        invalid_payload.message = json!({ "image": { "media_id": "media" } });
        assert!(invalid_payload.validate().is_err());
        let mut invalid_type = send_all;
        invalid_type.msgtype = "video".to_string();
        assert!(invalid_type.validate().is_err());

        let recipients = MassSendOpenIdsRequest {
            touser: vec!["openid-1".to_string(), "openid-2".to_string()],
            msgtype: "image".to_string(),
            message: json!({ "image": { "media_id": "media" } }),
            send_ignore_reprint: None,
        };
        assert!(recipients.validate().is_ok());
        let mut duplicate = recipients.clone();
        duplicate.touser[1] = "openid-1".to_string();
        assert!(duplicate.validate().is_err());
        let mut too_few = recipients;
        too_few.touser.pop();
        assert!(too_few.validate().is_err());

        let preview = MassPreviewRequest {
            touser: Some("openid".to_string()),
            towxname: None,
            msgtype: "wxcard".to_string(),
            message: json!({ "wxcard": { "card_id": "card" } }),
        };
        assert!(preview.validate().is_ok());
        let mut ambiguous = preview;
        ambiguous.towxname = Some("account".to_string());
        assert!(ambiguous.validate().is_err());

        let success: MassSendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "msg_id": 1001
        }))
        .unwrap();
        assert!(success.validate().is_ok());
        let missing_id: MassSendResponse = serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_id.validate().is_err());
        let api_error: MassSendResponse = serde_json::from_value(json!({
            "errcode": 45065,
            "errmsg": "clientmsgid exist"
        }))
        .unwrap();
        assert!(matches!(api_error.validate(), Err(WechatError::Api { .. })));

        let status: MassStatusResponse = serde_json::from_value(json!({
            "errcode": 0,
            "msg_id": 1001,
            "msg_status": "SEND_SUCCESS"
        }))
        .unwrap();
        assert!(status.validate().is_ok());
        let missing_status: MassStatusResponse =
            serde_json::from_value(json!({ "errcode": 0, "msg_id": 1001 })).unwrap();
        assert!(missing_status.validate().is_err());
    }

    #[test]
    fn validates_menu_response_matrix() {
        let menu: MenuGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "menu": {
                "button": [{
                    "type": "view",
                    "name": "Docs",
                    "url": "https://example.com",
                    "key": ""
                }]
            },
            "conditionalmenu": []
        }))
        .unwrap();
        assert!(menu.validate().is_ok());

        let missing_menu: MenuGetResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_menu.validate().is_err());
        let api_error: MenuGetResponse = serde_json::from_value(json!({
            "errcode": 46003,
            "errmsg": "menu no exist"
        }))
        .unwrap();
        assert!(matches!(api_error.validate(), Err(WechatError::Api { .. })));

        let conditional: CreateConditionalMenuResponse =
            serde_json::from_value(json!({ "errcode": 0, "menuid": "208396938" })).unwrap();
        assert!(conditional.validate().is_ok());
        let missing_id: CreateConditionalMenuResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_id.validate().is_err());

        let current: CurrentSelfMenuResponse = serde_json::from_value(json!({
            "errcode": 0,
            "is_menu_open": 1,
            "selfmenu_info": {
                "button": [{
                    "type": "click",
                    "name": "Help",
                    "key": "help"
                }]
            }
        }))
        .unwrap();
        assert!(current.validate().is_ok());
        let inconsistent: CurrentSelfMenuResponse =
            serde_json::from_value(json!({ "errcode": 0, "is_menu_open": 1 })).unwrap();
        assert!(inconsistent.validate().is_err());

        let matched: MenuTryMatchResponse = serde_json::from_value(json!({
            "button": [{
                "type": "click",
                "name": "Member",
                "key": "member"
            }]
        }))
        .unwrap();
        assert!(matched.validate().is_ok());
    }

    #[test]
    fn validates_template_message_request_and_response_matrix() {
        let request = TemplateMessageRequest {
            touser: "openid".to_string(),
            template_id: "template".to_string(),
            url: Some("https://example.com/order/1".to_string()),
            miniprogram: Some(TemplateMiniProgram {
                appid: "wxmini".to_string(),
                pagepath: Some("pages/order?id=1".to_string()),
            }),
            data: json!({
                "first": { "value": "Order ready", "color": "#00AA11" }
            }),
            client_msg_id: Some("order-1".to_string()),
        };
        assert!(request.validate().is_ok());

        let mut invalid_url = request.clone();
        invalid_url.url = Some("javascript:alert(1)".to_string());
        assert!(invalid_url.validate().is_err());
        let mut invalid_data = request.clone();
        invalid_data.data = json!({ "first": "Order ready" });
        assert!(invalid_data.validate().is_err());
        let mut invalid_color = request;
        invalid_color.data = json!({ "first": { "value": "ready", "color": "green" } });
        assert!(invalid_color.validate().is_err());

        let subscribe = TemplateSubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "template".to_string(),
            url: "https://example.com".to_string(),
            miniprogram: Some(json!({
                "appid": "wxmini",
                "pagepath": "pages/index"
            })),
            scene: "1000".to_string(),
            title: "Status".to_string(),
            data: json!({ "status": { "value": "ready" } }),
        };
        assert!(subscribe.validate().is_ok());

        assert!(
            validate_template_keywords(&["keyword1".to_string(), "keyword2".to_string()]).is_ok()
        );
        assert!(
            validate_template_keywords(&["keyword1".to_string(), "keyword1".to_string()]).is_err()
        );

        let send: TemplateMessageSendResponse =
            serde_json::from_value(json!({ "errcode": 0, "msgid": 10001 })).unwrap();
        assert!(send.validate().is_ok());
        let missing_msgid: TemplateMessageSendResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_msgid.validate().is_err());

        let industry: TemplateIndustryResponse = serde_json::from_value(json!({
            "errcode": 0,
            "primary_industry": {
                "first_class": "IT",
                "second_class": "Internet"
            },
            "secondary_industry": {
                "first_class": "Commerce",
                "second_class": "Retail"
            }
        }))
        .unwrap();
        assert!(industry.validate().is_ok());

        let templates: PrivateTemplateListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "template_list": [
                { "template_id": "template-1" },
                { "template_id": "template-2" }
            ]
        }))
        .unwrap();
        assert!(templates.validate().is_ok());
        let duplicate: PrivateTemplateListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "template_list": [
                { "template_id": "template-1" },
                { "template_id": "template-1" }
            ]
        }))
        .unwrap();
        assert!(duplicate.validate().is_err());
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
