use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::Result,
    modules::{DomainModule, PlatformClient},
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
        action: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "wxa/change_visitstatus",
                Some(authorizer_access_token.into()),
                json!({ "action": action.into() }),
            )
            .await
    }

    pub async fn gray_release_authorizer_mini_program(
        &self,
        authorizer_access_token: impl Into<String>,
        gray_percentage: i64,
    ) -> Result<OpenPlatformStatusResponse> {
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
        self.inner
            .post(
                "cgi-bin/wxopen/setweappsupportversion",
                Some(authorizer_access_token.into()),
                json!({ "version": version.into() }),
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
        action: impl Into<String>,
        domains: Vec<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "wxa/setwebviewdomain",
                Some(authorizer_access_token.into()),
                json!({ "action": action.into(), "webviewdomain": domains }),
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
        self.inner
            .post(
                "wxa/bind_tester",
                Some(authorizer_access_token.into()),
                json!({ "wechatid": wechat_id.into() }),
            )
            .await
    }

    pub async fn unbind_authorizer_mini_program_tester(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramTesterUnbindRequest,
    ) -> Result<OpenPlatformStatusResponse> {
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
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
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
        self.inner
            .get_with_query(
                "sns/component/jscode2session",
                None,
                vec![
                    ("appid".to_string(), appid.into()),
                    ("js_code".to_string(), js_code.into()),
                    ("grant_type".to_string(), "authorization_code".to_string()),
                    ("component_appid".to_string(), component_appid.into()),
                    (
                        "component_access_token".to_string(),
                        component_access_token.into(),
                    ),
                ],
            )
            .await
    }

    pub fn official_account_fast_registration_url(
        request: OpenPlatformOfficialAccountFastRegistrationUrlRequest,
    ) -> String {
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
        url.to_string()
    }

    pub async fn fast_register_authorizer_official_account_mini_program(
        &self,
        authorizer_access_token: impl Into<String>,
        ticket: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "cgi-bin/account/fastregister",
                Some(authorizer_access_token.into()),
                json!({ "ticket": ticket.into() }),
            )
            .await
    }

    pub async fn get_authorizer_account_basic_info(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformAuthorizerAccountBasicInfoResponse> {
        self.inner
            .post(
                "cgi-bin/account/getaccountbasicinfo",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn modify_authorizer_account_head_image(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformAuthorizerAccountHeadImageRequest,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "cgi-bin/account/modifyheadimage",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_authorizer_account_signature(
        &self,
        authorizer_access_token: impl Into<String>,
        signature: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "cgi-bin/account/modifysignature",
                Some(authorizer_access_token.into()),
                json!({ "signature": signature.into() }),
            )
            .await
    }

    pub async fn get_authorizer_material_bytes(
        &self,
        authorizer_access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .post_json_bytes(
                "cgi-bin/material/get_material",
                Some(authorizer_access_token.into()),
                json!({ "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn create_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
    ) -> Result<OpenPlatformOpenAccountResponse> {
        self.inner
            .post(
                "cgi-bin/open/create",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid.into() }),
            )
            .await
    }

    pub async fn bind_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
        open_appid: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "cgi-bin/open/bind",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid.into(), "open_appid": open_appid.into() }),
            )
            .await
    }

    pub async fn unbind_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
        open_appid: impl Into<String>,
    ) -> Result<OpenPlatformStatusResponse> {
        self.inner
            .post(
                "cgi-bin/open/unbind",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid.into(), "open_appid": open_appid.into() }),
            )
            .await
    }

    pub async fn get_authorizer_open_account(
        &self,
        authorizer_access_token: impl Into<String>,
        appid: impl Into<String>,
    ) -> Result<OpenPlatformOpenAccountResponse> {
        self.inner
            .post(
                "cgi-bin/open/get",
                Some(authorizer_access_token.into()),
                json!({ "appid": appid.into() }),
            )
            .await
    }

    pub async fn get_authorizer_mini_program_privacy_interface(
        &self,
        authorizer_access_token: impl Into<String>,
    ) -> Result<OpenPlatformMiniProgramPrivacyInterfaceResponse> {
        self.inner
            .post(
                "wxa/security/get_privacy_interface",
                Some(authorizer_access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn apply_authorizer_mini_program_privacy_interface(
        &self,
        authorizer_access_token: impl Into<String>,
        request: OpenPlatformMiniProgramPrivacyInterfaceApplyRequest,
    ) -> Result<OpenPlatformMiniProgramPrivacyInterfaceApplyResponse> {
        self.inner
            .post(
                "wxa/security/apply_privacy_interface",
                Some(authorizer_access_token.into()),
                request,
            )
            .await
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
    pub authorization_info: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformConfirmInfo {
    #[serde(default)]
    pub need_confirm: Option<i64>,
    #[serde(default)]
    pub already_confirm: Option<i64>,
    #[serde(default)]
    pub can_confirm: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformFuncInfo {
    #[serde(default)]
    pub funcscope_category: Option<OpenPlatformFuncScopeCategory>,
    #[serde(default)]
    pub confirm_info: Option<OpenPlatformConfirmInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformHandleAuthorizeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<OpenPlatformAuthorizationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformServiceTypeInfo {
    #[serde(default)]
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformVerifyTypeInfo {
    #[serde(default)]
    pub id: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformBasicConfig {
    #[serde(default)]
    pub is_phone_configured: Option<bool>,
    #[serde(default)]
    pub is_email_configured: Option<bool>,
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
    #[serde(default, rename = "MiniProgramInfo")]
    pub mini_program_info: Option<Value>,
    #[serde(default)]
    pub register_type: Option<i64>,
    #[serde(default)]
    pub basic_config: Option<OpenPlatformBasicConfig>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizationSummary {
    #[serde(default)]
    pub authorizer_appid: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub auth_time: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramCommitRequest {
    pub template_id: String,
    pub ext_json: String,
    pub user_version: String,
    pub user_desc: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<i64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramCategoryResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub category_list: Vec<OpenPlatformMiniProgramCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub page_list: Vec<String>,
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
    pub user_version: Option<String>,
    #[serde(default)]
    pub user_desc: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramRollbackReleaseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub version_list: Vec<OpenPlatformMiniProgramReleaseVersion>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramGrayReleasePlanResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub gray_release_plan: Option<OpenPlatformMiniProgramGrayReleasePlan>,
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
    pub version: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramModifyDomainRequest {
    pub action: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterUnbindRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wechatid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userstr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterBindResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userstr: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramTesterListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub members: Vec<OpenPlatformMiniProgramTesterMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyOwnerSetting {
    #[serde(default)]
    pub contact_email: Option<String>,
    #[serde(default)]
    pub contact_phone: Option<String>,
    #[serde(default)]
    pub contact_qq: Option<String>,
    #[serde(default)]
    pub contact_weixin: Option<String>,
    #[serde(default)]
    pub notice_method: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySettingItem {
    #[serde(default)]
    pub privacy_key: Option<String>,
    #[serde(default)]
    pub privacy_text: Option<String>,
    #[serde(default)]
    pub privacy_label: Option<String>,
    #[serde(default)]
    pub privacy_desc: Option<String>,
    #[serde(default)]
    pub privacy_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacySettingRequest {
    pub owner_setting: OpenPlatformMiniProgramPrivacyOwnerSetting,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub setting_list: Vec<OpenPlatformMiniProgramPrivacySettingItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_ver: Option<i64>,
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
    pub privacy_ver: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyExtFileResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ext_file_media_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformAuthorizerAccountHeadImageRequest {
    pub head_img_media_id: String,
    pub x1: String,
    pub y1: String,
    pub x2: String,
    pub y2: String,
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
    pub wx_verify_info: Option<Value>,
    #[serde(default)]
    pub signature_info: Option<Value>,
    #[serde(default)]
    pub head_image_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformOpenAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_appid: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPlatformMiniProgramPrivacyInterfaceApplyRequest {
    pub api_name: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<i64>,
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        AddTemplateFromDraftRequest, AuthorizerAccessTokenRequest, AuthorizerAccessTokenResponse,
        ComponentAccessTokenRequest, ComponentAccessTokenResponse, DeleteTemplateRequest,
        OpenPlatform, OpenPlatformAuthorizerAccountBasicInfoResponse,
        OpenPlatformAuthorizerAccountHeadImageRequest, OpenPlatformAuthorizerInfoResponse,
        OpenPlatformAuthorizerOptionResponse, OpenPlatformAuthorizersResponse,
        OpenPlatformComponentLoginPageUrlRequest, OpenPlatformHandleAuthorizeResponse,
        OpenPlatformMiniProgramAuditItem, OpenPlatformMiniProgramAuditPreviewInfo,
        OpenPlatformMiniProgramAuditQuotaResponse, OpenPlatformMiniProgramAuditStatusResponse,
        OpenPlatformMiniProgramCategoryResponse, OpenPlatformMiniProgramCommitRequest,
        OpenPlatformMiniProgramGrayReleasePlanResponse,
        OpenPlatformMiniProgramLatestAuditStatusResponse,
        OpenPlatformMiniProgramModifyDomainRequest, OpenPlatformMiniProgramModifyDomainResponse,
        OpenPlatformMiniProgramPageResponse, OpenPlatformMiniProgramPrivacyExtFileResponse,
        OpenPlatformMiniProgramPrivacyInterfaceApplyRequest,
        OpenPlatformMiniProgramPrivacyInterfaceApplyResponse,
        OpenPlatformMiniProgramPrivacyInterfaceResponse,
        OpenPlatformMiniProgramPrivacyOwnerSetting, OpenPlatformMiniProgramPrivacySettingItem,
        OpenPlatformMiniProgramPrivacySettingRequest,
        OpenPlatformMiniProgramPrivacySettingResponse,
        OpenPlatformMiniProgramRollbackReleaseResponse, OpenPlatformMiniProgramSessionResponse,
        OpenPlatformMiniProgramSubmitAuditRequest, OpenPlatformMiniProgramSubmitAuditResponse,
        OpenPlatformMiniProgramSupportVersionResponse, OpenPlatformMiniProgramTesterBindResponse,
        OpenPlatformMiniProgramTesterListResponse, OpenPlatformMiniProgramTesterUnbindRequest,
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
                "authorizer_appid": "wx-authorizer"
            }
        }))
        .unwrap();

        assert_eq!(
            token.component_access_token.as_deref(),
            Some("component-token")
        );
        assert_eq!(token.expires_in, Some(7200));
        assert_eq!(preauth.pre_auth_code.as_deref(), Some("preauth"));
        assert_eq!(
            query_auth.authorization_info.expect("authorization_info")["authorizer_appid"],
            "wx-authorizer"
        );
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
                    "network": { "RequestDomain": ["https://api.example.com"] },
                    "visit_status": 0
                },
                "basic_config": {
                    "is_phone_configured": true,
                    "is_email_configured": false
                }
            },
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "func_info": []
            }
        }))
        .unwrap();
        let authorizer_info = authorizer.authorizer_info.expect("authorizer_info");
        assert_eq!(authorizer_info.nick_name.as_deref(), Some("demo"));
        assert_eq!(authorizer_info.service_type_info.unwrap().id, Some(2));
        assert_eq!(
            authorizer_info.basic_config.unwrap().is_phone_configured,
            Some(true)
        );
        assert_eq!(
            authorizer_info.mini_program_info.unwrap()["network"]["RequestDomain"][0],
            "https://api.example.com"
        );

        let list: OpenPlatformAuthorizersResponse = serde_json::from_value(json!({
            "total_count": 1,
            "list": [{
                "authorizer_appid": "wx-authorizer",
                "refresh_token": "refresh-token",
                "auth_time": 1800000000
            }]
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.list[0].refresh_token.as_deref(), Some("refresh-token"));

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
        let commit = serde_json::to_value(OpenPlatformMiniProgramCommitRequest {
            template_id: "100".to_string(),
            ext_json: "{\"extAppid\":\"wx\"}".to_string(),
            user_version: "1.0.0".to_string(),
            user_desc: "release".to_string(),
        })
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
            ugc_declare: None,
            privacy_api_not_use: Some(true),
            order_path: None,
        })
        .unwrap();
        assert_eq!(audit["item_list"][0]["address"], "pages/index/index");
        assert_eq!(audit["preview_info"]["pic_id_list"][0], "pic");
        assert_eq!(audit["privacy_api_not_use"], true);
        assert!(audit.get("feedback_stuff").is_none());
    }

    #[test]
    fn deserializes_authorizer_mini_program_code_responses() {
        let categories: OpenPlatformMiniProgramCategoryResponse = serde_json::from_value(json!({
            "category_list": [{ "first_class": "tool", "first_id": 1, "extra_field": "kept" }]
        }))
        .unwrap();
        assert_eq!(categories.category_list[0].first_id, Some(1));
        assert_eq!(
            categories.category_list[0].first_class.as_deref(),
            Some("tool")
        );
        assert_eq!(categories.category_list[0].extra["extra_field"], "kept");

        let pages: OpenPlatformMiniProgramPageResponse =
            serde_json::from_value(json!({ "page_list": ["pages/index/index"] })).unwrap();
        assert_eq!(pages.page_list[0], "pages/index/index");

        let submitted: OpenPlatformMiniProgramSubmitAuditResponse = serde_json::from_value(json!({
            "type": "audit",
            "mediaid": "media",
            "auditid": 123
        }))
        .unwrap();
        assert_eq!(submitted.audit_type.as_deref(), Some("audit"));
        assert_eq!(submitted.auditid, Some(123));

        let status: OpenPlatformMiniProgramAuditStatusResponse = serde_json::from_value(json!({
            "status": 0,
            "reason": "ok",
            "screenshot": "https://example.com/s.png"
        }))
        .unwrap();
        assert_eq!(status.status, Some(0));
        assert_eq!(status.reason.as_deref(), Some("ok"));

        let latest: OpenPlatformMiniProgramLatestAuditStatusResponse =
            serde_json::from_value(json!({
                "auditid": 124,
                "status": 0,
                "ScreenShot": "https://example.com/latest.png",
                "user_version": "1.0.0",
                "submit_audit_time": 1800000000
            }))
            .unwrap();
        assert_eq!(latest.auditid, Some(124));
        assert_eq!(
            latest.screenshot.as_deref(),
            Some("https://example.com/latest.png")
        );

        let rollback: OpenPlatformMiniProgramRollbackReleaseResponse =
            serde_json::from_value(json!({
                "version_list": [{ "user_version": "1.0.0", "app_version": 1 }]
            }))
            .unwrap();
        assert_eq!(
            rollback.version_list[0].user_version.as_deref(),
            Some("1.0.0")
        );
        assert_eq!(rollback.version_list[0].extra["app_version"], 1);

        let gray: OpenPlatformMiniProgramGrayReleasePlanResponse = serde_json::from_value(json!({
            "gray_release_plan": { "status": 1, "gray_percentage": 10 }
        }))
        .unwrap();
        assert_eq!(gray.gray_release_plan.unwrap().gray_percentage, Some(10));

        let support: OpenPlatformMiniProgramSupportVersionResponse =
            serde_json::from_value(json!({
                "now_version": "3.0.0",
                "uv_info": { "items": [{ "visit_uv": 90, "version": "3.0.0" }] }
            }))
            .unwrap();
        assert_eq!(support.now_version.as_deref(), Some("3.0.0"));
        assert_eq!(
            support.uv_info.unwrap().items[0].version.as_deref(),
            Some("3.0.0")
        );

        let quota: OpenPlatformMiniProgramAuditQuotaResponse = serde_json::from_value(json!({
            "rest": 10,
            "limit": 100,
            "speedup_rest": 1,
            "speedup_limit": 10
        }))
        .unwrap();
        assert_eq!(quota.rest, Some(10));
        assert_eq!(quota.speedup_limit, Some(10));
    }

    #[test]
    fn serializes_authorizer_mini_program_domain_and_tester_requests() {
        let domain = serde_json::to_value(OpenPlatformMiniProgramModifyDomainRequest {
            action: "set".to_string(),
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

        let list: OpenPlatformMiniProgramTesterListResponse = serde_json::from_value(json!({
            "members": [{ "wechatid": "tester", "userstr": "userstr" }]
        }))
        .unwrap();
        assert_eq!(list.members[0].wechatid.as_deref(), Some("tester"));
    }

    #[test]
    fn serializes_authorizer_mini_program_privacy_requests() {
        let privacy = serde_json::to_value(OpenPlatformMiniProgramPrivacySettingRequest {
            owner_setting: OpenPlatformMiniProgramPrivacyOwnerSetting {
                contact_email: Some("dev@example.com".to_string()),
                contact_phone: None,
                contact_qq: None,
                contact_weixin: None,
                notice_method: Some("email".to_string()),
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
            privacy_ver: Some(2),
        })
        .unwrap();
        assert_eq!(privacy["owner_setting"]["contact_email"], "dev@example.com");
        assert_eq!(privacy["setting_list"][0]["privacy_key"], "UserInfo");
        assert_eq!(privacy["privacy_ver"], 2);

        let response: OpenPlatformMiniProgramPrivacySettingResponse =
            serde_json::from_value(json!({
                "owner_setting": { "contact_email": "dev@example.com" },
                "setting_list": [{ "privacy_key": "UserInfo" }],
                "privacy_ver": 2
            }))
            .unwrap();
        assert_eq!(
            response.owner_setting.unwrap().contact_email.as_deref(),
            Some("dev@example.com")
        );
        assert_eq!(
            response.setting_list[0].privacy_key.as_deref(),
            Some("UserInfo")
        );

        let upload: OpenPlatformMiniProgramPrivacyExtFileResponse =
            serde_json::from_value(json!({ "ext_file_media_id": "media" })).unwrap();
        assert_eq!(upload.ext_file_media_id.as_deref(), Some("media"));

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
                "interface_list": [{ "api_name": "getUserInfo", "status": 2, "scope": "profile" }],
                "request_id": "privacy-interface"
            }))
            .unwrap();
        assert_eq!(
            interfaces.interface_list[0].api_name.as_deref(),
            Some("getUserInfo")
        );
        assert_eq!(interfaces.interface_list[0].status, Some(2));
        assert_eq!(interfaces.interface_list[0].extra["scope"], "profile");
        assert_eq!(interfaces.extra["request_id"], "privacy-interface");

        let apply_response: OpenPlatformMiniProgramPrivacyInterfaceApplyResponse =
            serde_json::from_value(json!({ "audit_id": 10, "request_id": "apply-10" })).unwrap();
        assert_eq!(apply_response.audit_id, Some(10));
        assert_eq!(apply_response.extra["request_id"], "apply-10");
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
        );
        assert!(fast_url.starts_with("https://mp.weixin.qq.com/cgi-bin/fastregisterauth?"));
        assert!(fast_url.contains("copy_wx_verify=true"));
        assert!(fast_url.contains("appid=wx-authorizer"));
    }

    #[test]
    fn serializes_authorizer_account_and_open_account_requests() {
        let head = serde_json::to_value(OpenPlatformAuthorizerAccountHeadImageRequest {
            head_img_media_id: "media".to_string(),
            x1: "0.000000".to_string(),
            y1: "0.000000".to_string(),
            x2: "1.000000".to_string(),
            y2: "1.000000".to_string(),
        })
        .unwrap();
        assert_eq!(head["head_img_media_id"], "media");
        assert_eq!(head["x2"], "1.000000");

        let basic: OpenPlatformAuthorizerAccountBasicInfoResponse = serde_json::from_value(json!({
            "appid": "wx-authorizer",
            "account_type": 1,
            "principal_name": "principal",
            "signature_info": { "signature": "hello" },
            "head_image_info": { "head_image_url": "https://example.com/head.png" }
        }))
        .unwrap();
        assert_eq!(basic.appid.as_deref(), Some("wx-authorizer"));
        assert_eq!(basic.signature_info.unwrap()["signature"], "hello");

        let open: OpenPlatformOpenAccountResponse =
            serde_json::from_value(json!({ "open_appid": "open-appid" })).unwrap();
        assert_eq!(open.open_appid.as_deref(), Some("open-appid"));
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
