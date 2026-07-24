use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::{Result, WechatError},
    modules::{
        official_account::{
            validate_material_articles, validate_material_required, validate_material_upload,
            Article, MaterialGetResponse, MaterialListRequest, MaterialListResponse,
            MaterialMediaResponse, MaterialStatsResponse, MaterialUploadKind, OfficialAccount,
        },
        DomainModule, PlatformClient,
    },
    Client,
};

#[derive(Debug, Clone)]
pub struct OpenPlatform {
    inner: PlatformClient,
}

impl OpenPlatform {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn auth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.auth")
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.base")
    }

    pub async fn component_access_token(
        &self,
        request: ComponentAccessTokenRequest,
    ) -> Result<ComponentAccessTokenResponse> {
        self.inner
            .post("cgi-bin/component/api_component_token", None, request)
            .await
    }

    pub async fn create_preauth_code(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<PreauthCodeResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/component/api_create_preauthcode",
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
                json!({ "component_appid": component_appid.into() }),
                Vec::new(),
            )
            .await
    }

    pub async fn query_auth(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorization_code: impl Into<String>,
    ) -> Result<QueryAuthResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/component/api_query_auth",
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
                json!({
                    "component_appid": component_appid.into(),
                    "authorization_code": authorization_code.into(),
                }),
                Vec::new(),
            )
            .await
    }

    pub async fn authorizer_access_token(
        &self,
        component_access_token: impl Into<String>,
        request: AuthorizerAccessTokenRequest,
    ) -> Result<AuthorizerAccessTokenResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/component/api_authorizer_token",
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
                serde_json::to_value(request)?,
                Vec::new(),
            )
            .await
    }

    pub async fn handle_authorize(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorization_code: impl Into<String>,
    ) -> Result<OpenPlatformHandleAuthorizeResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_query_auth",
            json!({
                "component_appid": component_appid.into(),
                "authorization_code": authorization_code.into(),
            }),
        )
        .await
    }

    pub async fn get_base_authorizer(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerInfoResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_info",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
            }),
        )
        .await
    }

    pub async fn get_base_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerOptionResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_option",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
                "option_name": option_name.into(),
            }),
        )
        .await
    }

    pub async fn set_base_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
        option_value: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_set_authorizer_option",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
                "option_name": option_name.into(),
                "option_value": option_value.into(),
            }),
        )
        .await
    }

    pub async fn get_base_authorizers(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        offset: i64,
        count: i64,
    ) -> Result<OpenPlatformAuthorizersResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_list",
            json!({
                "component_appid": component_appid.into(),
                "offset": offset,
                "count": count,
            }),
        )
        .await
    }

    pub async fn create_pre_authorization_code(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<PreauthCodeResponse> {
        self.create_preauth_code(component_access_token, component_appid)
            .await
    }

    pub async fn clear_component_quota(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/clear_quota",
            json!({ "component_appid": component_appid.into() }),
        )
        .await
    }

    pub fn authorizer(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.authorizer")
    }

    pub fn authorizer_official_account(&self) -> OfficialAccount {
        OfficialAccount::new(self.inner.client(), Platform::OfficialAccount)
    }

    pub fn authorizer_mini_program_code(&self) -> DomainModule {
        DomainModule::new(
            self.inner.clone(),
            "open_platform.authorizer.mini_program.code",
        )
    }

    pub async fn commit_authorizer_mini_program_code(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramCommitRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        request.validate()?;
        self.inner
            .post("wxa/commit", Some(authorizer_access_token.into()), request)
            .await
    }

    pub async fn get_authorizer_mini_program_qrcode_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        path: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .get_bytes(
                "wxa/get_qrcode",
                Some(authorizer_access_token.into()),
                vec![("path".to_string(), path.into())],
            )
            .await
    }

    pub async fn get_authorizer_mini_program_category(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramCategoryResponse> {
        self.inner
            .get("wxa/get_category", Some(authorizer_access_token.into()))
            .await
    }

    pub async fn get_authorizer_mini_program_pages(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramPageResponse> {
        self.inner
            .get("wxa/get_page", Some(authorizer_access_token.into()))
            .await
    }

    pub async fn submit_authorizer_mini_program_audit(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramSubmitAuditRequest,
    ) -> Result<OpenPlatformMiniProgramSubmitAuditResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxa/submit_audit",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_authorizer_mini_program_audit_status(
        &self,
        authorizer_access_token: impl Into<String>,
        audit_id: i64,
    ) -> Result<OpenPlatformMiniProgramAuditStatusResponse> {
        validate_open_platform_positive("mini-program audit id", audit_id)?;
        self.inner
            .post(
                "wxa/get_auditstatus",
                Some(authorizer_access_token.into()),
                json!({ "auditid": audit_id }),
            )
            .await
    }

    pub async fn get_latest_authorizer_mini_program_audit_status(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramLatestAuditStatusResponse> {
        self.inner
            .get(
                "wxa/get_latest_auditstatus",
                Some(authorizer_access_token.into()),
            )
            .await
    }

    pub async fn release_authorizer_mini_program(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "wxa/release",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn withdraw_authorizer_mini_program_audit(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .get("wxa/undocodeaudit", Some(authorizer_access_token.into()))
            .await
    }

    pub async fn rollback_authorizer_mini_program_release(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramRollbackReleaseResponse> {
        self.inner
            .get(
                "wxa/revertcoderelease",
                Some(authorizer_access_token.into()),
            )
            .await
    }

    pub async fn change_authorizer_mini_program_visit_status(
        &self,
        authorizer_access_token: impl Into<String>,
        action: OpenPlatformMiniProgramVisitStatusAction,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "wxa/change_visitstatus",
                Some(authorizer_access_token.into()),
                json!({ "action": action }),
            )
            .await
    }

    pub async fn gray_release_authorizer_mini_program(
        &self,
        authorizer_access_token: impl Into<String>,
        gray_percentage: i64,
    ) -> Result<OpenPlatformStatusResponse> {
        if !(1..=100).contains(&gray_percentage) {
            return Err(WechatError::Config(
                "open platform mini-program gray percentage must be between 1 and 100".to_string(),
            ));
        }
        self.inner
            .post(
                "wxa/grayrelease",
                Some(authorizer_access_token.into()),
                json!({ "gray_percentage": gray_percentage }),
            )
            .await
    }

    pub async fn revert_authorizer_mini_program_gray_release(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .get(
                "wxa/revertgrayrelease",
                Some(authorizer_access_token.into()),
            )
            .await
    }

    pub async fn get_authorizer_mini_program_gray_release_plan(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramGrayReleasePlanResponse> {
        self.inner
            .get(
                "wxa/getgrayreleaseplan",
                Some(authorizer_access_token.into()),
            )
            .await
    }

    pub async fn get_authorizer_mini_program_support_version(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramSupportVersionResponse> {
        self.inner
            .post(
                "cgi-bin/wxopen/getweappsupportversion",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn set_authorizer_mini_program_support_version(
        &self,
        authorizer_access_token: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        let version = version.into();
        validate_open_platform_non_empty("mini-program support version", &version)?;
        self.inner
            .post(
                "cgi-bin/wxopen/setweappsupportversion",
                Some(authorizer_access_token.into()),
                json!({ "version": version }),
            )
            .await
    }

    pub async fn query_authorizer_mini_program_audit_quota(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramAuditQuotaResponse> {
        self.inner
            .get("wxa/queryquota", Some(authorizer_access_token.into()))
            .await
    }

    pub async fn speedup_authorizer_mini_program_audit(
        &self,
        authorizer_access_token: impl Into<String>,
        audit_id: i64,
    ) -> Result<OpenPlatformStatusResponse> {
        validate_open_platform_positive("mini-program audit id", audit_id)?;
        self.inner
            .post(
                "wxa/speedupaudit",
                Some(authorizer_access_token.into()),
                json!({ "auditid": audit_id }),
            )
            .await
    }

    pub fn authorizer_mini_program_domain(&self) -> DomainModule {
        DomainModule::new(
            self.inner.clone(),
            "open_platform.authorizer.mini_program.domain",
        )
    }

    pub async fn modify_authorizer_mini_program_domain(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramModifyDomainRequest,
    ) -> Result<OpenPlatformMiniProgramModifyDomainResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxa/modify_domain",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_authorizer_mini_program_webview_domain(
        &self,
        authorizer_access_token: impl Into<String>,
        action: OpenPlatformMiniProgramDomainAction,
        domains: Vec<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        validate_open_platform_domains("web-view", &domains, &["https"])?;
        validate_open_platform_domain_action(action, !domains.is_empty())?;
        self.inner
            .post(
                "wxa/setwebviewdomain",
                Some(authorizer_access_token.into()),
                json!({ "action": action, "webviewdomain": domains }),
            )
            .await
    }

    pub fn authorizer_mini_program_tester(&self) -> DomainModule {
        DomainModule::new(
            self.inner.clone(),
            "open_platform.authorizer.mini_program.tester",
        )
    }

    pub async fn bind_authorizer_mini_program_tester(
        &self,
        authorizer_access_token: impl Into<String>,
        wechat_id: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramTesterBindResponse> {
        let wechat_id = wechat_id.into();
        validate_open_platform_non_empty("tester WeChat ID", &wechat_id)?;
        self.inner
            .post(
                "wxa/bind_tester",
                Some(authorizer_access_token.into()),
                json!({ "wechatid": wechat_id }),
            )
            .await
    }

    pub async fn unbind_authorizer_mini_program_tester(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramTesterUnbindRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxa/unbind_tester",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_authorizer_mini_program_testers(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramTesterListResponse> {
        self.inner
            .post(
                "wxa/memberauth",
                Some(authorizer_access_token.into()),
                json!({ "action": "get_experiencer" }),
            )
            .await
    }

    pub fn authorizer_mini_program_privacy(&self) -> DomainModule {
        DomainModule::new(
            self.inner.clone(),
            "open_platform.authorizer.mini_program.privacy",
        )
    }

    pub async fn get_authorizer_mini_program_privacy_setting(
        &self,
        authorizer_access_token: impl Into<String>,
        privacy_version: i64,
    ) -> Result<OpenPlatformMiniProgramPrivacySettingResponse> {
        if privacy_version <= 0 {
            return Err(WechatError::Config(
                "open platform privacy version must be positive".to_string(),
            ));
        }
        self.inner
            .post(
                "cgi-bin/component/getprivacysetting",
                Some(authorizer_access_token.into()),
                json!({ "privacy_ver": privacy_version }),
            )
            .await
    }

    pub async fn set_authorizer_mini_program_privacy_setting(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramPrivacySettingRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/component/setprivacysetting",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
    }

    pub async fn upload_authorizer_mini_program_privacy_ext_file_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OpenPlatformMiniProgramPrivacyExtFileResponse> {
        let file_name = file_name.into();
        validate_open_platform_non_empty("privacy extension file name", &file_name)?;
        if data.is_empty() {
            return Err(WechatError::Config(
                "open platform privacy extension file must not be empty".to_string(),
            ));
        }
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(data).file_name(file_name),
        );
        self.inner
            .post_multipart(
                "cgi-bin/component/uploadprivacyextfile",
                Some(authorizer_access_token.into()),
                Vec::new(),
                form,
            )
            .await
    }

    pub fn component_login_page_url(request: OpenPlatformComponentLoginPageUrlRequest) -> String {
        let mut url = url::Url::parse("https://mp.weixin.qq.com/cgi-bin/componentloginpage")
            .expect("static component login page url is valid");
        url.query_pairs_mut()
            .append_pair("component_appid", &request.component_appid)
            .append_pair("pre_auth_code", &request.pre_auth_code)
            .append_pair("redirect_uri", &request.redirect_uri);
        for (key, value) in request.extra_query {
            url.query_pairs_mut().append_pair(&key, &value);
        }
        url.to_string()
    }

    pub async fn authorizer_mini_program_code_to_session(
        &self,
        component_access_token: impl Into<String>,
        appid: impl Into<String>,
        js_code: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramSessionResponse> {
        let appid = appid.into();
        let js_code = js_code.into();
        let component_appid = component_appid.into();
        validate_open_platform_appid("authorizer mini-program appid", &appid)?;
        validate_open_platform_identifier("mini-program login code", &js_code, 512)?;
        validate_open_platform_appid("component appid", &component_appid)?;
        let response: OpenPlatformMiniProgramSessionResponse = self
            .inner
            .get_with_query(
                "sns/component/jscode2session",
                None,
                vec![
                    ("appid".to_string(), appid),
                    ("js_code".to_string(), js_code),
                    ("grant_type".to_string(), "authorization_code".to_string()),
                    ("component_appid".to_string(), component_appid),
                    (
                        "component_access_token".to_string(),
                        component_access_token.into(),
                    ),
                ],
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub fn official_account_fast_registration_url(
        request: OpenPlatformOfficialAccountFastRegistrationUrlRequest,
    ) -> Result<String> {
        request.validate()?;
        let mut url = url::Url::parse("https://mp.weixin.qq.com/cgi-bin/fastregisterauth")
            .expect("static fast registration auth url is valid");
        url.query_pairs_mut()
            .append_pair(
                "copy_wx_verify",
                if request.copy_wx_verify {
                    "true"
                } else {
                    "false"
                },
            )
            .append_pair("component_appid", &request.component_appid)
            .append_pair("appid", &request.appid)
            .append_pair("redirect_uri", &request.redirect_uri);
        Ok(url.to_string())
    }

    pub async fn fast_register_authorizer_official_account_mini_program(
        &self,
        authorizer_access_token: impl Into<String>,
        ticket: impl Into<String>,
    ) -> Result<OpenPlatformOfficialAccountFastRegistrationResponse> {
        let ticket = ticket.into();
        validate_open_platform_identifier("official-account registration ticket", &ticket, 512)?;
        let response: OpenPlatformOfficialAccountFastRegistrationResponse = self
            .inner
            .post(
                "cgi-bin/account/fastregister",
                Some(authorizer_access_token.into()),
                json!({ "ticket": ticket }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_authorizer_account_basic_info(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerAccountBasicInfoResponse> {
        let response: OpenPlatformAuthorizerAccountBasicInfoResponse = self
            .inner
            .post(
                "cgi-bin/account/getaccountbasicinfo",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn modify_authorizer_account_head_image(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformAuthorizerAccountHeadImageRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        request.validate()?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/account/modifyheadimage",
                Some(authorizer_access_token.into()),
                request,
            )
            .await?;
        response.validate_for("modify authorizer account head image")?;
        Ok(response)
    }

    pub async fn modify_authorizer_account_signature(
        &self,
        authorizer_access_token: impl Into<String>,
        signature: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        let signature = signature.into();
        validate_open_platform_text("authorizer account signature", &signature, 120)?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/account/modifysignature",
                Some(authorizer_access_token.into()),
                json!({ "signature": signature }),
            )
            .await?;
        response.validate_for("modify authorizer account signature")?;
        Ok(response)
    }

    pub async fn get_authorizer_material_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        let media_id = media_id.into();
        validate_material_required("media id", &media_id)?;
        self.inner
            .post_json_bytes(
                "cgi-bin/material/get_material",
                Some(authorizer_access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await
    }

    pub async fn upload_authorizer_material_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
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
                Some(authorizer_access_token.into()),
                vec![("type".to_string(), kind.as_code().to_string())],
                form,
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn upload_authorizer_image_material_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_authorizer_material_from_bytes(
            authorizer_access_token,
            MaterialUploadKind::Image,
            file_name,
            data,
        )
        .await
    }

    pub async fn upload_authorizer_voice_material_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_authorizer_material_from_bytes(
            authorizer_access_token,
            MaterialUploadKind::Voice,
            file_name,
            data,
        )
        .await
    }

    pub async fn upload_authorizer_thumb_material_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MaterialMediaResponse> {
        self.upload_authorizer_material_from_bytes(
            authorizer_access_token,
            MaterialUploadKind::Thumb,
            file_name,
            data,
        )
        .await
    }

    pub async fn upload_authorizer_video_material_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
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
                Some(authorizer_access_token.into()),
                vec![("type".to_string(), "video".to_string())],
                form,
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn upload_authorizer_article_image_from_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
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
                Some(authorizer_access_token.into()),
                Vec::new(),
                form,
            )
            .await?;
        response.require_url()?;
        Ok(response)
    }

    pub async fn add_authorizer_news_material(
        &self,
        authorizer_access_token: impl Into<String>,
        articles: Vec<Article>,
    ) -> Result<MaterialMediaResponse> {
        validate_material_articles(&articles)?;
        let response: MaterialMediaResponse = self
            .inner
            .post(
                "cgi-bin/material/add_news",
                Some(authorizer_access_token.into()),
                json!({ "articles": articles }),
            )
            .await?;
        response.require_media_id()?;
        Ok(response)
    }

    pub async fn update_authorizer_news_material(
        &self,
        authorizer_access_token: impl Into<String>,
        media_id: impl Into<String>,
        index: i64,
        article: Article,
    ) -> Result<OpenPlatformStatusResponse> {
        let media_id = media_id.into();
        validate_material_required("media id", &media_id)?;
        if !(0..=7).contains(&index) {
            return Err(WechatError::Config(
                "open platform material article index must be between 0 and 7".to_string(),
            ));
        }
        article.validate()?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/material/update_news",
                Some(authorizer_access_token.into()),
                json!({
                    "media_id": media_id,
                    "index": index,
                    "articles": article,
                }),
            )
            .await?;
        response.validate_for("update authorizer news material")?;
        Ok(response)
    }

    pub async fn get_authorizer_material(
        &self,
        authorizer_access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<MaterialGetResponse> {
        let media_id = media_id.into();
        validate_material_required("media id", &media_id)?;
        let response: MaterialGetResponse = self
            .inner
            .post(
                "cgi-bin/material/get_material",
                Some(authorizer_access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn delete_authorizer_material(
        &self,
        authorizer_access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        let media_id = media_id.into();
        validate_material_required("media id", &media_id)?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/material/del_material",
                Some(authorizer_access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await?;
        response.validate_for("delete authorizer material")?;
        Ok(response)
    }

    pub async fn list_authorizer_materials(
        &self,
        authorizer_access_token: impl Into<String>,
        request: MaterialListRequest,
    ) -> Result<MaterialListResponse> {
        request.validate()?;
        let response: MaterialListResponse = self
            .inner
            .post(
                "cgi-bin/material/batchget_material",
                Some(authorizer_access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_authorizer_material_stats(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<MaterialStatsResponse> {
        let response: MaterialStatsResponse = self
            .inner
            .post(
                "cgi-bin/material/get_materialcount",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn create_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
    ) -> Result<OpenPlatformOpenAccountResponse> {
        let appid = appid.into();
        validate_open_platform_appid("authorizer appid", &appid)?;
        let response: OpenPlatformOpenAccountResponse = self
            .inner
            .post(
                "cgi-bin/open/create",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn bind_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
        open_appid: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        let appid = appid.into();
        let open_appid = open_appid.into();
        validate_open_platform_appid("authorizer appid", &appid)?;
        validate_open_platform_appid("open account appid", &open_appid)?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/open/bind",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid, "open_appid": open_appid }),
            )
            .await?;
        response.validate_for("bind authorizer open account")?;
        Ok(response)
    }

    pub async fn unbind_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
        open_appid: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        let appid = appid.into();
        let open_appid = open_appid.into();
        validate_open_platform_appid("authorizer appid", &appid)?;
        validate_open_platform_appid("open account appid", &open_appid)?;
        let response: OpenPlatformStatusResponse = self
            .inner
            .post(
                "cgi-bin/open/unbind",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid, "open_appid": open_appid }),
            )
            .await?;
        response.validate_for("unbind authorizer open account")?;
        Ok(response)
    }

    pub async fn get_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
    ) -> Result<OpenPlatformOpenAccountResponse> {
        let appid = appid.into();
        validate_open_platform_appid("authorizer appid", &appid)?;
        let response: OpenPlatformOpenAccountResponse = self
            .inner
            .post(
                "cgi-bin/open/get",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_authorizer_mini_program_privacy_interface(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramPrivacyInterfaceResponse> {
        let response: OpenPlatformMiniProgramPrivacyInterfaceResponse = self
            .inner
            .post(
                "wxa/security/get_privacy_interface",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn apply_authorizer_mini_program_privacy_interface(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramPrivacyInterfaceApplyRequest,
    ) -> Result<OpenPlatformMiniProgramPrivacyInterfaceApplyResponse> {
        request.validate()?;
        let response: OpenPlatformMiniProgramPrivacyInterfaceApplyResponse = self
            .inner
            .post(
                "wxa/security/apply_privacy_interface",
                Some(authorizer_access_token.into()),
                request,
            )
            .await?;
        response.require_audit_id()?;
        Ok(response)
    }

    pub async fn get_authorizer_info(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerInfoResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_info",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
            }),
        )
        .await
    }

    pub async fn get_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerOptionResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_option",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
                "option_name": option_name.into(),
            }),
        )
        .await
    }

    pub async fn set_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
        option_value: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_set_authorizer_option",
            json!({
                "component_appid": component_appid.into(),
                "authorizer_appid": authorizer_appid.into(),
                "option_name": option_name.into(),
                "option_value": option_value.into(),
            }),
        )
        .await
    }

    pub async fn list_authorizers(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        offset: i64,
        count: i64,
    ) -> Result<OpenPlatformAuthorizersResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_get_authorizer_list",
            json!({
                "component_appid": component_appid.into(),
                "offset": offset,
                "count": count,
            }),
        )
        .await
    }

    pub fn code_template(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.code_template")
    }

    pub async fn template_drafts(
        &self,
        component_access_token: impl Into<String>,
    ) -> Result<OpenPlatformTemplateDraftListResponse> {
        self.inner
            .get_with_query(
                "wxa/gettemplatedraftlist",
                None,
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
            )
            .await
    }

    pub async fn add_template_from_draft(
        &self,
        component_access_token: impl Into<String>,
        draft_id: i64,
        template_type: i64,
    ) -> Result<OpenPlatformStatusResponse> {
        self.add_template(
            component_access_token,
            AddTemplateFromDraftRequest {
                draft_id,
                template_type,
            },
        )
        .await
    }

    pub async fn add_template(
        &self,
        component_access_token: impl Into<String>,
        request: AddTemplateFromDraftRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        self.post_component_json(
            component_access_token,
            "wxa/addtotemplate",
            serde_json::to_value(request)?,
        )
        .await
    }

    pub async fn templates(
        &self,
        component_access_token: impl Into<String>,
    ) -> Result<OpenPlatformTemplateListResponse> {
        self.inner
            .get_with_query(
                "wxa/gettemplatelist",
                None,
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
            )
            .await
    }

    pub async fn delete_template(
        &self,
        component_access_token: impl Into<String>,
        template_id: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.remove_template(
            component_access_token,
            DeleteTemplateRequest {
                template_id: template_id.into(),
            },
        )
        .await
    }

    pub async fn remove_template(
        &self,
        component_access_token: impl Into<String>,
        request: DeleteTemplateRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        self.post_component_json(
            component_access_token,
            "wxa/deletetemplate",
            serde_json::to_value(request)?,
        )
        .await
    }

    pub fn component(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.component")
    }

    pub async fn register_mini_program(
        &self,
        component_access_token: impl Into<String>,
        request: RegisterMiniProgramRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/component/fastregisterweapp",
                vec![
                    (
                        "component_access_token".to_string(),
                        component_access_token.into(),
                    ),
                    ("action".to_string(), "create".to_string()),
                ],
                serde_json::to_value(request)?,
                Vec::new(),
            )
            .await
    }

    pub async fn get_registration_status(
        &self,
        component_access_token: impl Into<String>,
        request: RegistrationStatusRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/component/fastregisterweapp",
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
                serde_json::to_value(request)?,
                Vec::new(),
            )
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_platform.server")
    }

    async fn post_component_json<R>(
        &self,
        component_access_token: impl Into<String>,
        path: &str,
        body: Value,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        self.inner
            .post_json_with_query(
                path,
                vec![(
                    "component_access_token".to_string(),
                    component_access_token.into(),
                )],
                body,
                Vec::new(),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAccessTokenRequest {
    pub component_appid: String,
    pub component_appsecret: String,
    pub component_verify_ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub component_access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreauthCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub pre_auth_code: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<OpenPlatformAuthorizationInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerAccessTokenRequest {
    pub component_appid: String,
    pub authorizer_appid: String,
    pub authorizer_refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorizer_access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub authorizer_refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformFuncScopeCategory {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformConfirmInfo {
    #[serde(default)]
    pub need_confirm: Option<i64>,
    #[serde(default)]
    pub already_confirm: Option<i64>,
    #[serde(default)]
    pub can_confirm: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformFuncInfo {
    #[serde(default)]
    pub funcscope_category: Option<OpenPlatformFuncScopeCategory>,
    #[serde(default)]
    pub confirm_info: Option<OpenPlatformConfirmInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizationInfo {
    #[serde(default)]
    pub authorizer_appid: Option<String>,
    #[serde(default)]
    pub authorizer_access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub authorizer_refresh_token: Option<String>,
    #[serde(default)]
    pub func_info: Vec<OpenPlatformFuncInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformHandleAuthorizeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<OpenPlatformAuthorizationInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformServiceTypeInfo {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformVerifyTypeInfo {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformBusinessInfo {
    #[serde(default)]
    pub open_store: Option<i64>,
    #[serde(default)]
    pub open_scan: Option<i64>,
    #[serde(default)]
    pub open_pay: Option<i64>,
    #[serde(default)]
    pub open_card: Option<i64>,
    #[serde(default)]
    pub open_shake: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformBasicConfig {
    #[serde(default)]
    pub is_phone_configured: Option<bool>,
    #[serde(default)]
    pub is_email_configured: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramNetwork {
    #[serde(default, rename = "RequestDomain")]
    pub request_domain: Vec<String>,
    #[serde(default, rename = "WsRequestDomain")]
    pub ws_request_domain: Vec<String>,
    #[serde(default, rename = "UploadDomain")]
    pub upload_domain: Vec<String>,
    #[serde(default, rename = "DownloadDomain")]
    pub download_domain: Vec<String>,
    #[serde(default, rename = "BizDomain")]
    pub biz_domain: Vec<String>,
    #[serde(default, rename = "UDPDomain")]
    pub udp_domain: Vec<String>,
    #[serde(default, rename = "TCPDomain")]
    pub tcp_domain: Vec<String>,
    #[serde(default, rename = "PrefetchDNSDomain")]
    pub prefetch_dns_domain: Vec<String>,
    #[serde(default, rename = "NewRequestDomain")]
    pub new_request_domain: Vec<String>,
    #[serde(default, rename = "NewWsRequestDomain")]
    pub new_ws_request_domain: Vec<String>,
    #[serde(default, rename = "NewUploadDomain")]
    pub new_upload_domain: Vec<String>,
    #[serde(default, rename = "NewDownloadDomain")]
    pub new_download_domain: Vec<String>,
    #[serde(default, rename = "NewBizDomain")]
    pub new_biz_domain: Vec<String>,
    #[serde(default, rename = "NewUDPDomain")]
    pub new_udp_domain: Vec<String>,
    #[serde(default, rename = "NewTCPDomain")]
    pub new_tcp_domain: Vec<String>,
    #[serde(default, rename = "NewPrefetchDNSDomain")]
    pub new_prefetch_dns_domain: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerMiniProgramCategory {
    #[serde(default)]
    pub first: Option<String>,
    #[serde(default)]
    pub second: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerMiniProgramInfo {
    #[serde(default)]
    pub network: Option<OpenPlatformMiniProgramNetwork>,
    #[serde(default)]
    pub categories: Vec<OpenPlatformAuthorizerMiniProgramCategory>,
    #[serde(default)]
    pub visit_status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerInfo {
    #[serde(default)]
    pub nick_name: Option<String>,
    #[serde(default)]
    pub head_img: Option<String>,
    #[serde(default)]
    pub service_type_info: Option<OpenPlatformServiceTypeInfo>,
    #[serde(default)]
    pub verify_type_info: Option<OpenPlatformVerifyTypeInfo>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub principal_name: Option<String>,
    #[serde(default)]
    pub business_info: Option<OpenPlatformBusinessInfo>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub qrcode_url: Option<String>,
    #[serde(default)]
    pub account_status: Option<i64>,
    #[serde(default)]
    pub idc: Option<i64>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default, rename = "MiniProgramInfo", alias = "mini_program_info")]
    pub mini_program_info: Option<OpenPlatformAuthorizerMiniProgramInfo>,
    #[serde(default)]
    pub register_type: Option<i64>,
    #[serde(default)]
    pub basic_config: Option<OpenPlatformBasicConfig>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorizer_info: Option<OpenPlatformAuthorizerInfo>,
    #[serde(default)]
    pub authorization_info: Option<OpenPlatformAuthorizationInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizationSummary {
    #[serde(default)]
    pub authorizer_appid: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub auth_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub list: Vec<OpenPlatformAuthorizationSummary>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerOptionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorizer_appid: Option<String>,
    #[serde(default)]
    pub option_name: Option<String>,
    #[serde(default)]
    pub option_value: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramCommitRequest {
    pub template_id: String,
    pub ext_json: String,
    pub user_version: String,
    pub user_desc: String,
}

impl OpenPlatformMiniProgramCommitRequest {
    pub fn new(
        template_id: impl Into<String>,
        ext_json: impl Into<String>,
        user_version: impl Into<String>,
        user_desc: impl Into<String>,
    ) -> Self {
        Self {
            template_id: template_id.into(),
            ext_json: ext_json.into(),
            user_version: user_version.into(),
            user_desc: user_desc.into(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_open_platform_non_empty("mini-program template id", &self.template_id)?;
        validate_open_platform_non_empty("mini-program user version", &self.user_version)?;
        validate_open_platform_non_empty("mini-program user description", &self.user_desc)?;
        let ext_json: Value = serde_json::from_str(&self.ext_json).map_err(|error| {
            WechatError::Config(format!(
                "open platform mini-program ext_json is invalid JSON: {error}"
            ))
        })?;
        if !ext_json.is_object() {
            return Err(WechatError::Config(
                "open platform mini-program ext_json must be a JSON object".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramAuditItem {
    pub address: String,
    pub tag: String,
    pub first_class: String,
    pub second_class: String,
    pub first_id: i64,
    pub second_id: i64,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub third_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub third_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_stuff: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramAuditPreviewInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pic_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub video_id_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramUgcDeclare {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scene: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_audit_team: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_desc: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramSubmitAuditRequest {
    pub item_list: Vec<OpenPlatformMiniProgramAuditItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_stuff: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_info: Option<OpenPlatformMiniProgramAuditPreviewInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ugc_declare: Option<OpenPlatformMiniProgramUgcDeclare>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_api_not_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_path: Option<String>,
}

impl OpenPlatformMiniProgramSubmitAuditRequest {
    pub fn new(item_list: Vec<OpenPlatformMiniProgramAuditItem>) -> Self {
        Self {
            item_list,
            feedback_info: None,
            feedback_stuff: None,
            preview_info: None,
            version_desc: None,
            ugc_declare: None,
            privacy_api_not_use: None,
            order_path: None,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.item_list.is_empty() {
            return Err(WechatError::Config(
                "open platform mini-program audit item list must not be empty".to_string(),
            ));
        }
        let mut addresses = std::collections::HashSet::with_capacity(self.item_list.len());
        for item in &self.item_list {
            item.validate()?;
            if !addresses.insert(item.address.trim()) {
                return Err(WechatError::Config(
                    "open platform mini-program audit page addresses must be unique".to_string(),
                ));
            }
        }
        validate_open_platform_optional_non_empty(
            "mini-program audit feedback information",
            self.feedback_info.as_deref(),
        )?;
        validate_open_platform_optional_non_empty(
            "mini-program audit feedback material",
            self.feedback_stuff.as_deref(),
        )?;
        validate_open_platform_optional_non_empty(
            "mini-program audit version description",
            self.version_desc.as_deref(),
        )?;
        validate_open_platform_optional_non_empty(
            "mini-program audit order path",
            self.order_path.as_deref(),
        )?;
        if let Some(preview) = &self.preview_info {
            preview.validate()?;
        }
        if let Some(ugc) = &self.ugc_declare {
            ugc.validate()?;
        }
        Ok(())
    }
}

impl OpenPlatformMiniProgramAuditItem {
    fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("audit page address", self.address.as_str()),
            ("audit tag", self.tag.as_str()),
            ("audit first category", self.first_class.as_str()),
            ("audit second category", self.second_class.as_str()),
            ("audit page title", self.title.as_str()),
        ] {
            validate_open_platform_non_empty(kind, value)?;
        }
        validate_open_platform_positive("audit first category id", self.first_id)?;
        validate_open_platform_positive("audit second category id", self.second_id)?;
        match (&self.third_class, self.third_id) {
            (None, None) => {}
            (Some(third_class), Some(third_id)) => {
                validate_open_platform_non_empty("audit third category", third_class)?;
                validate_open_platform_positive("audit third category id", third_id)?;
            }
            _ => {
                return Err(WechatError::Config(
                    "open platform mini-program audit third category name and id must be supplied together"
                        .to_string(),
                ));
            }
        }
        validate_open_platform_optional_non_empty(
            "audit item feedback information",
            self.feedback_info.as_deref(),
        )?;
        validate_open_platform_optional_non_empty(
            "audit item feedback material",
            self.feedback_stuff.as_deref(),
        )
    }
}

impl OpenPlatformMiniProgramAuditPreviewInfo {
    fn validate(&self) -> Result<()> {
        validate_open_platform_unique_values("audit preview picture id", &self.pic_id_list)?;
        validate_open_platform_unique_values("audit preview video id", &self.video_id_list)
    }
}

impl OpenPlatformMiniProgramUgcDeclare {
    fn validate(&self) -> Result<()> {
        validate_open_platform_unique_positive_values("UGC scene", &self.scene)?;
        validate_open_platform_unique_positive_values("UGC method", &self.method)?;
        if self
            .has_audit_team
            .is_some_and(|value| !matches!(value, 0 | 1))
        {
            return Err(WechatError::Config(
                "open platform mini-program UGC audit-team flag must be 0 or 1".to_string(),
            ));
        }
        validate_open_platform_optional_non_empty(
            "UGC audit description",
            self.audit_desc.as_deref(),
        )?;
        if self.has_audit_team == Some(1)
            && self
                .audit_desc
                .as_deref()
                .is_none_or(|description| description.trim().is_empty())
        {
            return Err(WechatError::Config(
                "open platform mini-program UGC audit description is required when an audit team exists"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramCategory {
    #[serde(default)]
    pub first_class: Option<String>,
    #[serde(default)]
    pub second_class: Option<String>,
    #[serde(default)]
    pub third_class: Option<String>,
    #[serde(default)]
    pub first_id: Option<i64>,
    #[serde(default)]
    pub second_id: Option<i64>,
    #[serde(default)]
    pub third_id: Option<i64>,
    #[serde(default)]
    pub audit_status: Option<i64>,
    #[serde(default)]
    pub audit_reason: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlatformMiniProgramCategoryAuditState {
    Auditing,
    Rejected,
    Approved,
    Other,
}

impl OpenPlatformMiniProgramCategory {
    pub fn audit_state(&self) -> Option<OpenPlatformMiniProgramCategoryAuditState> {
        self.audit_status.map(|status| match status {
            1 => OpenPlatformMiniProgramCategoryAuditState::Auditing,
            2 => OpenPlatformMiniProgramCategoryAuditState::Rejected,
            3 => OpenPlatformMiniProgramCategoryAuditState::Approved,
            _ => OpenPlatformMiniProgramCategoryAuditState::Other,
        })
    }

    pub fn is_audit_approved(&self) -> bool {
        self.audit_state() == Some(OpenPlatformMiniProgramCategoryAuditState::Approved)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramCategoryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub category_list: Vec<OpenPlatformMiniProgramCategory>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub page_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramSubmitAuditResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "type")]
    pub audit_type: Option<String>,
    #[serde(default)]
    pub mediaid: Option<String>,
    #[serde(default)]
    pub auditid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramSubmitAuditResponse {
    pub fn require_audit_id(&self) -> Result<i64> {
        require_open_platform_positive_response_id("mini-program submitted audit id", self.auditid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramAuditStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub screenshot: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlatformMiniProgramAuditState {
    Approved,
    Rejected,
    Auditing,
    Withdrawn,
    Other,
}

impl OpenPlatformMiniProgramAuditState {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Approved | Self::Rejected | Self::Withdrawn)
    }

    pub fn can_release(self) -> bool {
        matches!(self, Self::Approved)
    }

    pub fn needs_attention(self) -> bool {
        matches!(self, Self::Rejected | Self::Other)
    }
}

impl OpenPlatformMiniProgramAuditStatusResponse {
    pub fn audit_state(&self) -> Option<OpenPlatformMiniProgramAuditState> {
        mini_program_audit_state(self.status)
    }

    pub fn is_audit_approved(&self) -> bool {
        self.audit_state() == Some(OpenPlatformMiniProgramAuditState::Approved)
    }

    pub fn is_audit_rejected(&self) -> bool {
        self.audit_state() == Some(OpenPlatformMiniProgramAuditState::Rejected)
    }

    pub fn rejection_reason(&self) -> Option<&str> {
        self.is_audit_rejected()
            .then_some(self.reason.as_deref())
            .flatten()
            .filter(|reason| !reason.trim().is_empty())
    }

    pub fn ensure_releasable(&self) -> Result<()> {
        validate_open_platform_audit_result(self.status, self.reason.as_deref())?;
        if !self
            .audit_state()
            .is_some_and(OpenPlatformMiniProgramAuditState::can_release)
        {
            return Err(WechatError::Config(format!(
                "open platform mini-program audit is not releasable in state {:?}",
                self.audit_state()
            )));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        validate_open_platform_audit_result(self.status, self.reason.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramLatestAuditStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub auditid: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default, alias = "ScreenShot")]
    pub screenshot: Option<String>,
    #[serde(default)]
    pub user_version: Option<String>,
    #[serde(default)]
    pub user_desc: Option<String>,
    #[serde(default)]
    pub submit_audit_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramLatestAuditStatusResponse {
    pub fn audit_state(&self) -> Option<OpenPlatformMiniProgramAuditState> {
        mini_program_audit_state(self.status)
    }

    pub fn is_audit_approved(&self) -> bool {
        self.audit_state() == Some(OpenPlatformMiniProgramAuditState::Approved)
    }

    pub fn is_audit_rejected(&self) -> bool {
        self.audit_state() == Some(OpenPlatformMiniProgramAuditState::Rejected)
    }

    pub fn require_audit_id(&self) -> Result<i64> {
        require_open_platform_positive_response_id("mini-program latest audit id", self.auditid)
    }

    pub fn rejection_reason(&self) -> Option<&str> {
        self.is_audit_rejected()
            .then_some(self.reason.as_deref())
            .flatten()
            .filter(|reason| !reason.trim().is_empty())
    }

    pub fn ensure_releasable(&self) -> Result<i64> {
        self.validate()?;
        if !self
            .audit_state()
            .is_some_and(OpenPlatformMiniProgramAuditState::can_release)
        {
            return Err(WechatError::Config(format!(
                "open platform mini-program latest audit is not releasable in state {:?}",
                self.audit_state()
            )));
        }
        self.require_audit_id()
    }

    pub fn validate(&self) -> Result<()> {
        validate_open_platform_audit_result(self.status, self.reason.as_deref())?;
        if self.auditid.is_some() {
            self.require_audit_id()?;
        }
        if self.submit_audit_time.is_some_and(|time| time <= 0) {
            return Err(WechatError::Config(
                "open platform mini-program submit audit time must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

fn require_open_platform_positive_response_id(kind: &str, value: Option<i64>) -> Result<i64> {
    let value =
        value.ok_or_else(|| WechatError::Config(format!("open platform {kind} is missing")))?;
    validate_open_platform_positive(kind, value)?;
    Ok(value)
}

fn validate_open_platform_audit_result(status: Option<i64>, reason: Option<&str>) -> Result<()> {
    let state = mini_program_audit_state(status).ok_or_else(|| {
        WechatError::Config("open platform mini-program audit status is missing".to_string())
    })?;
    if state == OpenPlatformMiniProgramAuditState::Rejected
        && reason.is_none_or(|reason| reason.trim().is_empty())
    {
        return Err(WechatError::Config(
            "open platform mini-program rejected audit reason is missing".to_string(),
        ));
    }
    Ok(())
}

fn mini_program_audit_state(status: Option<i64>) -> Option<OpenPlatformMiniProgramAuditState> {
    status.map(|status| match status {
        0 => OpenPlatformMiniProgramAuditState::Approved,
        1 => OpenPlatformMiniProgramAuditState::Rejected,
        2 => OpenPlatformMiniProgramAuditState::Auditing,
        3 => OpenPlatformMiniProgramAuditState::Withdrawn,
        _ => OpenPlatformMiniProgramAuditState::Other,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramReleaseVersion {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub percentage: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub commit_time: Option<i64>,
    #[serde(default)]
    pub user_version: Option<String>,
    #[serde(default)]
    pub user_desc: Option<String>,
    #[serde(default)]
    pub app_version: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramReleaseVersion {
    pub fn release_label(&self) -> Option<&str> {
        self.user_version
            .as_deref()
            .or(self.version.as_deref())
            .filter(|version| !version.trim().is_empty())
    }

    pub fn is_full_release(&self) -> bool {
        self.percentage == Some(100)
    }

    pub fn validate(&self) -> Result<()> {
        if self
            .percentage
            .is_some_and(|value| !(0..=100).contains(&value))
        {
            return Err(WechatError::Config(
                "open platform mini-program release percentage must be between 0 and 100"
                    .to_string(),
            ));
        }
        for (kind, value) in [
            ("release create time", self.create_time),
            ("release commit time", self.commit_time),
            ("release app version", self.app_version),
        ] {
            if value.is_some_and(|value| value < 0) {
                return Err(WechatError::Config(format!(
                    "open platform mini-program {kind} cannot be negative"
                )));
            }
        }
        if self.version.is_some() || self.user_version.is_some() {
            self.release_label().ok_or_else(|| {
                WechatError::Config(
                    "open platform mini-program release version cannot be blank".to_string(),
                )
            })?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramRollbackReleaseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub version_list: Vec<OpenPlatformMiniProgramReleaseVersion>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramRollbackReleaseResponse {
    pub fn validate(&self) -> Result<()> {
        let mut app_versions = std::collections::HashSet::new();
        for version in &self.version_list {
            version.validate()?;
            if let Some(app_version) = version.app_version {
                if !app_versions.insert(app_version) {
                    return Err(WechatError::Config(format!(
                        "open platform mini-program rollback app version {app_version} is duplicated"
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn latest(&self) -> Option<&OpenPlatformMiniProgramReleaseVersion> {
        self.version_list.iter().max_by_key(|version| {
            version
                .commit_time
                .or(version.create_time)
                .unwrap_or_default()
        })
    }

    pub fn find_app_version(
        &self,
        app_version: i64,
    ) -> Option<&OpenPlatformMiniProgramReleaseVersion> {
        self.version_list
            .iter()
            .find(|version| version.app_version == Some(app_version))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramGrayReleasePlan {
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub create_timestamp: Option<i64>,
    #[serde(default)]
    pub gray_percentage: Option<i64>,
    #[serde(default)]
    pub support_experiencer_first: Option<bool>,
    #[serde(default)]
    pub support_debuger_first: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlatformMiniProgramGrayReleaseState {
    Initial,
    Running,
    Paused,
    Finished,
    Deleted,
    Other,
}

impl From<i64> for OpenPlatformMiniProgramGrayReleaseState {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Initial,
            1 => Self::Running,
            2 => Self::Paused,
            3 => Self::Finished,
            4 => Self::Deleted,
            _ => Self::Other,
        }
    }
}

impl OpenPlatformMiniProgramGrayReleaseState {
    pub fn is_active(self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Finished | Self::Deleted)
    }
}

impl OpenPlatformMiniProgramGrayReleasePlan {
    pub fn release_state(&self) -> Option<OpenPlatformMiniProgramGrayReleaseState> {
        self.status
            .map(OpenPlatformMiniProgramGrayReleaseState::from)
    }

    pub fn validate(&self) -> Result<()> {
        if self.create_timestamp.is_some_and(|value| value < 0) {
            return Err(WechatError::Config(
                "open platform mini-program gray release timestamp cannot be negative".to_string(),
            ));
        }
        if self
            .gray_percentage
            .is_some_and(|value| !(0..=100).contains(&value))
        {
            return Err(WechatError::Config(
                "open platform mini-program gray percentage must be between 0 and 100".to_string(),
            ));
        }
        if self.release_state() == Some(OpenPlatformMiniProgramGrayReleaseState::Running)
            && !self
                .gray_percentage
                .is_some_and(|percentage| (1..=100).contains(&percentage))
        {
            return Err(WechatError::Config(
                "open platform running gray release requires a percentage between 1 and 100"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramGrayReleasePlanResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub gray_release_plan: Option<OpenPlatformMiniProgramGrayReleasePlan>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramGrayReleasePlanResponse {
    pub fn validate(&self) -> Result<()> {
        if let Some(plan) = &self.gray_release_plan {
            plan.validate()?;
        }
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.gray_release_plan
            .as_ref()
            .and_then(OpenPlatformMiniProgramGrayReleasePlan::release_state)
            .is_some_and(OpenPlatformMiniProgramGrayReleaseState::is_active)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramUvInfo {
    #[serde(default)]
    pub items: Vec<OpenPlatformMiniProgramUvInfoItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramUvInfoItem {
    #[serde(default)]
    pub ref_date: Option<String>,
    #[serde(default)]
    pub visit_uv: Option<i64>,
    #[serde(default)]
    pub percentage: Option<i64>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenPlatformMiniProgramVisitStatusAction {
    Open,
    Close,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramSupportVersionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub now_version: Option<String>,
    #[serde(default)]
    pub uv_info: Option<OpenPlatformMiniProgramUvInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramSupportVersionResponse {
    pub fn validate(&self) -> Result<()> {
        if self
            .now_version
            .as_deref()
            .is_some_and(|version| version.trim().is_empty())
        {
            return Err(WechatError::Config(
                "open platform mini-program support version cannot be blank".to_string(),
            ));
        }
        if let Some(uv_info) = &self.uv_info {
            for item in &uv_info.items {
                if item.visit_uv.is_some_and(|value| value < 0) {
                    return Err(WechatError::Config(
                        "open platform mini-program support-version visit UV cannot be negative"
                            .to_string(),
                    ));
                }
                if item
                    .percentage
                    .is_some_and(|value| !(0..=100).contains(&value))
                {
                    return Err(WechatError::Config(
                        "open platform mini-program support-version percentage must be between 0 and 100"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn current_version(&self) -> Option<&str> {
        self.now_version
            .as_deref()
            .filter(|version| !version.trim().is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramAuditQuotaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub rest: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub speedup_rest: Option<i64>,
    #[serde(default)]
    pub speedup_limit: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramAuditQuotaResponse {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_quota_pair("audit", self.rest, self.limit)?;
        validate_open_platform_quota_pair("speedup audit", self.speedup_rest, self.speedup_limit)
    }

    pub fn can_submit(&self) -> bool {
        self.rest.is_some_and(|remaining| remaining > 0)
    }

    pub fn can_speedup(&self) -> bool {
        self.speedup_rest.is_some_and(|remaining| remaining > 0)
    }

    pub fn used(&self) -> Option<i64> {
        self.limit
            .zip(self.rest)
            .and_then(|(limit, remaining)| limit.checked_sub(remaining))
    }

    pub fn speedup_used(&self) -> Option<i64> {
        self.speedup_limit
            .zip(self.speedup_rest)
            .and_then(|(limit, remaining)| limit.checked_sub(remaining))
    }
}

fn validate_open_platform_quota_pair(
    kind: &str,
    remaining: Option<i64>,
    limit: Option<i64>,
) -> Result<()> {
    if remaining.is_some_and(|value| value < 0) || limit.is_some_and(|value| value < 0) {
        return Err(WechatError::Config(format!(
            "open platform mini-program {kind} quota cannot be negative"
        )));
    }
    if remaining
        .zip(limit)
        .is_some_and(|(rest, limit)| rest > limit)
    {
        return Err(WechatError::Config(format!(
            "open platform mini-program remaining {kind} quota cannot exceed its limit"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenPlatformMiniProgramDomainAction {
    Get,
    Set,
    Add,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramModifyDomainRequest {
    pub action: OpenPlatformMiniProgramDomainAction,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requestdomain: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wsrequestdomain: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uploaddomain: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downloaddomain: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udpdomain: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tcpdomain: Vec<String>,
}

impl OpenPlatformMiniProgramModifyDomainRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_domains("request", &self.requestdomain, &["https"])?;
        validate_open_platform_domains("WebSocket request", &self.wsrequestdomain, &["wss"])?;
        validate_open_platform_domains("upload", &self.uploaddomain, &["https"])?;
        validate_open_platform_domains("download", &self.downloaddomain, &["https"])?;
        validate_open_platform_domains("UDP", &self.udpdomain, &["udp"])?;
        validate_open_platform_domains("TCP", &self.tcpdomain, &["tcp"])?;

        let has_domains = [
            &self.requestdomain,
            &self.wsrequestdomain,
            &self.uploaddomain,
            &self.downloaddomain,
            &self.udpdomain,
            &self.tcpdomain,
        ]
        .into_iter()
        .any(|domains| !domains.is_empty());
        validate_open_platform_domain_action(self.action, has_domains)
    }
}

fn validate_open_platform_domain_action(
    action: OpenPlatformMiniProgramDomainAction,
    has_domains: bool,
) -> Result<()> {
    match (action, has_domains) {
        (OpenPlatformMiniProgramDomainAction::Get, true) => Err(WechatError::Config(
            "open platform domain get request must not include domains".to_string(),
        )),
        (
            OpenPlatformMiniProgramDomainAction::Add | OpenPlatformMiniProgramDomainAction::Delete,
            false,
        ) => Err(WechatError::Config(
            "open platform domain add/delete request must include at least one domain".to_string(),
        )),
        _ => Ok(()),
    }
}

fn validate_open_platform_domains(kind: &str, domains: &[String], schemes: &[&str]) -> Result<()> {
    let mut seen = std::collections::HashSet::with_capacity(domains.len());
    for domain in domains {
        let normalized = domain.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            return Err(WechatError::Config(format!(
                "open platform {kind} domain must not be blank"
            )));
        }
        if !seen.insert(normalized) {
            return Err(WechatError::Config(format!(
                "open platform {kind} domains must be unique"
            )));
        }
        let parsed = url::Url::parse(domain).map_err(|error| {
            WechatError::Config(format!(
                "open platform {kind} domain is not an absolute URL: {error}"
            ))
        })?;
        if parsed.host_str().is_none() || !schemes.contains(&parsed.scheme()) {
            return Err(WechatError::Config(format!(
                "open platform {kind} domain must use {}",
                schemes.join(" or ")
            )));
        }
    }
    Ok(())
}

fn validate_open_platform_non_empty(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "open platform {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_open_platform_identifier(kind: &str, value: &str, maximum: usize) -> Result<()> {
    validate_open_platform_non_empty(kind, value)?;
    if value.len() > maximum || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "open platform {kind} must contain at most {maximum} non-control bytes"
        )));
    }
    Ok(())
}

fn validate_open_platform_optional_identifier(
    kind: &str,
    value: Option<&str>,
    maximum: usize,
) -> Result<()> {
    if let Some(value) = value {
        validate_open_platform_identifier(kind, value, maximum)?;
    }
    Ok(())
}

fn validate_open_platform_text(kind: &str, value: &str, maximum: usize) -> Result<()> {
    validate_open_platform_identifier(kind, value, maximum)
}

fn validate_open_platform_optional_text(
    kind: &str,
    value: Option<&str>,
    maximum: usize,
) -> Result<()> {
    if let Some(value) = value {
        validate_open_platform_text(kind, value, maximum)?;
    }
    Ok(())
}

fn validate_open_platform_appid(kind: &str, value: &str) -> Result<()> {
    if !(3..=64).contains(&value.len())
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(WechatError::Config(format!(
            "open platform {kind} must contain 3 to 64 ASCII letters, digits, underscores, or hyphens"
        )));
    }
    Ok(())
}

fn validate_open_platform_callback_url(kind: &str, value: &str) -> Result<()> {
    let url = url::Url::parse(value).map_err(|error| {
        WechatError::Config(format!("open platform {kind} is invalid: {error}"))
    })?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(WechatError::Config(format!(
            "open platform {kind} must be an absolute credential-free HTTPS URL without fragment"
        )));
    }
    Ok(())
}

fn validate_open_platform_modification_quota(
    kind: &str,
    used: Option<i64>,
    quota: Option<i64>,
) -> Result<()> {
    if used.is_some_and(|value| value < 0) || quota.is_some_and(|value| value < 0) {
        return Err(WechatError::Config(format!(
            "open platform {kind} modification counts cannot be negative"
        )));
    }
    if used.zip(quota).is_some_and(|(used, quota)| used > quota) {
        return Err(WechatError::Config(format!(
            "open platform {kind} modification used count cannot exceed quota"
        )));
    }
    Ok(())
}

fn ensure_open_platform_response_success(
    kind: &str,
    errcode: Option<i64>,
    errmsg: Option<&str>,
) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg.unwrap_or(kind).to_string(),
        });
    }
    Ok(())
}

fn validate_open_platform_optional_non_empty(kind: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        validate_open_platform_non_empty(kind, value)?;
    }
    Ok(())
}

fn validate_open_platform_positive(kind: &str, value: i64) -> Result<()> {
    if value <= 0 {
        return Err(WechatError::Config(format!(
            "open platform {kind} must be positive"
        )));
    }
    Ok(())
}

fn validate_open_platform_unique_values(kind: &str, values: &[String]) -> Result<()> {
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_open_platform_non_empty(kind, value)?;
        if !seen.insert(value.trim()) {
            return Err(WechatError::Config(format!(
                "open platform {kind} values must be unique"
            )));
        }
    }
    Ok(())
}

fn validate_open_platform_unique_positive_values(kind: &str, values: &[i64]) -> Result<()> {
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_open_platform_positive(kind, *value)?;
        if !seen.insert(*value) {
            return Err(WechatError::Config(format!(
                "open platform {kind} values must be unique"
            )));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramDomainResult {
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramDomainResult {
    pub fn require_domain(&self) -> Result<&str> {
        let domain = self.domain.as_deref().ok_or_else(|| {
            WechatError::Config("open platform domain result is missing its domain".to_string())
        })?;
        validate_open_platform_non_empty("domain result", domain)?;
        Ok(domain)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_domain()?;
        validate_open_platform_optional_non_empty("domain rejection reason", self.reason.as_deref())
    }
}

fn deserialize_domain_results<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<OpenPlatformMiniProgramDomainResult>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values = Vec::<Value>::deserialize(deserializer)?;
    values
        .into_iter()
        .map(|value| match value {
            Value::String(domain) => Ok(OpenPlatformMiniProgramDomainResult {
                domain: Some(domain),
                status: None,
                reason: None,
                extra: Value::Null,
            }),
            other => serde_json::from_value(other).map_err(serde::de::Error::custom),
        })
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramModifyDomainResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub requestdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub wsrequestdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub uploaddomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub downloaddomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub udpdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub tcpdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_requestdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_wsrequestdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_uploaddomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_downloaddomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_udpdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub invalid_tcpdomain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, deserialize_with = "deserialize_domain_results")]
    pub no_icp_domain: Vec<OpenPlatformMiniProgramDomainResult>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramModifyDomainResponse {
    pub fn configured_domains(&self) -> Vec<&OpenPlatformMiniProgramDomainResult> {
        [
            self.requestdomain.as_slice(),
            self.wsrequestdomain.as_slice(),
            self.uploaddomain.as_slice(),
            self.downloaddomain.as_slice(),
            self.udpdomain.as_slice(),
            self.tcpdomain.as_slice(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn invalid_domains(&self) -> Vec<&OpenPlatformMiniProgramDomainResult> {
        [
            self.invalid_requestdomain.as_slice(),
            self.invalid_wsrequestdomain.as_slice(),
            self.invalid_uploaddomain.as_slice(),
            self.invalid_downloaddomain.as_slice(),
            self.invalid_udpdomain.as_slice(),
            self.invalid_tcpdomain.as_slice(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn has_rejected_domains(&self) -> bool {
        !self.invalid_domains().is_empty() || !self.no_icp_domain.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform domain operation failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        for result in self
            .configured_domains()
            .into_iter()
            .chain(self.invalid_domains())
            .chain(self.no_icp_domain.iter())
        {
            result.validate()?;
        }
        Ok(())
    }

    pub fn ensure_applied(&self) -> Result<()> {
        self.validate()?;
        let invalid_count = self.invalid_domains().len();
        if invalid_count > 0 || !self.no_icp_domain.is_empty() {
            return Err(WechatError::Config(format!(
                "open platform domain operation rejected {invalid_count} invalid and {} non-ICP domains",
                self.no_icp_domain.len()
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterUnbindRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wechatid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userstr: Option<String>,
}

impl OpenPlatformMiniProgramTesterUnbindRequest {
    pub fn by_wechat_id(wechat_id: impl Into<String>) -> Self {
        Self {
            wechatid: Some(wechat_id.into()),
            userstr: None,
        }
    }

    pub fn by_userstr(userstr: impl Into<String>) -> Self {
        Self {
            wechatid: None,
            userstr: Some(userstr.into()),
        }
    }

    pub fn validate(&self) -> Result<()> {
        match (&self.wechatid, &self.userstr) {
            (Some(wechat_id), None) => {
                validate_open_platform_non_empty("tester WeChat ID", wechat_id)
            }
            (None, Some(userstr)) => validate_open_platform_non_empty("tester userstr", userstr),
            (None, None) => Err(WechatError::Config(
                "open platform tester unbind requires wechatid or userstr".to_string(),
            )),
            (Some(_), Some(_)) => Err(WechatError::Config(
                "open platform tester unbind accepts either wechatid or userstr, not both"
                    .to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterBindResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userstr: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramTesterBindResponse {
    pub fn require_userstr(&self) -> Result<&str> {
        ensure_open_platform_response_success(
            "open platform tester binding failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let userstr = self.userstr.as_deref().ok_or_else(|| {
            WechatError::Config(
                "open platform tester binding response is missing userstr".to_string(),
            )
        })?;
        validate_open_platform_non_empty("tester userstr", userstr)?;
        Ok(userstr)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterMember {
    #[serde(default)]
    pub userstr: Option<String>,
    #[serde(default)]
    pub wechatid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub bind_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramTesterMember {
    pub fn validate(&self) -> Result<()> {
        if self.userstr.is_none() && self.wechatid.is_none() {
            return Err(WechatError::Config(
                "open platform tester member requires userstr or WeChat ID".to_string(),
            ));
        }
        validate_open_platform_optional_non_empty("tester userstr", self.userstr.as_deref())?;
        validate_open_platform_optional_non_empty("tester WeChat ID", self.wechatid.as_deref())?;
        if self.bind_time.is_some_and(|bind_time| bind_time < 0) {
            return Err(WechatError::Config(
                "open platform tester bind time cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub members: Vec<OpenPlatformMiniProgramTesterMember>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramTesterListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform tester listing failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let mut userstrs = std::collections::HashSet::with_capacity(self.members.len());
        let mut wechat_ids = std::collections::HashSet::with_capacity(self.members.len());
        for member in &self.members {
            member.validate()?;
            if let Some(userstr) = member.userstr.as_deref() {
                if !userstrs.insert(userstr.trim()) {
                    return Err(WechatError::Config(
                        "open platform tester members contain duplicate userstr values".to_string(),
                    ));
                }
            }
            if let Some(wechat_id) = member.wechatid.as_deref() {
                if !wechat_ids.insert(wechat_id.trim()) {
                    return Err(WechatError::Config(
                        "open platform tester members contain duplicate WeChat IDs".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn find_by_userstr(&self, userstr: &str) -> Option<&OpenPlatformMiniProgramTesterMember> {
        self.members
            .iter()
            .find(|member| member.userstr.as_deref() == Some(userstr))
    }

    pub fn find_by_wechat_id(
        &self,
        wechat_id: &str,
    ) -> Option<&OpenPlatformMiniProgramTesterMember> {
        self.members
            .iter()
            .find(|member| member.wechatid.as_deref() == Some(wechat_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyOwnerSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_qq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_weixin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_file_media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_expire_timestamp: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySettingItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySdkItem {
    pub privacy_key: String,
    pub privacy_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_label: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramPrivacySdkItem {
    fn validate(&self) -> Result<()> {
        validate_open_platform_non_empty("privacy SDK key", &self.privacy_key)?;
        validate_open_platform_non_empty("privacy SDK text", &self.privacy_text)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySdkInfo {
    pub sdk_name: String,
    pub sdk_biz_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sdk_list: Vec<OpenPlatformMiniProgramPrivacySdkItem>,
}

impl OpenPlatformMiniProgramPrivacySdkInfo {
    fn validate(&self) -> Result<()> {
        validate_open_platform_non_empty("privacy SDK name", &self.sdk_name)?;
        validate_open_platform_non_empty("privacy SDK business name", &self.sdk_biz_name)?;
        if self.sdk_list.is_empty() {
            return Err(WechatError::Config(
                "open platform privacy SDK list must not be empty".to_string(),
            ));
        }
        let mut keys = std::collections::HashSet::with_capacity(self.sdk_list.len());
        for item in &self.sdk_list {
            item.validate()?;
            if !keys.insert(item.privacy_key.trim()) {
                return Err(WechatError::Config(
                    "open platform privacy SDK keys must be unique within an SDK".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySettingRequest {
    pub owner_setting: OpenPlatformMiniProgramPrivacyOwnerSetting,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub setting_list: Vec<OpenPlatformMiniProgramPrivacySettingItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sdk_privacy_info_list: Vec<OpenPlatformMiniProgramPrivacySdkInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_ver: Option<i64>,
}

impl OpenPlatformMiniProgramPrivacySettingRequest {
    pub fn validate(&self) -> Result<()> {
        self.owner_setting.validate()?;
        if self.privacy_ver.is_some_and(|version| version <= 0) {
            return Err(WechatError::Config(
                "open platform privacy version must be positive".to_string(),
            ));
        }
        let mut keys = std::collections::HashSet::with_capacity(self.setting_list.len());
        for item in &self.setting_list {
            item.validate()?;
            let key = item
                .privacy_key
                .as_deref()
                .expect("validated privacy key is present")
                .trim();
            if !keys.insert(key) {
                return Err(WechatError::Config(
                    "open platform privacy setting keys must be unique".to_string(),
                ));
            }
        }
        let mut sdk_names =
            std::collections::HashSet::with_capacity(self.sdk_privacy_info_list.len());
        for sdk in &self.sdk_privacy_info_list {
            sdk.validate()?;
            if !sdk_names.insert(sdk.sdk_name.trim()) {
                return Err(WechatError::Config(
                    "open platform privacy SDK names must be unique".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl OpenPlatformMiniProgramPrivacyOwnerSetting {
    fn validate(&self) -> Result<()> {
        let contacts = [
            self.contact_email.as_deref(),
            self.contact_phone.as_deref(),
            self.contact_qq.as_deref(),
            self.contact_weixin.as_deref(),
        ];
        if !contacts
            .into_iter()
            .flatten()
            .any(|contact| !contact.trim().is_empty())
        {
            return Err(WechatError::Config(
                "open platform privacy owner requires at least one contact method".to_string(),
            ));
        }
        for (kind, value) in [
            (
                "privacy extension media id",
                self.ext_file_media_id.as_deref(),
            ),
            ("privacy notice method", self.notice_method.as_deref()),
            (
                "privacy store expiration timestamp",
                self.store_expire_timestamp.as_deref(),
            ),
        ] {
            if let Some(value) = value {
                validate_open_platform_non_empty(kind, value)?;
            }
        }
        Ok(())
    }
}

impl OpenPlatformMiniProgramPrivacySettingItem {
    fn validate(&self) -> Result<()> {
        let key = self.privacy_key.as_deref().ok_or_else(|| {
            WechatError::Config("open platform privacy setting key is required".to_string())
        })?;
        let text = self.privacy_text.as_deref().ok_or_else(|| {
            WechatError::Config("open platform privacy setting text is required".to_string())
        })?;
        validate_open_platform_non_empty("privacy setting key", key)?;
        validate_open_platform_non_empty("privacy setting text", text)?;
        if let Some(url) = self.privacy_url.as_deref() {
            validate_open_platform_http_url("privacy setting URL", url)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyDescriptionItem {
    #[serde(default)]
    pub privacy_key: Option<String>,
    #[serde(default)]
    pub privacy_desc: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyDescription {
    #[serde(default)]
    pub privacy_desc_list: Vec<OpenPlatformMiniProgramPrivacyDescriptionItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySettingResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub owner_setting: Option<OpenPlatformMiniProgramPrivacyOwnerSetting>,
    #[serde(default)]
    pub setting_list: Vec<OpenPlatformMiniProgramPrivacySettingItem>,
    #[serde(default)]
    pub sdk_privacy_info_list: Vec<OpenPlatformMiniProgramPrivacySdkInfo>,
    #[serde(default)]
    pub privacy_list: Vec<String>,
    #[serde(default)]
    pub code_exist: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default)]
    pub privacy_desc: Option<OpenPlatformMiniProgramPrivacyDescription>,
    #[serde(default)]
    pub privacy_ver: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramPrivacySettingResponse {
    pub fn is_code_configured(&self) -> bool {
        self.code_exist == Some(1)
    }

    pub fn find_setting(
        &self,
        privacy_key: &str,
    ) -> Option<&OpenPlatformMiniProgramPrivacySettingItem> {
        self.setting_list
            .iter()
            .find(|item| item.privacy_key.as_deref() == Some(privacy_key))
    }

    pub fn undeclared_code_privacy_keys(&self) -> Vec<&str> {
        self.privacy_list
            .iter()
            .map(String::as_str)
            .filter(|privacy_key| self.find_setting(privacy_key).is_none())
            .collect()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform privacy setting query failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self
            .code_exist
            .is_some_and(|code_exist| !matches!(code_exist, 0 | 1))
        {
            return Err(WechatError::Config(
                "open platform privacy code-exist flag must be 0 or 1".to_string(),
            ));
        }
        if self.update_time.is_some_and(|update_time| update_time < 0) {
            return Err(WechatError::Config(
                "open platform privacy update time cannot be negative".to_string(),
            ));
        }
        if self.privacy_ver.is_some_and(|version| version <= 0) {
            return Err(WechatError::Config(
                "open platform privacy version must be positive".to_string(),
            ));
        }

        let mut setting_keys = std::collections::HashSet::with_capacity(self.setting_list.len());
        for item in &self.setting_list {
            let key = item.privacy_key.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform privacy response setting key is required".to_string(),
                )
            })?;
            validate_open_platform_non_empty("privacy response setting key", key)?;
            validate_open_platform_optional_non_empty(
                "privacy response setting text",
                item.privacy_text.as_deref(),
            )?;
            if let Some(url) = item.privacy_url.as_deref() {
                validate_open_platform_http_url("privacy response setting URL", url)?;
            }
            if !setting_keys.insert(key.trim()) {
                return Err(WechatError::Config(
                    "open platform privacy response setting keys must be unique".to_string(),
                ));
            }
        }

        validate_open_platform_unique_values("privacy code key", &self.privacy_list)?;
        let mut sdk_names =
            std::collections::HashSet::with_capacity(self.sdk_privacy_info_list.len());
        for sdk in &self.sdk_privacy_info_list {
            sdk.validate()?;
            if !sdk_names.insert(sdk.sdk_name.trim()) {
                return Err(WechatError::Config(
                    "open platform privacy response SDK names must be unique".to_string(),
                ));
            }
        }
        if let Some(description) = &self.privacy_desc {
            let mut description_keys =
                std::collections::HashSet::with_capacity(description.privacy_desc_list.len());
            for item in &description.privacy_desc_list {
                let key = item.privacy_key.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "open platform privacy description key is required".to_string(),
                    )
                })?;
                let value = item.privacy_desc.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "open platform privacy description text is required".to_string(),
                    )
                })?;
                validate_open_platform_non_empty("privacy description key", key)?;
                validate_open_platform_non_empty("privacy description text", value)?;
                if !description_keys.insert(key.trim()) {
                    return Err(WechatError::Config(
                        "open platform privacy description keys must be unique".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn ensure_ready_for_submission(&self) -> Result<()> {
        self.validate()?;
        if !self.is_code_configured() {
            return Err(WechatError::Config(
                "open platform mini-program code has no detected privacy usage".to_string(),
            ));
        }
        let undeclared = self.undeclared_code_privacy_keys();
        if !undeclared.is_empty() {
            return Err(WechatError::Config(format!(
                "open platform privacy settings do not declare code privacy keys: {}",
                undeclared.join(", ")
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyExtFileResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ext_file_media_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramPrivacyExtFileResponse {
    pub fn require_media_id(&self) -> Result<&str> {
        ensure_open_platform_response_success(
            "open platform privacy extension upload failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let media_id = self.ext_file_media_id.as_deref().ok_or_else(|| {
            WechatError::Config(
                "open platform privacy extension upload response is missing media id".to_string(),
            )
        })?;
        validate_open_platform_non_empty("privacy extension media id", media_id)?;
        Ok(media_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformComponentLoginPageUrlRequest {
    pub component_appid: String,
    pub pre_auth_code: String,
    pub redirect_uri: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_query: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformOfficialAccountFastRegistrationUrlRequest {
    pub component_appid: String,
    pub appid: String,
    pub redirect_uri: String,
    pub copy_wx_verify: bool,
}

impl OpenPlatformOfficialAccountFastRegistrationUrlRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_appid("component appid", &self.component_appid)?;
        validate_open_platform_appid("authorizer appid", &self.appid)?;
        validate_open_platform_callback_url(
            "official-account fast-registration redirect",
            &self.redirect_uri,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenPlatformBinaryFlag {
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl OpenPlatformBinaryFlag {
    pub fn value(&self) -> Result<bool> {
        match self {
            Self::Boolean(value) => Ok(*value),
            Self::Integer(0) => Ok(false),
            Self::Integer(1) => Ok(true),
            Self::String(value) if value == "0" || value.eq_ignore_ascii_case("false") => Ok(false),
            Self::String(value) if value == "1" || value.eq_ignore_ascii_case("true") => Ok(true),
            _ => Err(WechatError::Config(
                "open platform binary flag must be bool, 0/1, or true/false".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformOfficialAccountFastRegistrationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub authorization_code: Option<String>,
    #[serde(default)]
    pub is_wx_verify_succ: Option<OpenPlatformBinaryFlag>,
    #[serde(default)]
    pub is_link_succ: Option<OpenPlatformBinaryFlag>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformOfficialAccountFastRegistrationResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform official-account fast registration failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_platform_appid(
            "registered mini-program appid",
            self.appid.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform fast-registration response is missing appid".to_string(),
                )
            })?,
        )?;
        validate_open_platform_identifier(
            "fast-registration authorization code",
            self.authorization_code.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform fast-registration response is missing authorization code"
                        .to_string(),
                )
            })?,
            512,
        )?;
        self.is_wx_verify_succ
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open platform fast-registration response is missing verification result"
                        .to_string(),
                )
            })?
            .value()?;
        self.is_link_succ
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open platform fast-registration response is missing link result".to_string(),
                )
            })?
            .value()?;
        Ok(())
    }

    pub fn is_verified(&self) -> Result<bool> {
        self.validate()?;
        self.is_wx_verify_succ
            .as_ref()
            .expect("validated verification result")
            .value()
    }

    pub fn is_linked(&self) -> Result<bool> {
        self.validate()?;
        self.is_link_succ
            .as_ref()
            .expect("validated link result")
            .value()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramSessionResponse {
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramSessionResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform mini-program code-to-session failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_platform_identifier(
            "mini-program session openid",
            self.openid.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform mini-program session is missing openid".to_string(),
                )
            })?,
            128,
        )?;
        validate_open_platform_identifier(
            "mini-program session key",
            self.session_key.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform mini-program session is missing session key".to_string(),
                )
            })?,
            512,
        )?;
        validate_open_platform_optional_identifier(
            "mini-program session unionid",
            self.unionid.as_deref(),
            128,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerAccountHeadImageRequest {
    pub head_img_media_id: String,
    pub x1: String,
    pub y1: String,
    pub x2: String,
    pub y2: String,
}

impl OpenPlatformAuthorizerAccountHeadImageRequest {
    pub fn from_crop(
        media_id: impl Into<String>,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    ) -> Self {
        Self {
            head_img_media_id: media_id.into(),
            x1: left.to_string(),
            y1: top.to_string(),
            x2: right.to_string(),
            y2: bottom.to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_open_platform_identifier(
            "authorizer head-image media id",
            &self.head_img_media_id,
            256,
        )?;
        let [left, top, right, bottom] = [
            ("x1", &self.x1),
            ("y1", &self.y1),
            ("x2", &self.x2),
            ("y2", &self.y2),
        ]
        .map(|(label, value)| {
            let coordinate = value.parse::<f64>().map_err(|error| {
                WechatError::Config(format!(
                    "open platform authorizer head-image {label} is invalid: {error}"
                ))
            })?;
            if !coordinate.is_finite() || !(0.0..=1.0).contains(&coordinate) {
                return Err(WechatError::Config(format!(
                    "open platform authorizer head-image {label} must be between 0 and 1"
                )));
            }
            Ok(coordinate)
        })
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| {
            WechatError::Config(
                "open platform authorizer head-image crop requires four coordinates".to_string(),
            )
        })?;
        if left >= right || top >= bottom {
            return Err(WechatError::Config(
                "open platform authorizer head-image crop must have positive width and height"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerAccountBasicInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub account_type: Option<i64>,
    #[serde(default)]
    pub principal_type: Option<i64>,
    #[serde(default)]
    pub principal_name: Option<String>,
    #[serde(default)]
    pub realname_status: Option<i64>,
    #[serde(default)]
    pub wx_verify_info: Option<OpenPlatformAuthorizerWxVerifyInfo>,
    #[serde(default)]
    pub signature_info: Option<OpenPlatformAuthorizerSignatureInfo>,
    #[serde(default)]
    pub head_image_info: Option<OpenPlatformAuthorizerHeadImageInfo>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub registered_country: Option<i64>,
    #[serde(default)]
    pub nickname_info: Option<OpenPlatformAuthorizerNicknameInfo>,
    #[serde(default)]
    pub credential: Option<String>,
    #[serde(default)]
    pub customer_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerWxVerifyInfo {
    #[serde(default)]
    pub qualification_verify: Option<bool>,
    #[serde(default)]
    pub naming_verify: Option<bool>,
    #[serde(default)]
    pub annual_review: Option<bool>,
    #[serde(default)]
    pub annual_review_begin_time: Option<i64>,
    #[serde(default)]
    pub annual_review_end_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerSignatureInfo {
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub modify_used_count: Option<i64>,
    #[serde(default)]
    pub modify_quota: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerHeadImageInfo {
    #[serde(default)]
    pub head_image_url: Option<String>,
    #[serde(default)]
    pub modify_used_count: Option<i64>,
    #[serde(default)]
    pub modify_quota: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerNicknameInfo {
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub modify_used_count: Option<i64>,
    #[serde(default)]
    pub modify_quota: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformAuthorizerAccountBasicInfoResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform get authorizer account basic info failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_platform_appid(
            "authorizer account appid",
            self.appid.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform authorizer account basic info is missing appid".to_string(),
                )
            })?,
        )?;
        for (label, value) in [
            ("account type", self.account_type),
            ("principal type", self.principal_type),
            ("real-name status", self.realname_status),
            ("registered country", self.registered_country),
            ("customer type", self.customer_type),
        ] {
            if value.is_some_and(|value| value < 0) {
                return Err(WechatError::Config(format!(
                    "open platform authorizer account {label} cannot be negative"
                )));
            }
        }
        validate_open_platform_optional_text(
            "authorizer principal name",
            self.principal_name.as_deref(),
            256,
        )?;
        validate_open_platform_optional_text("authorizer nickname", self.nickname.as_deref(), 64)?;
        validate_open_platform_optional_identifier(
            "authorizer credential",
            self.credential.as_deref(),
            512,
        )?;
        if let Some(info) = &self.wx_verify_info {
            info.validate()?;
        }
        if let Some(info) = &self.signature_info {
            info.validate()?;
        }
        if let Some(info) = &self.head_image_info {
            info.validate()?;
        }
        if let Some(info) = &self.nickname_info {
            info.validate()?;
        }
        Ok(())
    }
}

impl OpenPlatformAuthorizerWxVerifyInfo {
    pub fn validate(&self) -> Result<()> {
        for (label, value) in [
            ("annual-review begin time", self.annual_review_begin_time),
            ("annual-review end time", self.annual_review_end_time),
        ] {
            if value.is_some_and(|value| value < 0) {
                return Err(WechatError::Config(format!(
                    "open platform authorizer {label} cannot be negative"
                )));
            }
        }
        if self
            .annual_review_begin_time
            .zip(self.annual_review_end_time)
            .is_some_and(|(begin, end)| end < begin)
        {
            return Err(WechatError::Config(
                "open platform authorizer annual-review end time cannot precede begin time"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

impl OpenPlatformAuthorizerSignatureInfo {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_optional_text(
            "authorizer signature",
            self.signature.as_deref(),
            120,
        )?;
        validate_open_platform_modification_quota(
            "authorizer signature",
            self.modify_used_count,
            self.modify_quota,
        )
    }
}

impl OpenPlatformAuthorizerHeadImageInfo {
    pub fn validate(&self) -> Result<()> {
        if let Some(url) = self.head_image_url.as_deref() {
            validate_open_platform_http_url("authorizer head-image URL", url)?;
        }
        validate_open_platform_modification_quota(
            "authorizer head image",
            self.modify_used_count,
            self.modify_quota,
        )
    }
}

impl OpenPlatformAuthorizerNicknameInfo {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_optional_text(
            "authorizer nickname quota value",
            self.nickname.as_deref(),
            64,
        )?;
        validate_open_platform_modification_quota(
            "authorizer nickname",
            self.modify_used_count,
            self.modify_quota,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformOpenAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_appid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformOpenAccountResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform open-account operation failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_platform_appid(
            "open account appid",
            self.open_appid.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "open platform open-account response is missing open_appid".to_string(),
                )
            })?,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyInterfaceResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub interface_list: Vec<OpenPlatformMiniProgramPrivacyInterface>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyInterface {
    #[serde(default)]
    pub api_name: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub fail_reason: Option<String>,
    #[serde(default)]
    pub audit_id: Option<i64>,
    #[serde(default)]
    pub apply_time: Option<i64>,
    #[serde(default)]
    pub audit_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlatformMiniProgramPrivacyInterfaceState {
    PendingApply,
    NoPermission,
    Applying,
    Rejected,
    Approved,
    Other,
}

impl OpenPlatformMiniProgramPrivacyInterface {
    pub fn interface_state(&self) -> Option<OpenPlatformMiniProgramPrivacyInterfaceState> {
        self.status.map(|status| match status {
            1 => OpenPlatformMiniProgramPrivacyInterfaceState::PendingApply,
            2 => OpenPlatformMiniProgramPrivacyInterfaceState::NoPermission,
            3 => OpenPlatformMiniProgramPrivacyInterfaceState::Applying,
            4 => OpenPlatformMiniProgramPrivacyInterfaceState::Rejected,
            5 => OpenPlatformMiniProgramPrivacyInterfaceState::Approved,
            _ => OpenPlatformMiniProgramPrivacyInterfaceState::Other,
        })
    }

    pub fn is_approved(&self) -> bool {
        self.interface_state() == Some(OpenPlatformMiniProgramPrivacyInterfaceState::Approved)
    }

    pub fn needs_apply(&self) -> bool {
        matches!(
            self.interface_state(),
            Some(
                OpenPlatformMiniProgramPrivacyInterfaceState::PendingApply
                    | OpenPlatformMiniProgramPrivacyInterfaceState::NoPermission
                    | OpenPlatformMiniProgramPrivacyInterfaceState::Rejected
            )
        )
    }

    pub fn validate(&self) -> Result<()> {
        let api_name = self.api_name.as_deref().ok_or_else(|| {
            WechatError::Config(
                "open platform privacy interface is missing its API name".to_string(),
            )
        })?;
        validate_open_platform_non_empty("privacy interface API name", api_name)?;
        validate_open_platform_optional_non_empty(
            "privacy interface failure reason",
            self.fail_reason.as_deref(),
        )?;
        if self.audit_id.is_some_and(|audit_id| audit_id <= 0) {
            return Err(WechatError::Config(
                "open platform privacy interface audit id must be positive".to_string(),
            ));
        }
        if self.apply_time.is_some_and(|apply_time| apply_time < 0)
            || self.audit_time.is_some_and(|audit_time| audit_time < 0)
        {
            return Err(WechatError::Config(
                "open platform privacy interface timestamps cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

impl OpenPlatformMiniProgramPrivacyInterfaceResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_platform_response_success(
            "open platform privacy interface query failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let mut api_names = std::collections::HashSet::with_capacity(self.interface_list.len());
        for interface in &self.interface_list {
            interface.validate()?;
            let api_name = interface
                .api_name
                .as_deref()
                .expect("validated privacy interface API name is present");
            if !api_names.insert(api_name.trim()) {
                return Err(WechatError::Config(
                    "open platform privacy interface API names must be unique".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find(&self, api_name: &str) -> Option<&OpenPlatformMiniProgramPrivacyInterface> {
        self.interface_list
            .iter()
            .find(|interface| interface.api_name.as_deref() == Some(api_name))
    }

    pub fn requiring_action(&self) -> Vec<&OpenPlatformMiniProgramPrivacyInterface> {
        self.interface_list
            .iter()
            .filter(|interface| interface.needs_apply())
            .collect()
    }

    pub fn all_approved(&self) -> bool {
        !self.interface_list.is_empty()
            && self
                .interface_list
                .iter()
                .all(OpenPlatformMiniProgramPrivacyInterface::is_approved)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
    pub api_name: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<i64>,
}

impl OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_platform_non_empty("privacy interface API name", &self.api_name)?;
        validate_open_platform_non_empty("privacy interface application content", &self.content)?;
        if let Some(url) = self.url.as_deref() {
            validate_open_platform_http_url("privacy interface application URL", url)?;
        }
        if self.scene.is_some_and(|scene| scene <= 0) {
            return Err(WechatError::Config(
                "open platform privacy interface scene must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_open_platform_http_url(kind: &str, value: &str) -> Result<()> {
    let url = url::Url::parse(value).map_err(|error| {
        WechatError::Config(format!("open platform {kind} is invalid: {error}"))
    })?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(WechatError::Config(format!(
            "open platform {kind} must be an absolute HTTP(S) URL"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyInterfaceApplyResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub audit_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl OpenPlatformMiniProgramPrivacyInterfaceApplyResponse {
    pub fn require_audit_id(&self) -> Result<i64> {
        ensure_open_platform_response_success(
            "open platform privacy interface application failed",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        let audit_id = self.audit_id.ok_or_else(|| {
            WechatError::Config(
                "open platform privacy interface application response is missing audit id"
                    .to_string(),
            )
        })?;
        validate_open_platform_positive("privacy interface audit id", audit_id)?;
        Ok(audit_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTemplateFromDraftRequest {
    pub draft_id: i64,
    pub template_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformTemplateDraftListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub draft_list: Vec<OpenPlatformTemplateDraft>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformTemplateDraft {
    #[serde(default)]
    pub draft_id: Option<i64>,
    #[serde(default)]
    pub user_version: Option<String>,
    #[serde(default)]
    pub user_desc: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub source_miniprogram_appid: Option<String>,
    #[serde(default)]
    pub source_miniprogram: Option<String>,
    #[serde(default)]
    pub developer: Option<String>,
    #[serde(default)]
    pub template_type: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformTemplateListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_list: Vec<OpenPlatformTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformTemplate {
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub user_version: Option<String>,
    #[serde(default)]
    pub user_desc: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub source_miniprogram_appid: Option<String>,
    #[serde(default)]
    pub source_miniprogram: Option<String>,
    #[serde(default)]
    pub developer: Option<String>,
    #[serde(default)]
    pub template_type: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTemplateRequest {
    pub template_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterMiniProgramRequest {
    pub name: String,
    pub code: String,
    pub code_type: i64,
    pub legal_persona_wechat: String,
    pub legal_persona_name: String,
    pub component_phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationStatusRequest {
    pub name: String,
    pub legal_persona_wechat: String,
    pub legal_persona_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl OpenPlatformStatusResponse {
    pub fn validate_for(&self, operation: &str) -> Result<()> {
        ensure_open_platform_response_success(operation, self.errcode, self.errmsg.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::{
        error::WechatError,
        modules::official_account::{
            Article, MaterialGetResponse, MaterialListKind, MaterialListRequest,
            MaterialListResponse, MaterialMediaResponse, MaterialStatsResponse,
        },
    };

    use super::{
        AddTemplateFromDraftRequest, AuthorizerAccessTokenRequest, AuthorizerAccessTokenResponse,
        ComponentAccessTokenRequest, ComponentAccessTokenResponse, DeleteTemplateRequest,
        OpenPlatform, OpenPlatformAuthorizerAccountBasicInfoResponse,
        OpenPlatformAuthorizerAccountHeadImageRequest, OpenPlatformAuthorizerInfoResponse,
        OpenPlatformAuthorizerOptionResponse, OpenPlatformAuthorizersResponse,
        OpenPlatformBinaryFlag, OpenPlatformComponentLoginPageUrlRequest,
        OpenPlatformHandleAuthorizeResponse, OpenPlatformMiniProgramAuditItem,
        OpenPlatformMiniProgramAuditPreviewInfo, OpenPlatformMiniProgramAuditQuotaResponse,
        OpenPlatformMiniProgramAuditState, OpenPlatformMiniProgramAuditStatusResponse,
        OpenPlatformMiniProgramCategoryAuditState, OpenPlatformMiniProgramCategoryResponse,
        OpenPlatformMiniProgramCommitRequest, OpenPlatformMiniProgramDomainAction,
        OpenPlatformMiniProgramGrayReleasePlanResponse, OpenPlatformMiniProgramGrayReleaseState,
        OpenPlatformMiniProgramLatestAuditStatusResponse,
        OpenPlatformMiniProgramModifyDomainRequest, OpenPlatformMiniProgramModifyDomainResponse,
        OpenPlatformMiniProgramPageResponse, OpenPlatformMiniProgramPrivacyExtFileResponse,
        OpenPlatformMiniProgramPrivacyInterfaceApplyRequest,
        OpenPlatformMiniProgramPrivacyInterfaceApplyResponse,
        OpenPlatformMiniProgramPrivacyInterfaceResponse,
        OpenPlatformMiniProgramPrivacyInterfaceState, OpenPlatformMiniProgramPrivacyOwnerSetting,
        OpenPlatformMiniProgramPrivacySdkInfo, OpenPlatformMiniProgramPrivacySdkItem,
        OpenPlatformMiniProgramPrivacySettingItem, OpenPlatformMiniProgramPrivacySettingRequest,
        OpenPlatformMiniProgramPrivacySettingResponse,
        OpenPlatformMiniProgramRollbackReleaseResponse, OpenPlatformMiniProgramSessionResponse,
        OpenPlatformMiniProgramSubmitAuditRequest, OpenPlatformMiniProgramSubmitAuditResponse,
        OpenPlatformMiniProgramSupportVersionResponse, OpenPlatformMiniProgramTesterBindResponse,
        OpenPlatformMiniProgramTesterListResponse, OpenPlatformMiniProgramTesterUnbindRequest,
        OpenPlatformMiniProgramUgcDeclare, OpenPlatformMiniProgramVisitStatusAction,
        OpenPlatformOfficialAccountFastRegistrationResponse,
        OpenPlatformOfficialAccountFastRegistrationUrlRequest, OpenPlatformOpenAccountResponse,
        OpenPlatformStatusResponse, OpenPlatformTemplateDraftListResponse,
        OpenPlatformTemplateListResponse, PreauthCodeResponse, QueryAuthResponse,
        RegisterMiniProgramRequest, RegistrationStatusRequest,
    };

    #[test]
    fn serializes_component_token_request() {
        let value = serde_json::to_value(ComponentAccessTokenRequest {
            component_appid: "appid".to_string(),
            component_appsecret: "secret".to_string(),
            component_verify_ticket: "ticket".to_string(),
        })
        .unwrap();

        assert_eq!(
            value,
            json!({
                "component_appid": "appid",
                "component_appsecret": "secret",
                "component_verify_ticket": "ticket"
            })
        );
    }

    #[test]
    fn deserializes_component_auth_responses() {
        let token: ComponentAccessTokenResponse = serde_json::from_value(json!({
            "component_access_token": "component-token",
            "expires_in": 7200
        }))
        .unwrap();
        let preauth: PreauthCodeResponse =
            serde_json::from_value(json!({ "pre_auth_code": "preauth", "expires_in": 600 }))
                .unwrap();
        let query_auth: QueryAuthResponse = serde_json::from_value(json!({
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "func_info": [{
                    "funcscope_category": {
                        "id": 18,
                        "scope_name": "customer-service"
                    },
                    "scope_revision": 2
                }],
                "authorization_revision": 3
            },
            "request_id": "query-auth-1"
        }))
        .unwrap();

        assert_eq!(
            token.component_access_token.as_deref(),
            Some("component-token")
        );
        assert_eq!(token.expires_in, Some(7200));
        assert_eq!(preauth.pre_auth_code.as_deref(), Some("preauth"));
        let query_authorization = query_auth.authorization_info.expect("authorization_info");
        assert_eq!(
            query_authorization.authorizer_appid.as_deref(),
            Some("wx-authorizer")
        );
        assert_eq!(
            query_authorization.func_info[0]
                .funcscope_category
                .as_ref()
                .unwrap()
                .id,
            Some(18)
        );
        assert_eq!(
            query_authorization.func_info[0]
                .funcscope_category
                .as_ref()
                .unwrap()
                .extra["scope_name"],
            "customer-service"
        );
        assert_eq!(query_authorization.func_info[0].extra["scope_revision"], 2);
        assert_eq!(query_authorization.extra["authorization_revision"], 3);
        assert_eq!(query_auth.extra["request_id"], "query-auth-1");
    }

    #[test]
    fn serializes_authorizer_access_token_request() {
        let value = serde_json::to_value(AuthorizerAccessTokenRequest {
            component_appid: "component".to_string(),
            authorizer_appid: "authorizer".to_string(),
            authorizer_refresh_token: "refresh".to_string(),
        })
        .unwrap();

        assert_eq!(value["component_appid"], "component");
        assert_eq!(value["authorizer_appid"], "authorizer");
        assert_eq!(value["authorizer_refresh_token"], "refresh");
    }

    #[test]
    fn deserializes_authorizer_access_token_response() {
        let response: AuthorizerAccessTokenResponse = serde_json::from_value(json!({
            "authorizer_access_token": "authorizer-token",
            "expires_in": 7200,
            "authorizer_refresh_token": "refresh-token"
        }))
        .unwrap();

        assert_eq!(
            response.authorizer_access_token.as_deref(),
            Some("authorizer-token")
        );
        assert_eq!(response.expires_in, Some(7200));
        assert_eq!(
            response.authorizer_refresh_token.as_deref(),
            Some("refresh-token")
        );
    }

    #[test]
    fn deserializes_open_platform_base_responses() {
        let authorize: OpenPlatformHandleAuthorizeResponse = serde_json::from_value(json!({
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "authorizer_access_token": "authorizer-token",
                "expires_in": 7200,
                "authorizer_refresh_token": "refresh-token",
                "func_info": [{
                    "funcscope_category": { "id": 1 },
                    "confirm_info": {
                        "need_confirm": 1,
                        "already_confirm": 0,
                        "can_confirm": 1
                    }
                }]
            }
        }))
        .unwrap();
        let info = authorize.authorization_info.expect("authorization_info");
        assert_eq!(info.authorizer_appid.as_deref(), Some("wx-authorizer"));
        assert_eq!(
            info.func_info[0].funcscope_category.as_ref().unwrap().id,
            Some(1)
        );
        assert_eq!(
            info.func_info[0].confirm_info.as_ref().unwrap().can_confirm,
            Some(1)
        );

        let authorizer: OpenPlatformAuthorizerInfoResponse = serde_json::from_value(json!({
            "authorizer_info": {
                "nick_name": "demo",
                "head_img": "https://example.com/head.png",
                "service_type_info": { "id": 2 },
                "verify_type_info": { "id": 0 },
                "user_name": "gh_xxx",
                "principal_name": "principal",
                "business_info": { "open_store": 1, "open_scan": 1 },
                "alias": "alias",
                "qrcode_url": "https://example.com/qrcode",
                "account_status": 0,
                "MiniProgramInfo": {
                    "network": {
                        "RequestDomain": ["https://api.example.com"],
                        "WsRequestDomain": ["wss://api.example.com"],
                        "UploadDomain": ["https://upload.example.com"],
                        "DownloadDomain": ["https://download.example.com"],
                        "BizDomain": ["https://web.example.com"],
                        "NewRequestDomain": ["https://new-api.example.com"],
                        "network_revision": 2
                    },
                    "categories": [{
                        "first": "Tools",
                        "second": "Efficiency",
                        "category_id": 100
                    }],
                    "visit_status": 0,
                    "mini_program_revision": 3
                },
                "basic_config": {
                    "is_phone_configured": true,
                    "is_email_configured": false,
                    "config_revision": 4
                },
                "authorizer_revision": 5
            },
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "func_info": []
            },
            "request_id": "authorizer-info-1"
        }))
        .unwrap();
        let authorizer_info = authorizer.authorizer_info.expect("authorizer_info");
        assert_eq!(authorizer_info.nick_name.as_deref(), Some("demo"));
        assert_eq!(authorizer_info.service_type_info.unwrap().id, Some(2));
        let basic_config = authorizer_info.basic_config.as_ref().unwrap();
        assert_eq!(basic_config.is_phone_configured, Some(true));
        assert_eq!(basic_config.extra["config_revision"], 4);
        let mini_program_info = authorizer_info.mini_program_info.as_ref().unwrap();
        let network = mini_program_info.network.as_ref().unwrap();
        assert_eq!(network.request_domain[0], "https://api.example.com");
        assert_eq!(network.ws_request_domain[0], "wss://api.example.com");
        assert_eq!(network.new_request_domain[0], "https://new-api.example.com");
        assert_eq!(network.extra["network_revision"], 2);
        assert_eq!(
            mini_program_info.categories[0].first.as_deref(),
            Some("Tools")
        );
        assert_eq!(mini_program_info.categories[0].extra["category_id"], 100);
        assert_eq!(mini_program_info.visit_status, Some(0));
        assert_eq!(mini_program_info.extra["mini_program_revision"], 3);
        assert_eq!(authorizer_info.extra["authorizer_revision"], 5);
        assert_eq!(authorizer.extra["request_id"], "authorizer-info-1");

        let list: OpenPlatformAuthorizersResponse = serde_json::from_value(json!({
            "total_count": 1,
            "list": [{
                "authorizer_appid": "wx-authorizer",
                "refresh_token": "refresh-token",
                "auth_time": 1800000000,
                "authorization_revision": 2
            }],
            "next_offset": 1
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.list[0].refresh_token.as_deref(), Some("refresh-token"));
        assert_eq!(list.list[0].extra["authorization_revision"], 2);
        assert_eq!(list.extra["next_offset"], 1);

        let option: OpenPlatformAuthorizerOptionResponse = serde_json::from_value(json!({
            "authorizer_appid": "wx-authorizer",
            "option_name": "location_report",
            "option_value": "1"
        }))
        .unwrap();
        assert_eq!(option.option_name.as_deref(), Some("location_report"));
        assert_eq!(option.option_value.as_deref(), Some("1"));
    }

    #[test]
    fn serializes_code_template_requests() {
        let add = serde_json::to_value(AddTemplateFromDraftRequest {
            draft_id: 100,
            template_type: 0,
        })
        .unwrap();
        let delete = serde_json::to_value(DeleteTemplateRequest {
            template_id: "tpl".to_string(),
        })
        .unwrap();

        assert_eq!(add, json!({ "draft_id": 100, "template_type": 0 }));
        assert_eq!(delete, json!({ "template_id": "tpl" }));

        let drafts: OpenPlatformTemplateDraftListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "draft_list": [{
                "draft_id": 100,
                "user_version": "1.0.0",
                "user_desc": "draft",
                "create_time": 1_800_000_000,
                "source_miniprogram_appid": "wx-source",
                "source_miniprogram": "Source",
                "developer": "dev",
                "template_type": 0,
                "extra_field": "kept"
            }]
        }))
        .unwrap();
        assert_eq!(drafts.draft_list[0].draft_id, Some(100));
        assert_eq!(drafts.draft_list[0].user_version.as_deref(), Some("1.0.0"));
        assert_eq!(drafts.draft_list[0].extra["extra_field"], "kept");

        let templates: OpenPlatformTemplateListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "template_list": [{
                "template_id": "tpl",
                "user_version": "1.0.0",
                "user_desc": "template",
                "create_time": 1_800_000_001,
                "source_miniprogram_appid": "wx-source",
                "source_miniprogram": "Source",
                "developer": "dev",
                "template_type": 0,
                "extra_field": "kept"
            }]
        }))
        .unwrap();
        assert_eq!(
            templates.template_list[0].template_id.as_deref(),
            Some("tpl")
        );
        assert_eq!(
            templates.template_list[0].source_miniprogram.as_deref(),
            Some("Source")
        );
        assert_eq!(templates.template_list[0].extra["extra_field"], "kept");
    }

    #[test]
    fn serializes_authorizer_mini_program_code_requests() {
        let commit = serde_json::to_value(OpenPlatformMiniProgramCommitRequest::new(
            "100",
            "{\"extAppid\":\"wx\"}",
            "1.0.0",
            "release",
        ))
        .unwrap();
        assert_eq!(commit["template_id"], "100");
        assert_eq!(commit["user_version"], "1.0.0");
        assert_eq!(commit["user_desc"], "release");

        let audit = serde_json::to_value(OpenPlatformMiniProgramSubmitAuditRequest {
            item_list: vec![OpenPlatformMiniProgramAuditItem {
                address: "pages/index/index".to_string(),
                tag: "tool".to_string(),
                first_class: "tool".to_string(),
                second_class: "efficiency".to_string(),
                first_id: 1,
                second_id: 2,
                title: "home".to_string(),
                third_class: None,
                third_id: None,
                feedback_info: None,
                feedback_stuff: None,
                extra: serde_json::Value::Null,
            }],
            feedback_info: Some("feedback".to_string()),
            feedback_stuff: None,
            preview_info: Some(OpenPlatformMiniProgramAuditPreviewInfo {
                pic_id_list: vec!["pic".to_string()],
                video_id_list: Vec::new(),
                extra: serde_json::Value::Null,
            }),
            version_desc: Some("v1".to_string()),
            ugc_declare: Some(OpenPlatformMiniProgramUgcDeclare {
                scene: vec![1, 2],
                method: vec![1],
                has_audit_team: Some(1),
                audit_desc: Some("Content is reviewed before publishing".to_string()),
                extra: Value::Null,
            }),
            privacy_api_not_use: Some(true),
            order_path: None,
        })
        .unwrap();
        assert_eq!(audit["item_list"][0]["address"], "pages/index/index");
        assert_eq!(audit["preview_info"]["pic_id_list"][0], "pic");
        assert_eq!(audit["ugc_declare"]["scene"], json!([1, 2]));
        assert_eq!(audit["privacy_api_not_use"], true);
        assert!(audit.get("feedback_stuff").is_none());
    }

    #[test]
    fn validates_authorizer_mini_program_code_release_requests() {
        assert!(OpenPlatformMiniProgramCommitRequest::new(
            "100",
            "{\"extAppid\":\"wx\"}",
            "1.0.0",
            "release"
        )
        .validate()
        .is_ok());
        assert!(
            OpenPlatformMiniProgramCommitRequest::new("100", "[]", "1.0.0", "release")
                .validate()
                .is_err()
        );

        let item = OpenPlatformMiniProgramAuditItem {
            address: "pages/index/index".to_string(),
            tag: "tool".to_string(),
            first_class: "tool".to_string(),
            second_class: "efficiency".to_string(),
            first_id: 1,
            second_id: 2,
            title: "home".to_string(),
            third_class: None,
            third_id: None,
            feedback_info: None,
            feedback_stuff: None,
            extra: Value::Null,
        };
        let mut audit = OpenPlatformMiniProgramSubmitAuditRequest::new(vec![item.clone()]);
        audit.preview_info = Some(OpenPlatformMiniProgramAuditPreviewInfo {
            pic_id_list: vec!["pic-1".to_string()],
            video_id_list: vec!["video-1".to_string()],
            extra: Value::Null,
        });
        audit.ugc_declare = Some(OpenPlatformMiniProgramUgcDeclare {
            scene: vec![1, 2],
            method: vec![1],
            has_audit_team: Some(1),
            audit_desc: Some("Content is reviewed before publishing".to_string()),
            extra: Value::Null,
        });
        assert!(audit.validate().is_ok());

        let mut duplicate = audit.clone();
        duplicate.item_list.push(item.clone());
        assert!(duplicate.validate().is_err());

        let mut incomplete_category = OpenPlatformMiniProgramSubmitAuditRequest::new(vec![item]);
        incomplete_category.item_list[0].third_class = Some("utility".to_string());
        assert!(incomplete_category.validate().is_err());

        let mut invalid_ugc = audit;
        invalid_ugc.ugc_declare = Some(OpenPlatformMiniProgramUgcDeclare {
            scene: vec![1, 1],
            method: vec![1],
            has_audit_team: Some(1),
            audit_desc: None,
            extra: Value::Null,
        });
        assert!(invalid_ugc.validate().is_err());

        assert_eq!(
            serde_json::to_value(OpenPlatformMiniProgramVisitStatusAction::Open).unwrap(),
            "open"
        );
        assert_eq!(
            serde_json::to_value(OpenPlatformMiniProgramVisitStatusAction::Close).unwrap(),
            "close"
        );
    }

    #[test]
    fn deserializes_authorizer_mini_program_code_responses() {
        let categories: OpenPlatformMiniProgramCategoryResponse = serde_json::from_value(json!({
            "category_list": [{ "first_class": "tool", "first_id": 1, "audit_status": 3, "extra_field": "kept" }],
            "request_id": "category"
        }))
        .unwrap();
        assert_eq!(categories.category_list[0].first_id, Some(1));
        assert_eq!(
            categories.category_list[0].first_class.as_deref(),
            Some("tool")
        );
        assert_eq!(
            categories.category_list[0].audit_state(),
            Some(OpenPlatformMiniProgramCategoryAuditState::Approved)
        );
        assert!(categories.category_list[0].is_audit_approved());
        assert_eq!(categories.category_list[0].extra["extra_field"], "kept");
        assert_eq!(categories.extra["request_id"], "category");

        let pages: OpenPlatformMiniProgramPageResponse = serde_json::from_value(json!({
            "page_list": ["pages/index/index"],
            "request_id": "pages"
        }))
        .unwrap();
        assert_eq!(pages.page_list[0], "pages/index/index");
        assert_eq!(pages.extra["request_id"], "pages");

        let submitted: OpenPlatformMiniProgramSubmitAuditResponse = serde_json::from_value(json!({
            "type": "audit",
            "mediaid": "media",
            "auditid": 123,
            "request_id": "submit"
        }))
        .unwrap();
        assert_eq!(submitted.audit_type.as_deref(), Some("audit"));
        assert_eq!(submitted.auditid, Some(123));
        assert_eq!(submitted.require_audit_id().unwrap(), 123);
        assert_eq!(submitted.extra["request_id"], "submit");

        let status: OpenPlatformMiniProgramAuditStatusResponse = serde_json::from_value(json!({
            "status": 0,
            "reason": "ok",
            "screenshot": "https://example.com/s.png",
            "request_id": "status"
        }))
        .unwrap();
        assert_eq!(status.status, Some(0));
        assert_eq!(
            status.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Approved)
        );
        assert!(status.is_audit_approved());
        assert!(!status.is_audit_rejected());
        assert!(status.audit_state().expect("state").is_terminal());
        assert!(status.audit_state().expect("state").can_release());
        status.ensure_releasable().unwrap();
        status.validate().unwrap();
        assert_eq!(status.reason.as_deref(), Some("ok"));
        assert_eq!(status.extra["request_id"], "status");

        let latest: OpenPlatformMiniProgramLatestAuditStatusResponse =
            serde_json::from_value(json!({
                "auditid": 124,
                "status": 0,
                "ScreenShot": "https://example.com/latest.png",
                "user_version": "1.0.0",
                "submit_audit_time": 1800000000,
                "request_id": "latest"
            }))
            .unwrap();
        assert_eq!(latest.auditid, Some(124));
        assert_eq!(
            latest.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Approved)
        );
        assert!(latest.is_audit_approved());
        assert!(!latest.is_audit_rejected());
        assert_eq!(latest.ensure_releasable().unwrap(), 124);
        latest.validate().unwrap();
        assert_eq!(
            latest.screenshot.as_deref(),
            Some("https://example.com/latest.png")
        );
        assert_eq!(latest.extra["request_id"], "latest");
        let rejected: OpenPlatformMiniProgramAuditStatusResponse =
            serde_json::from_value(json!({ "status": 1, "reason": "bad content" })).unwrap();
        assert_eq!(
            rejected.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Rejected)
        );
        assert!(rejected.is_audit_rejected());
        assert_eq!(rejected.rejection_reason(), Some("bad content"));
        assert!(rejected.audit_state().expect("state").needs_attention());
        assert!(rejected.ensure_releasable().is_err());
        let auditing: OpenPlatformMiniProgramAuditStatusResponse =
            serde_json::from_value(json!({ "status": 2 })).unwrap();
        assert_eq!(
            auditing.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Auditing)
        );
        let withdrawn: OpenPlatformMiniProgramLatestAuditStatusResponse =
            serde_json::from_value(json!({ "status": 3 })).unwrap();
        assert_eq!(
            withdrawn.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Withdrawn)
        );
        let unknown: OpenPlatformMiniProgramAuditStatusResponse =
            serde_json::from_value(json!({ "status": 99 })).unwrap();
        assert_eq!(
            unknown.audit_state(),
            Some(OpenPlatformMiniProgramAuditState::Other)
        );

        let rollback: OpenPlatformMiniProgramRollbackReleaseResponse =
            serde_json::from_value(json!({
                "version_list": [{
                    "commit_time": 1800000000,
                    "user_version": "1.0.0",
                    "app_version": 1
                }],
                "request_id": "rollback"
            }))
            .unwrap();
        assert_eq!(
            rollback.version_list[0].user_version.as_deref(),
            Some("1.0.0")
        );
        assert_eq!(rollback.version_list[0].commit_time, Some(1800000000));
        assert_eq!(rollback.version_list[0].app_version, Some(1));
        rollback.validate().unwrap();
        assert_eq!(
            rollback
                .latest()
                .and_then(|version| version.release_label()),
            Some("1.0.0")
        );
        assert_eq!(
            rollback
                .find_app_version(1)
                .and_then(|version| version.app_version),
            Some(1)
        );
        assert_eq!(rollback.extra["request_id"], "rollback");

        let gray: OpenPlatformMiniProgramGrayReleasePlanResponse = serde_json::from_value(json!({
            "gray_release_plan": { "status": 1, "gray_percentage": 10, "plan_extra": "kept" },
            "request_id": "gray"
        }))
        .unwrap();
        assert_eq!(gray.extra["request_id"], "gray");
        gray.validate().unwrap();
        assert!(gray.is_active());
        let gray_plan = gray.gray_release_plan.unwrap();
        assert_eq!(gray_plan.gray_percentage, Some(10));
        assert_eq!(
            gray_plan.release_state(),
            Some(OpenPlatformMiniProgramGrayReleaseState::Running)
        );
        assert!(gray_plan.release_state().expect("state").is_active());
        assert_eq!(gray_plan.extra["plan_extra"], "kept");
        assert!(OpenPlatformMiniProgramGrayReleaseState::Finished.is_terminal());
        assert!(OpenPlatformMiniProgramGrayReleaseState::Deleted.is_terminal());
        assert_eq!(
            OpenPlatformMiniProgramGrayReleaseState::from(99),
            OpenPlatformMiniProgramGrayReleaseState::Other
        );

        let support: OpenPlatformMiniProgramSupportVersionResponse =
            serde_json::from_value(json!({
                "now_version": "3.0.0",
                "uv_info": {
                    "items": [{
                        "visit_uv": 90,
                        "percentage": 90,
                        "version": "3.0.0",
                        "uv_extra": "kept"
                    }],
                    "uv_total": 1
                },
                "request_id": "support"
            }))
            .unwrap();
        support.validate().unwrap();
        assert_eq!(support.current_version(), Some("3.0.0"));
        assert_eq!(support.now_version.as_deref(), Some("3.0.0"));
        assert_eq!(support.extra["request_id"], "support");
        let uv_info = support.uv_info.unwrap();
        assert_eq!(uv_info.extra["uv_total"], 1);
        assert_eq!(uv_info.items[0].version.as_deref(), Some("3.0.0"));
        assert_eq!(uv_info.items[0].percentage, Some(90));
        assert_eq!(uv_info.items[0].extra["uv_extra"], "kept");

        let quota: OpenPlatformMiniProgramAuditQuotaResponse = serde_json::from_value(json!({
            "rest": 10,
            "limit": 100,
            "speedup_rest": 1,
            "speedup_limit": 10,
            "request_id": "quota"
        }))
        .unwrap();
        assert_eq!(quota.rest, Some(10));
        assert_eq!(quota.speedup_limit, Some(10));
        quota.validate().unwrap();
        assert!(quota.can_submit());
        assert!(quota.can_speedup());
        assert_eq!(quota.used(), Some(90));
        assert_eq!(quota.speedup_used(), Some(9));
        assert_eq!(quota.extra["request_id"], "quota");
    }

    #[test]
    fn rejects_inconsistent_authorizer_release_workflow_responses() {
        let missing_id: OpenPlatformMiniProgramSubmitAuditResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_id.require_audit_id().is_err());

        let rejected_without_reason: OpenPlatformMiniProgramAuditStatusResponse =
            serde_json::from_value(json!({ "status": 1 })).unwrap();
        let reason_error = rejected_without_reason
            .validate()
            .expect_err("rejected audit without reason must fail");
        assert!(reason_error.to_string().contains("reason is missing"));

        let auditing: OpenPlatformMiniProgramLatestAuditStatusResponse =
            serde_json::from_value(json!({
                "auditid": 1,
                "status": 2,
                "submit_audit_time": 1_800_000_000
            }))
            .unwrap();
        assert!(auditing.validate().is_ok());
        assert!(auditing.ensure_releasable().is_err());

        let invalid_quota: OpenPlatformMiniProgramAuditQuotaResponse =
            serde_json::from_value(json!({
                "rest": 11,
                "limit": 10,
                "speedup_rest": -1,
                "speedup_limit": 1
            }))
            .unwrap();
        assert!(invalid_quota.validate().is_err());

        let duplicate_versions: OpenPlatformMiniProgramRollbackReleaseResponse =
            serde_json::from_value(json!({
                "version_list": [
                    {"app_version": 1, "commit_time": 10, "user_version": "1.0.0"},
                    {"app_version": 1, "commit_time": 20, "user_version": "1.0.1"}
                ]
            }))
            .unwrap();
        assert!(duplicate_versions.validate().is_err());

        let invalid_release_version: OpenPlatformMiniProgramRollbackReleaseResponse =
            serde_json::from_value(json!({
                "version_list": [{
                    "app_version": 2,
                    "percentage": 101,
                    "user_version": "2.0.0"
                }]
            }))
            .unwrap();
        assert!(invalid_release_version.validate().is_err());

        let invalid_gray: OpenPlatformMiniProgramGrayReleasePlanResponse =
            serde_json::from_value(json!({
                "gray_release_plan": {"status": 1, "gray_percentage": 0}
            }))
            .unwrap();
        assert!(invalid_gray.validate().is_err());

        let invalid_support: OpenPlatformMiniProgramSupportVersionResponse =
            serde_json::from_value(json!({
                "now_version": "3.0.0",
                "uv_info": {
                    "items": [{"visit_uv": 1, "percentage": 101, "version": "3.0.0"}]
                }
            }))
            .unwrap();
        assert!(invalid_support.validate().is_err());
    }

    #[test]
    fn serializes_authorizer_mini_program_domain_and_tester_requests() {
        let domain = serde_json::to_value(OpenPlatformMiniProgramModifyDomainRequest {
            action: OpenPlatformMiniProgramDomainAction::Set,
            requestdomain: vec!["https://api.example.com".to_string()],
            wsrequestdomain: Vec::new(),
            uploaddomain: vec!["https://upload.example.com".to_string()],
            downloaddomain: Vec::new(),
            udpdomain: Vec::new(),
            tcpdomain: Vec::new(),
        })
        .unwrap();
        assert_eq!(domain["action"], "set");
        assert_eq!(domain["requestdomain"][0], "https://api.example.com");
        assert!(domain.get("wsrequestdomain").is_none());

        let domain_response: OpenPlatformMiniProgramModifyDomainResponse =
            serde_json::from_value(json!({
                "requestdomain": ["https://api.example.com"],
                "invalid_requestdomain": [{ "domain": "bad-domain", "reason": "invalid" }],
                "no_icp_domain": ["https://no-icp.example.com"]
            }))
            .unwrap();
        assert_eq!(
            domain_response.requestdomain[0].domain.as_deref(),
            Some("https://api.example.com")
        );
        assert_eq!(
            domain_response.invalid_requestdomain[0].domain.as_deref(),
            Some("bad-domain")
        );
        assert_eq!(
            domain_response.invalid_requestdomain[0].reason.as_deref(),
            Some("invalid")
        );
        assert_eq!(domain_response.configured_domains().len(), 1);
        assert_eq!(domain_response.invalid_domains().len(), 1);
        assert!(domain_response.has_rejected_domains());
        assert!(domain_response.validate().is_ok());
        assert!(domain_response.ensure_applied().is_err());

        let unbind = serde_json::to_value(OpenPlatformMiniProgramTesterUnbindRequest {
            wechatid: None,
            userstr: Some("userstr".to_string()),
        })
        .unwrap();
        assert_eq!(unbind["userstr"], "userstr");
        assert!(unbind.get("wechatid").is_none());

        let bind: OpenPlatformMiniProgramTesterBindResponse =
            serde_json::from_value(json!({ "userstr": "userstr" })).unwrap();
        assert_eq!(bind.userstr.as_deref(), Some("userstr"));
        assert_eq!(bind.require_userstr().unwrap(), "userstr");

        let list: OpenPlatformMiniProgramTesterListResponse = serde_json::from_value(json!({
            "members": [{ "wechatid": "tester", "userstr": "userstr" }]
        }))
        .unwrap();
        assert_eq!(list.members[0].wechatid.as_deref(), Some("tester"));
        assert!(list.validate().is_ok());
        assert_eq!(
            list.find_by_userstr("userstr")
                .and_then(|member| member.wechatid.as_deref()),
            Some("tester")
        );
        assert!(list.find_by_wechat_id("tester").is_some());
    }

    #[test]
    fn serializes_authorizer_mini_program_privacy_requests() {
        let privacy = serde_json::to_value(OpenPlatformMiniProgramPrivacySettingRequest {
            owner_setting: OpenPlatformMiniProgramPrivacyOwnerSetting {
                contact_email: Some("dev@example.com".to_string()),
                contact_phone: None,
                contact_qq: None,
                contact_weixin: None,
                ext_file_media_id: Some("privacy-media".to_string()),
                notice_method: Some("email".to_string()),
                store_expire_timestamp: Some("1735689600".to_string()),
                extra: serde_json::Value::Null,
            },
            setting_list: vec![OpenPlatformMiniProgramPrivacySettingItem {
                privacy_key: Some("UserInfo".to_string()),
                privacy_text: Some("user info".to_string()),
                privacy_label: None,
                privacy_desc: None,
                privacy_url: None,
                extra: serde_json::Value::Null,
            }],
            sdk_privacy_info_list: vec![OpenPlatformMiniProgramPrivacySdkInfo {
                sdk_name: "analytics-sdk".to_string(),
                sdk_biz_name: "Analytics".to_string(),
                sdk_list: vec![OpenPlatformMiniProgramPrivacySdkItem {
                    privacy_key: "DeviceInfo".to_string(),
                    privacy_text: "device info".to_string(),
                    privacy_label: None,
                    extra: Value::Null,
                }],
            }],
            privacy_ver: Some(2),
        })
        .unwrap();
        assert_eq!(privacy["owner_setting"]["contact_email"], "dev@example.com");
        assert_eq!(privacy["setting_list"][0]["privacy_key"], "UserInfo");
        assert_eq!(
            privacy["sdk_privacy_info_list"][0]["sdk_list"][0]["privacy_key"],
            "DeviceInfo"
        );
        assert_eq!(privacy["privacy_ver"], 2);

        let response: OpenPlatformMiniProgramPrivacySettingResponse =
            serde_json::from_value(json!({
                "owner_setting": { "contact_email": "dev@example.com" },
                "setting_list": [{ "privacy_key": "UserInfo" }],
                "sdk_privacy_info_list": [{
                    "sdk_name": "analytics-sdk",
                    "sdk_biz_name": "Analytics",
                    "sdk_list": [{
                        "privacy_key": "DeviceInfo",
                        "privacy_text": "device info",
                        "privacy_label": "device information"
                    }]
                }],
                "privacy_list": ["UserInfo"],
                "privacy_desc": {
                    "privacy_desc_list": [{
                        "privacy_key": "UserInfo",
                        "privacy_desc": "member profile",
                        "description_source": "wechat"
                    }]
                },
                "code_exist": 1,
                "update_time": 1735689600,
                "privacy_ver": 2,
                "request_id": "privacy-setting"
            }))
            .unwrap();
        assert!(response.validate().is_ok());
        assert!(response.is_code_configured());
        assert!(response.undeclared_code_privacy_keys().is_empty());
        assert!(response.ensure_ready_for_submission().is_ok());
        assert_eq!(
            response
                .find_setting("UserInfo")
                .and_then(|item| item.privacy_key.as_deref()),
            Some("UserInfo")
        );
        assert_eq!(
            response.owner_setting.unwrap().contact_email.as_deref(),
            Some("dev@example.com")
        );
        assert_eq!(
            response.setting_list[0].privacy_key.as_deref(),
            Some("UserInfo")
        );
        assert_eq!(response.sdk_privacy_info_list[0].sdk_name, "analytics-sdk");
        assert_eq!(
            response.sdk_privacy_info_list[0].sdk_list[0]
                .privacy_label
                .as_deref(),
            Some("device information")
        );
        assert_eq!(response.privacy_list, vec!["UserInfo"]);
        assert_eq!(response.code_exist, Some(1));
        assert_eq!(
            response.privacy_desc.as_ref().unwrap().privacy_desc_list[0].extra
                ["description_source"],
            "wechat"
        );
        assert_eq!(response.extra["request_id"], "privacy-setting");

        let upload: OpenPlatformMiniProgramPrivacyExtFileResponse =
            serde_json::from_value(json!({ "ext_file_media_id": "media" })).unwrap();
        assert_eq!(upload.ext_file_media_id.as_deref(), Some("media"));
        assert_eq!(upload.require_media_id().unwrap(), "media");

        let apply = serde_json::to_value(OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
            api_name: "getUserInfo".to_string(),
            content: "reason".to_string(),
            url: Some("https://example.com/privacy".to_string()),
            scene: Some(1),
        })
        .unwrap();
        assert_eq!(apply["api_name"], "getUserInfo");
        assert_eq!(apply["url"], "https://example.com/privacy");

        let interfaces: OpenPlatformMiniProgramPrivacyInterfaceResponse =
            serde_json::from_value(json!({
                "interface_list": [
                    { "api_name": "getUserInfo", "status": 2, "scope": "profile" },
                    { "api_name": "chooseAddress", "status": 5 },
                    { "api_name": "getPhoneNumber", "status": 1 },
                    { "api_name": "openSetting", "status": 3 },
                    { "api_name": "chooseInvoiceTitle", "status": 4 },
                    { "api_name": "unknown", "status": 99 }
                ],
                "request_id": "privacy-interface"
            }))
            .unwrap();
        assert_eq!(
            interfaces.interface_list[0].api_name.as_deref(),
            Some("getUserInfo")
        );
        assert_eq!(interfaces.interface_list[0].status, Some(2));
        assert_eq!(
            interfaces.interface_list[0].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::NoPermission)
        );
        assert!(interfaces.interface_list[0].needs_apply());
        assert!(!interfaces.interface_list[0].is_approved());
        assert_eq!(
            interfaces.interface_list[1].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::Approved)
        );
        assert!(interfaces.interface_list[1].is_approved());
        assert!(!interfaces.interface_list[1].needs_apply());
        assert_eq!(
            interfaces.interface_list[2].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::PendingApply)
        );
        assert!(interfaces.interface_list[2].needs_apply());
        assert_eq!(
            interfaces.interface_list[3].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::Applying)
        );
        assert!(!interfaces.interface_list[3].needs_apply());
        assert_eq!(
            interfaces.interface_list[4].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::Rejected)
        );
        assert!(interfaces.interface_list[4].needs_apply());
        assert_eq!(
            interfaces.interface_list[5].interface_state(),
            Some(OpenPlatformMiniProgramPrivacyInterfaceState::Other)
        );
        assert!(interfaces.validate().is_ok());
        assert_eq!(interfaces.requiring_action().len(), 3);
        assert!(interfaces.find("chooseAddress").unwrap().is_approved());
        assert!(!interfaces.all_approved());
        assert_eq!(interfaces.interface_list[0].extra["scope"], "profile");
        assert_eq!(interfaces.extra["request_id"], "privacy-interface");

        let apply_response: OpenPlatformMiniProgramPrivacyInterfaceApplyResponse =
            serde_json::from_value(json!({ "audit_id": 10, "request_id": "apply-10" })).unwrap();
        assert_eq!(apply_response.audit_id, Some(10));
        assert_eq!(apply_response.require_audit_id().unwrap(), 10);
        assert_eq!(apply_response.extra["request_id"], "apply-10");
    }

    #[test]
    fn rejects_inconsistent_authorizer_domain_tester_and_privacy_responses() {
        let api_error: OpenPlatformMiniProgramModifyDomainResponse =
            serde_json::from_value(json!({ "errcode": 85015, "errmsg": "domain rejected" }))
                .unwrap();
        assert!(matches!(api_error.validate(), Err(WechatError::Api { .. })));

        let missing_domain: OpenPlatformMiniProgramModifyDomainResponse =
            serde_json::from_value(json!({ "invalid_requestdomain": [{ "reason": "invalid" }] }))
                .unwrap();
        assert!(missing_domain.validate().is_err());

        let duplicate_testers: OpenPlatformMiniProgramTesterListResponse =
            serde_json::from_value(json!({
                "members": [
                    { "userstr": "member-1", "wechatid": "tester" },
                    { "userstr": "member-1", "wechatid": "tester-2" }
                ]
            }))
            .unwrap();
        assert!(duplicate_testers.validate().is_err());
        assert!(OpenPlatformMiniProgramTesterBindResponse {
            errcode: Some(0),
            errmsg: None,
            userstr: None,
            extra: Value::Null,
        }
        .require_userstr()
        .is_err());

        let undeclared_privacy: OpenPlatformMiniProgramPrivacySettingResponse =
            serde_json::from_value(json!({
                "code_exist": 1,
                "privacy_list": ["UserInfo", "Location"],
                "setting_list": [{ "privacy_key": "UserInfo" }]
            }))
            .unwrap();
        assert_eq!(
            undeclared_privacy.undeclared_code_privacy_keys(),
            vec!["Location"]
        );
        assert!(undeclared_privacy.ensure_ready_for_submission().is_err());

        let duplicate_interfaces: OpenPlatformMiniProgramPrivacyInterfaceResponse =
            serde_json::from_value(json!({
                "interface_list": [
                    { "api_name": "getUserInfo", "status": 1 },
                    { "api_name": "getUserInfo", "status": 5 }
                ]
            }))
            .unwrap();
        assert!(duplicate_interfaces.validate().is_err());

        let missing_audit_id: OpenPlatformMiniProgramPrivacyInterfaceApplyResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_audit_id.require_audit_id().is_err());
    }

    #[test]
    fn validates_authorizer_mini_program_domain_tester_and_privacy_requests() {
        let valid_domain = OpenPlatformMiniProgramModifyDomainRequest {
            action: OpenPlatformMiniProgramDomainAction::Add,
            requestdomain: vec!["https://api.example.com".to_string()],
            wsrequestdomain: vec!["wss://socket.example.com".to_string()],
            uploaddomain: Vec::new(),
            downloaddomain: Vec::new(),
            udpdomain: vec!["udp://network.example.com:443".to_string()],
            tcpdomain: vec!["tcp://network.example.com:443".to_string()],
        };
        assert!(valid_domain.validate().is_ok());

        let mut invalid_domain = valid_domain.clone();
        invalid_domain.action = OpenPlatformMiniProgramDomainAction::Get;
        assert!(invalid_domain.validate().is_err());
        invalid_domain.action = OpenPlatformMiniProgramDomainAction::Add;
        invalid_domain.wsrequestdomain = vec!["https://socket.example.com".to_string()];
        assert!(invalid_domain.validate().is_err());

        assert!(
            OpenPlatformMiniProgramTesterUnbindRequest::by_wechat_id("tester")
                .validate()
                .is_ok()
        );
        assert!(OpenPlatformMiniProgramTesterUnbindRequest {
            wechatid: Some("tester".to_string()),
            userstr: Some("userstr".to_string()),
        }
        .validate()
        .is_err());

        let valid_privacy = OpenPlatformMiniProgramPrivacySettingRequest {
            owner_setting: OpenPlatformMiniProgramPrivacyOwnerSetting {
                contact_email: Some("dev@example.com".to_string()),
                contact_phone: None,
                contact_qq: None,
                contact_weixin: None,
                ext_file_media_id: None,
                notice_method: Some("email".to_string()),
                store_expire_timestamp: None,
                extra: Value::Null,
            },
            setting_list: vec![OpenPlatformMiniProgramPrivacySettingItem {
                privacy_key: Some("UserInfo".to_string()),
                privacy_text: Some("user info".to_string()),
                privacy_label: None,
                privacy_desc: None,
                privacy_url: Some("https://example.com/privacy".to_string()),
                extra: Value::Null,
            }],
            sdk_privacy_info_list: vec![OpenPlatformMiniProgramPrivacySdkInfo {
                sdk_name: "analytics-sdk".to_string(),
                sdk_biz_name: "Analytics".to_string(),
                sdk_list: vec![OpenPlatformMiniProgramPrivacySdkItem {
                    privacy_key: "DeviceInfo".to_string(),
                    privacy_text: "device info".to_string(),
                    privacy_label: None,
                    extra: Value::Null,
                }],
            }],
            privacy_ver: Some(2),
        };
        assert!(valid_privacy.validate().is_ok());

        let mut invalid_privacy = valid_privacy.clone();
        invalid_privacy.setting_list[0].privacy_text = None;
        assert!(invalid_privacy.validate().is_err());

        assert!(OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
            api_name: "getUserInfo".to_string(),
            content: "Used to display the member profile".to_string(),
            url: Some("https://example.com/privacy".to_string()),
            scene: Some(1),
        }
        .validate()
        .is_ok());
        assert!(OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
            api_name: String::new(),
            content: "reason".to_string(),
            url: Some("javascript:alert(1)".to_string()),
            scene: Some(0),
        }
        .validate()
        .is_err());
    }

    #[test]
    fn builds_open_platform_authorization_urls() {
        let component_url =
            OpenPlatform::component_login_page_url(OpenPlatformComponentLoginPageUrlRequest {
                component_appid: "component".to_string(),
                pre_auth_code: "preauth".to_string(),
                redirect_uri: "https://example.com/callback".to_string(),
                extra_query: vec![("auth_type".to_string(), "3".to_string())],
            });
        assert!(component_url.starts_with("https://mp.weixin.qq.com/cgi-bin/componentloginpage?"));
        assert!(component_url.contains("component_appid=component"));
        assert!(component_url.contains("pre_auth_code=preauth"));
        assert!(component_url.contains("auth_type=3"));

        let fast_url = OpenPlatform::official_account_fast_registration_url(
            OpenPlatformOfficialAccountFastRegistrationUrlRequest {
                component_appid: "component".to_string(),
                appid: "wx-authorizer".to_string(),
                redirect_uri: "https://example.com/fast".to_string(),
                copy_wx_verify: true,
            },
        )
        .unwrap();
        assert!(fast_url.starts_with("https://mp.weixin.qq.com/cgi-bin/fastregisterauth?"));
        assert!(fast_url.contains("copy_wx_verify=true"));
        assert!(fast_url.contains("appid=wx-authorizer"));
        assert!(OpenPlatform::official_account_fast_registration_url(
            OpenPlatformOfficialAccountFastRegistrationUrlRequest {
                component_appid: "component".to_string(),
                appid: "wx-authorizer".to_string(),
                redirect_uri: "http://example.com/fast#fragment".to_string(),
                copy_wx_verify: false,
            }
        )
        .is_err());
    }

    #[test]
    fn serializes_authorizer_account_and_open_account_requests() {
        let head_request =
            OpenPlatformAuthorizerAccountHeadImageRequest::from_crop("media", 0.0, 0.0, 1.0, 1.0);
        assert!(head_request.validate().is_ok());
        let head = serde_json::to_value(head_request).unwrap();
        assert_eq!(head["head_img_media_id"], "media");
        assert_eq!(head["x2"], "1");

        let basic: OpenPlatformAuthorizerAccountBasicInfoResponse = serde_json::from_value(json!({
            "appid": "wx-authorizer",
            "account_type": 1,
            "principal_type": 1,
            "principal_name": "principal",
            "realname_status": 1,
            "wx_verify_info": {
                "qualification_verify": true,
                "naming_verify": false,
                "annual_review": true,
                "annual_review_begin_time": 1800000000,
                "annual_review_end_time": 1800100000,
                "verify_extra": "retained"
            },
            "signature_info": {
                "signature": "hello",
                "modify_used_count": 1,
                "modify_quota": 5,
                "signature_extra": "retained"
            },
            "head_image_info": {
                "head_image_url": "https://example.com/head.png",
                "modify_used_count": 2,
                "modify_quota": 5,
                "image_extra": "retained"
            },
            "nickname": "Authorizer",
            "registered_country": 86,
            "nickname_info": {
                "nickname": "Authorizer",
                "modify_used_count": 1,
                "modify_quota": 5,
                "nickname_extra": "retained"
            },
            "credential": "credential",
            "customer_type": 1,
            "account_extra": "retained"
        }))
        .unwrap();
        assert!(basic.validate().is_ok());
        assert_eq!(basic.appid.as_deref(), Some("wx-authorizer"));
        assert_eq!(
            basic
                .wx_verify_info
                .as_ref()
                .and_then(|info| info.qualification_verify),
            Some(true)
        );
        assert_eq!(
            basic.wx_verify_info.as_ref().unwrap().extra["verify_extra"],
            "retained"
        );
        assert_eq!(
            basic
                .signature_info
                .as_ref()
                .and_then(|info| info.signature.as_deref()),
            Some("hello")
        );
        assert_eq!(
            basic.signature_info.as_ref().unwrap().modify_used_count,
            Some(1)
        );
        assert_eq!(
            basic.signature_info.as_ref().unwrap().extra["signature_extra"],
            "retained"
        );
        assert_eq!(
            basic
                .head_image_info
                .as_ref()
                .and_then(|info| info.head_image_url.as_deref()),
            Some("https://example.com/head.png")
        );
        assert_eq!(
            basic.head_image_info.as_ref().unwrap().extra["image_extra"],
            "retained"
        );
        assert_eq!(basic.nickname.as_deref(), Some("Authorizer"));
        assert_eq!(basic.registered_country, Some(86));
        assert_eq!(
            basic.nickname_info.as_ref().unwrap().extra["nickname_extra"],
            "retained"
        );
        assert_eq!(basic.credential.as_deref(), Some("credential"));
        assert_eq!(basic.customer_type, Some(1));
        assert_eq!(basic.extra["account_extra"], "retained");

        let open: OpenPlatformOpenAccountResponse = serde_json::from_value(json!({
            "open_appid": "open-appid",
            "request_id": "open-1"
        }))
        .unwrap();
        assert!(open.validate().is_ok());
        assert_eq!(open.open_appid.as_deref(), Some("open-appid"));
        assert_eq!(open.extra["request_id"], "open-1");

        let registration: OpenPlatformOfficialAccountFastRegistrationResponse =
            serde_json::from_value(json!({
                "appid": "wx-mini-program",
                "authorization_code": "authorization-code",
                "is_wx_verify_succ": "1",
                "is_link_succ": true,
                "request_id": "register-1"
            }))
            .unwrap();
        assert!(registration.validate().is_ok());
        assert!(registration.is_verified().unwrap());
        assert!(registration.is_linked().unwrap());
        assert_eq!(registration.extra["request_id"], "register-1");
        assert!(!OpenPlatformBinaryFlag::Integer(0).value().unwrap());
    }

    #[test]
    fn rejects_invalid_authorizer_account_contracts() {
        assert!(OpenPlatformAuthorizerAccountHeadImageRequest::from_crop(
            "media", 0.8, 0.0, 0.2, 1.0
        )
        .validate()
        .is_err());
        assert!(OpenPlatformAuthorizerAccountHeadImageRequest::from_crop(
            "media",
            f64::NAN,
            0.0,
            1.0,
            1.0
        )
        .validate()
        .is_err());

        let missing_appid: OpenPlatformAuthorizerAccountBasicInfoResponse =
            serde_json::from_value(json!({ "account_type": 1 })).unwrap();
        assert!(missing_appid.validate().is_err());

        let invalid_quota: OpenPlatformAuthorizerAccountBasicInfoResponse =
            serde_json::from_value(json!({
                "appid": "wx-authorizer",
                "signature_info": {
                    "signature": "hello",
                    "modify_used_count": 6,
                    "modify_quota": 5
                }
            }))
            .unwrap();
        assert!(invalid_quota.validate().is_err());

        let api_error: OpenPlatformAuthorizerAccountBasicInfoResponse =
            serde_json::from_value(json!({ "errcode": 40001, "errmsg": "invalid" })).unwrap();
        assert!(matches!(api_error.validate(), Err(WechatError::Api { .. })));

        let incomplete_registration: OpenPlatformOfficialAccountFastRegistrationResponse =
            serde_json::from_value(json!({
                "appid": "wx-mini-program",
                "is_wx_verify_succ": "yes",
                "is_link_succ": 1
            }))
            .unwrap();
        assert!(incomplete_registration.validate().is_err());

        let missing_open_appid: OpenPlatformOpenAccountResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_open_appid.validate().is_err());

        let incomplete_session: OpenPlatformMiniProgramSessionResponse =
            serde_json::from_value(json!({ "openid": "openid" })).unwrap();
        assert!(incomplete_session.validate().is_err());

        let status: OpenPlatformStatusResponse =
            serde_json::from_value(json!({ "errcode": 40002, "errmsg": "denied" })).unwrap();
        assert!(matches!(
            status.validate_for("authorizer operation"),
            Err(WechatError::Api { .. })
        ));
    }

    #[test]
    fn serializes_authorizer_material_requests() {
        let article = serde_json::to_value(Article {
            title: "Release notes".to_string(),
            thumb_media_id: "thumb-media".to_string(),
            author: "Roze".to_string(),
            digest: "Production update".to_string(),
            show_cover_pic: 1,
            content: "<p>Ready</p>".to_string(),
            content_source_url: "https://example.com/releases/1".to_string(),
            need_open_comment: Some(1),
            only_fans_can_comment: Some(0),
        })
        .unwrap();
        assert_eq!(article["thumb_media_id"], "thumb-media");
        assert_eq!(article["show_cover_pic"], 1);
        assert_eq!(article["need_open_comment"], 1);

        let list =
            serde_json::to_value(MaterialListRequest::new(MaterialListKind::News, 0, 20)).unwrap();
        assert_eq!(list["type"], "news");
        assert_eq!(list["count"], 20);
    }

    #[test]
    fn deserializes_authorizer_material_responses() {
        let uploaded: MaterialMediaResponse = serde_json::from_value(json!({
            "media_id": "media-1",
            "url": "https://example.com/material/1",
            "request_id": "upload-1"
        }))
        .unwrap();
        assert_eq!(uploaded.media_id.as_deref(), Some("media-1"));
        assert_eq!(uploaded.require_media_id().unwrap(), "media-1");
        assert!(uploaded.validate().is_ok());
        assert_eq!(uploaded.extra["request_id"], "upload-1");

        let material: MaterialGetResponse = serde_json::from_value(json!({
            "news_item": [{
                "title": "Release notes",
                "author": "Roze",
                "thumb_media_id": "thumb-media",
                "content": "<p>Ready</p>",
                "content_source_url": "https://example.com/releases/1",
                "thumb_url": "https://example.com/thumb.png"
            }],
            "request_id": "get-1"
        }))
        .unwrap();
        assert!(material.validate().is_ok());
        assert_eq!(
            material.news_item[0].title.as_deref(),
            Some("Release notes")
        );
        assert_eq!(material.extra["request_id"], "get-1");

        let materials: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 1,
            "item": [{
                "media_id": "media-1",
                "name": "release",
                "content": {
                    "news_item": [{
                        "title": "Release notes",
                        "thumb_media_id": "thumb-media"
                    }],
                    "update_time": 2
                },
                "item_revision": 3
            }],
            "next_offset": 1
        }))
        .unwrap();
        assert_eq!(materials.total_count, Some(2));
        assert_eq!(materials.item[0].media_id.as_deref(), Some("media-1"));
        assert_eq!(
            materials.item[0].news_content().unwrap().unwrap().news_item[0]
                .title
                .as_deref(),
            Some("Release notes")
        );
        assert_eq!(materials.item[0].extra["item_revision"], 3);
        assert_eq!(materials.extra["next_offset"], 1);
        assert!(materials.validate().is_ok());
        assert_eq!(materials.next_offset(0).unwrap(), Some(1));
        assert!(materials.find("media-1").is_some());

        let stats: MaterialStatsResponse = serde_json::from_value(json!({
            "voice_count": 1,
            "video_count": 2,
            "image_count": 3,
            "news_count": 4,
            "request_id": "count-1"
        }))
        .unwrap();
        assert_eq!(stats.news_count, Some(4));
        assert_eq!(stats.total_count().unwrap(), 10);
        assert_eq!(stats.extra["request_id"], "count-1");
    }

    #[test]
    fn rejects_inconsistent_authorizer_material_responses() {
        let missing_media: MaterialMediaResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_media.require_media_id().is_err());

        let mismatched_page: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 2,
            "item": [{ "media_id": "media-1" }]
        }))
        .unwrap();
        assert!(mismatched_page.validate().is_err());

        let duplicate_page: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 2,
            "item": [
                { "media_id": "media-1" },
                { "media_id": "media-1" }
            ]
        }))
        .unwrap();
        assert!(duplicate_page.validate().is_err());

        let stalled_page: MaterialListResponse = serde_json::from_value(json!({
            "total_count": 2,
            "item_count": 0,
            "item": []
        }))
        .unwrap();
        assert!(stalled_page.next_offset(0).is_err());

        let invalid_stats: MaterialStatsResponse =
            serde_json::from_value(json!({ "image_count": -1 })).unwrap();
        assert!(invalid_stats.validate().is_err());
    }

    #[test]
    fn deserializes_authorizer_session_and_privacy_interface_responses() {
        let session: OpenPlatformMiniProgramSessionResponse = serde_json::from_value(json!({
            "openid": "openid",
            "session_key": "session",
            "unionid": "union"
        }))
        .unwrap();
        assert_eq!(session.openid.as_deref(), Some("openid"));
        assert_eq!(session.unionid.as_deref(), Some("union"));

        let interfaces: OpenPlatformMiniProgramPrivacyInterfaceResponse =
            serde_json::from_value(json!({
                "interface_list": [{
                    "api_name": "wx.getUserProfile",
                    "status": 2,
                    "fail_reason": "needs description",
                    "audit_id": 100,
                    "apply_time": 1800000000,
                    "audit_time": 1800000100,
                    "interface_extra": "retained"
                }],
                "request_id": "privacy-interfaces"
            }))
            .unwrap();
        assert_eq!(
            interfaces.interface_list[0].api_name.as_deref(),
            Some("wx.getUserProfile")
        );
        assert_eq!(interfaces.interface_list[0].status, Some(2));
        assert_eq!(
            interfaces.interface_list[0].fail_reason.as_deref(),
            Some("needs description")
        );
        assert_eq!(interfaces.interface_list[0].audit_id, Some(100));
        assert_eq!(
            interfaces.interface_list[0].extra["interface_extra"],
            "retained"
        );
        assert_eq!(interfaces.extra["request_id"], "privacy-interfaces");

        let apply = serde_json::to_value(OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
            api_name: "wx.getUserProfile".to_string(),
            content: "用于完善会员资料".to_string(),
            url: Some("https://example.com/privacy".to_string()),
            scene: Some(1),
        })
        .unwrap();
        assert_eq!(apply["api_name"], "wx.getUserProfile");
        assert_eq!(apply["scene"], 1);

        let applied: OpenPlatformMiniProgramPrivacyInterfaceApplyResponse =
            serde_json::from_value(json!({
                "audit_id": 100,
                "request_id": "privacy-apply"
            }))
            .unwrap();
        assert_eq!(applied.audit_id, Some(100));
        assert_eq!(applied.extra["request_id"], "privacy-apply");
    }

    #[test]
    fn serializes_register_mini_program_request() {
        let value = serde_json::to_value(RegisterMiniProgramRequest {
            name: "corp".to_string(),
            code: "9133".to_string(),
            code_type: 1,
            legal_persona_wechat: "wechat".to_string(),
            legal_persona_name: "name".to_string(),
            component_phone: "13800000000".to_string(),
        })
        .unwrap();

        assert_eq!(value["legal_persona_wechat"], "wechat");
        assert_eq!(value["code_type"], 1);
    }

    #[test]
    fn serializes_registration_status_request() {
        let value = serde_json::to_value(RegistrationStatusRequest {
            name: "corp".to_string(),
            legal_persona_wechat: "wechat".to_string(),
            legal_persona_name: "name".to_string(),
        })
        .unwrap();

        assert_eq!(value["name"], "corp");
        assert_eq!(value["legal_persona_name"], "name");
    }

    #[test]
    fn deserializes_open_platform_status_extra_fields() {
        let response: OpenPlatformStatusResponse =
            serde_json::from_value(json!({ "errcode": 0, "status": 1, "msg": "ok" })).unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.extra["status"], 1);
        assert_eq!(response.extra["msg"], "ok");
    }
}
