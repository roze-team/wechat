use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::Result,
    modules::{DomainModule, PlatformClient},
    Client,
};

#[derive(Debug, Clone)]
pub struct OpenWork {
    inner: PlatformClient,
}

impl OpenWork {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn provider(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.provider")
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.base")
    }

    pub async fn create_component_preauth_code(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<OpenWorkComponentPreauthCodeResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/api_create_preauthcode",
            json!({ "component_appid": component_appid.into() }),
        )
        .await
    }

    pub async fn query_component_auth(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorization_code: impl Into<String>,
    ) -> Result<OpenWorkComponentQueryAuthResponse> {
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

    pub async fn get_component_authorizer_info(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
    ) -> Result<OpenWorkComponentAuthorizerInfoResponse> {
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

    pub async fn get_component_authorizers(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        offset: i64,
        count: i64,
    ) -> Result<OpenWorkComponentAuthorizersResponse> {
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

    pub async fn get_component_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
    ) -> Result<OpenWorkComponentAuthorizerOptionResponse> {
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

    pub async fn set_component_authorizer_option(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
        option_name: impl Into<String>,
        option_value: impl Into<String>,
    ) -> Result<OpenWorkStatusResponse> {
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

    pub async fn clear_component_quota(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
    ) -> Result<OpenWorkStatusResponse> {
        self.post_component_json(
            component_access_token,
            "cgi-bin/component/clear_quota",
            json!({ "component_appid": component_appid.into() }),
        )
        .await
    }

    pub async fn provider_token(
        &self,
        request: ProviderTokenRequest,
    ) -> Result<ProviderTokenResponse> {
        self.inner
            .post("cgi-bin/service/get_provider_token", None, request)
            .await
    }

    pub async fn corp_id_to_open_corp_id(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
    ) -> Result<OpenWorkOpenCorpIdResponse> {
        self.inner
            .post(
                "cgi-bin/service/corpid_to_opencorpid",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into() }),
            )
            .await
    }

    pub async fn get_customized_auth_url(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkCustomizedAuthUrlRequest,
    ) -> Result<OpenWorkCustomizedAuthUrlResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_customized_auth_url",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub fn suit_auth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.suit_auth")
    }

    pub async fn suite_token(&self, request: SuiteTokenRequest) -> Result<SuiteTokenResponse> {
        self.inner
            .post("cgi-bin/service/get_suite_token", None, request)
            .await
    }

    pub async fn pre_auth_code(
        &self,
        suite_access_token: impl Into<String>,
    ) -> Result<OpenWorkPreAuthCodeResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/service/get_pre_auth_code",
                Some(suite_access_token.into()),
                Vec::new(),
            )
            .await
    }

    pub async fn pre_auth_code_typed(
        &self,
        suite_access_token: impl Into<String>,
    ) -> Result<OpenWorkPreAuthCodeResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/service/get_pre_auth_code",
                Some(suite_access_token.into()),
                Vec::new(),
            )
            .await
    }

    pub async fn permanent_code(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_permanent_code",
                Some(suite_access_token.into()),
                json!({ "auth_code": auth_code.into() }),
            )
            .await
    }

    pub async fn permanent_code_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_permanent_code",
                Some(suite_access_token.into()),
                json!({ "auth_code": auth_code.into() }),
            )
            .await
    }

    pub async fn set_session_info(
        &self,
        suite_access_token: impl Into<String>,
        request: OpenWorkSetSessionInfoRequest,
    ) -> Result<OpenWorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/service/set_session_info",
                Some(suite_access_token.into()),
                request,
            )
            .await
    }

    pub async fn permanent_code_v2(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/v2/get_permanent_code",
                Some(suite_access_token.into()),
                json!({ "auth_code": auth_code.into() }),
            )
            .await
    }

    pub fn corp(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.corp")
    }

    pub async fn auth_info(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_auth_info",
                Some(suite_access_token.into()),
                json!({
                    "auth_corpid": auth_corpid.into(),
                    "permanent_code": permanent_code.into(),
                }),
            )
            .await
    }

    pub async fn auth_info_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_auth_info",
                Some(suite_access_token.into()),
                json!({
                    "auth_corpid": auth_corpid.into(),
                    "permanent_code": permanent_code.into(),
                }),
            )
            .await
    }

    pub async fn auth_info_v2(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.inner
            .post(
                "cgi-bin/service/v2/get_auth_info",
                Some(suite_access_token.into()),
                json!({
                    "auth_corpid": auth_corpid.into(),
                    "permanent_code": permanent_code.into(),
                }),
            )
            .await
    }

    pub async fn corp_token(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkCorpTokenResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_corp_token",
                Some(suite_access_token.into()),
                json!({
                    "auth_corpid": auth_corpid.into(),
                    "permanent_code": permanent_code.into(),
                }),
            )
            .await
    }

    pub async fn corp_token_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkCorpTokenResponse> {
        self.inner
            .post(
                "cgi-bin/service/get_corp_token",
                Some(suite_access_token.into()),
                json!({
                    "auth_corpid": auth_corpid.into(),
                    "permanent_code": permanent_code.into(),
                }),
            )
            .await
    }

    pub async fn user_id_to_open_user_id(
        &self,
        suite_access_token: impl Into<String>,
        user_id_list: Vec<String>,
    ) -> Result<OpenWorkUserIdToOpenUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/batch/userid_to_openuserid",
                Some(suite_access_token.into()),
                json!({ "userid_list": user_id_list }),
            )
            .await
    }

    pub fn user(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.user")
    }

    pub async fn get_user_info_3rd_by_code(
        &self,
        suite_access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<OpenWorkUserInfo3rdByCodeResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/service/getuserinfo3rd",
                Some(suite_access_token.into()),
                vec![("code".to_string(), code.into())],
            )
            .await
    }

    pub async fn get_user_info_3rd_by_user_ticket(
        &self,
        suite_access_token: impl Into<String>,
        user_ticket: impl Into<String>,
    ) -> Result<OpenWorkUserInfo3rdByUserTicketResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/service/getuserinfo3rd",
                Some(suite_access_token.into()),
                vec![("user_ticket".to_string(), user_ticket.into())],
            )
            .await
    }

    pub fn external_contact(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.external_contact")
    }

    pub async fn union_id_to_external_user_id_3rd(
        &self,
        provider_access_token: impl Into<String>,
        union_id: impl Into<String>,
        open_id: impl Into<String>,
        corp_id: Option<String>,
    ) -> Result<OpenWorkUnionIdToExternalUserId3rdResponse> {
        let mut query = vec![
            ("unionid".to_string(), union_id.into()),
            ("openid".to_string(), open_id.into()),
        ];
        if let Some(corp_id) = corp_id {
            query.push(("corpid".to_string(), corp_id));
        }
        self.inner
            .get_with_query(
                "cgi-bin/externalcontact/unionid_to_external_userid_3rd",
                Some(provider_access_token.into()),
                query,
            )
            .await
    }

    pub fn license(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.license")
    }

    pub async fn create_license_new_order(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseCreateNewOrderRequest,
    ) -> Result<OpenWorkLicenseOrderIdResponse> {
        self.inner
            .post(
                "cgi-bin/license/create_new_order",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn create_license_renew_order_job(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseCreateRenewOrderJobRequest,
    ) -> Result<OpenWorkLicenseCreateRenewOrderJobResponse> {
        self.inner
            .post(
                "cgi-bin/license/create_renew_order_job",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn submit_license_order_job(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseSubmitOrderJobRequest,
    ) -> Result<OpenWorkLicenseOrderIdResponse> {
        self.inner
            .post(
                "cgi-bin/license/submit_order_job",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_license_order(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListOrderRequest,
    ) -> Result<OpenWorkLicenseListOrderResponse> {
        self.inner
            .post(
                "cgi-bin/license/list_order",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_license_order(
        &self,
        provider_access_token: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseOrderResponse> {
        self.inner
            .post(
                "cgi-bin/license/get_order",
                Some(provider_access_token.into()),
                json!({ "order_id": order_id.into() }),
            )
            .await
    }

    pub async fn list_license_order_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListOrderAccountRequest,
    ) -> Result<OpenWorkLicenseListAccountResponse> {
        self.inner
            .post(
                "cgi-bin/license/list_order_account",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn cancel_license_order(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Result<OpenWorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/license/cancel_order",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "order_id": order_id.into() }),
            )
            .await
    }

    pub async fn activate_license_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseActiveInfo,
    ) -> Result<OpenWorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/license/active_account",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_activate_license_account(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_list: Vec<OpenWorkLicenseActiveInfo>,
    ) -> Result<OpenWorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/license/batch_active_account",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "active_list": active_list }),
            )
            .await
    }

    pub async fn get_license_active_info_by_code(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_code: impl Into<String>,
    ) -> Result<OpenWorkLicenseActiveInfoResponse> {
        self.inner
            .post(
                "cgi-bin/license/get_active_info_by_code",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "active_code": active_code.into() }),
            )
            .await
    }

    pub async fn batch_get_license_active_info_by_code(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_code_list: Vec<String>,
    ) -> Result<OpenWorkLicenseActiveInfoListResponse> {
        self.inner
            .post(
                "cgi-bin/license/batch_get_active_info_by_code",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "active_code_list": active_code_list }),
            )
            .await
    }

    pub async fn list_license_activated_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListActivatedAccountRequest,
    ) -> Result<OpenWorkLicenseListAccountResponse> {
        self.inner
            .post(
                "cgi-bin/license/list_activated_account",
                Some(provider_access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_license_active_info_by_user(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseUserActiveInfoResponse> {
        self.inner
            .post(
                "cgi-bin/license/get_active_info_by_user",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "userid": user_id.into() }),
            )
            .await
    }

    pub async fn batch_transfer_license(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        transfer_list: Vec<OpenWorkLicenseTransferInfo>,
    ) -> Result<OpenWorkLicenseTransferResponse> {
        self.inner
            .post(
                "cgi-bin/license/batch_transfer_license",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "transfer_list": transfer_list }),
            )
            .await
    }

    pub async fn get_app_license_info(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        suite_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseInfoResponse> {
        self.inner
            .post(
                "cgi-bin/license/get_app_license_info",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "suite_id": suite_id.into() }),
            )
            .await
    }

    pub async fn set_license_auto_active_status(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        auto_active_status: i64,
    ) -> Result<OpenWorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/license/set_auto_active_status",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into(), "auto_active_status": auto_active_status }),
            )
            .await
    }

    pub async fn get_license_auto_active_status(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseAutoActiveStatusResponse> {
        self.inner
            .post(
                "cgi-bin/license/get_auto_active_status",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into() }),
            )
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.server")
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
pub struct OpenWorkComponentPreauthCodeResponse {
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
pub struct OpenWorkComponentQueryAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentAuthorizerInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorizer_info: Option<Value>,
    #[serde(default)]
    pub authorization_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentAuthorizationSummary {
    #[serde(default)]
    pub authorizer_appid: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub auth_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentAuthorizersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub list: Vec<OpenWorkComponentAuthorizationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentAuthorizerOptionResponse {
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
pub struct ProviderTokenRequest {
    pub corpid: String,
    pub provider_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub provider_access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkOpenCorpIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_corpid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkCustomizedAuthUrlRequest {
    pub state: String,
    pub templateid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkCustomizedAuthUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub qrcode_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteTokenRequest {
    pub suite_id: String,
    pub suite_secret: String,
    pub suite_ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub suite_access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkPreAuthCodeResponse {
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
pub struct OpenWorkSetSessionInfoRequest {
    pub pre_auth_code: String,
    pub session_info: OpenWorkSessionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkSessionInfo {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub appid: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkPermanentCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub permanent_code: Option<String>,
    #[serde(default)]
    pub auth_corp_info: Option<Value>,
    #[serde(default)]
    pub auth_info: Option<Value>,
    #[serde(default)]
    pub auth_user_info: Option<Value>,
    #[serde(default)]
    pub register_code_info: Option<Value>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub dealer_corp_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkCorpTokenResponse {
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
pub struct OpenWorkUserIdToOpenUserIdItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkUserIdToOpenUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_userid_list: Vec<OpenWorkUserIdToOpenUserIdItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkUserInfo3rdByCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub deviceid: Option<String>,
    #[serde(default)]
    pub user_ticket: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub open_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkUserInfo3rdByUserTicketResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub gender: Option<i64>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub qrcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkExternalUserIdInfo {
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkUnionIdToExternalUserId3rdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid_info: Vec<OpenWorkExternalUserIdInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseAccountCount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_contact_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseAccountDuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_expire_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseCreateNewOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corpid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_count: Option<OpenWorkLicenseAccountCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_duration: Option<OpenWorkLicenseAccountDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseCreateRenewOrderJobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corpid: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub account_list: Vec<OpenWorkLicenseActiveInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseSubmitOrderJobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_duration: Option<OpenWorkLicenseAccountDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corpid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListOrderAccountRequest {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListActivatedAccountRequest {
    pub corpid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corpid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub account_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseTransferInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handover_userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takeover_userid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseOrderIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseCreateRenewOrderJobResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
    #[serde(default)]
    pub invalid_account_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: Option<i64>,
    #[serde(default)]
    pub order_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: Option<i64>,
    #[serde(default)]
    pub account_list: Vec<OpenWorkLicenseActiveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_info: Option<OpenWorkLicenseActiveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveInfoListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_info_list: Vec<OpenWorkLicenseActiveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseUserActiveInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_status: Option<i64>,
    #[serde(default)]
    pub active_info_list: Vec<OpenWorkLicenseActiveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseTransferResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub transfer_result: Vec<OpenWorkLicenseTransferInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub license_status: Option<i64>,
    #[serde(default)]
    pub license_check_time: Option<i64>,
    #[serde(default)]
    pub trail_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseAutoActiveStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub auto_active_status: Option<i64>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_provider_token_request() {
        let value = serde_json::to_value(ProviderTokenRequest {
            corpid: "corp".to_string(),
            provider_secret: "secret".to_string(),
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "corpid": "corp", "provider_secret": "secret" })
        );
    }

    #[test]
    fn serializes_suite_token_request() {
        let value = serde_json::to_value(SuiteTokenRequest {
            suite_id: "suite".to_string(),
            suite_secret: "secret".to_string(),
            suite_ticket: "ticket".to_string(),
        })
        .unwrap();

        assert_eq!(value["suite_ticket"], "ticket");
    }

    #[test]
    fn deserializes_open_work_token_responses() {
        let provider: ProviderTokenResponse = serde_json::from_value(json!({
            "errcode": 0,
            "provider_access_token": "provider-token",
            "expires_in": 7200
        }))
        .unwrap();
        assert_eq!(
            provider.provider_access_token.as_deref(),
            Some("provider-token")
        );
        assert_eq!(provider.expires_in, Some(7200));

        let suite: SuiteTokenResponse = serde_json::from_value(json!({
            "errcode": 0,
            "suite_access_token": "suite-token",
            "expires_in": 7200
        }))
        .unwrap();
        assert_eq!(suite.suite_access_token.as_deref(), Some("suite-token"));
        assert_eq!(suite.expires_in, Some(7200));
    }

    #[test]
    fn deserializes_open_work_component_base_responses() {
        let preauth: OpenWorkComponentPreauthCodeResponse = serde_json::from_value(json!({
            "pre_auth_code": "pre-auth",
            "expires_in": 1200
        }))
        .unwrap();
        assert_eq!(preauth.pre_auth_code.as_deref(), Some("pre-auth"));
        assert_eq!(preauth.expires_in, Some(1200));

        let query: OpenWorkComponentQueryAuthResponse = serde_json::from_value(json!({
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "authorizer_access_token": "token"
            }
        }))
        .unwrap();
        assert_eq!(
            query.authorization_info.unwrap()["authorizer_appid"],
            "wx-authorizer"
        );

        let info: OpenWorkComponentAuthorizerInfoResponse = serde_json::from_value(json!({
            "authorizer_info": { "nick_name": "Corp App" },
            "authorization_info": { "authorizer_appid": "wx-authorizer" }
        }))
        .unwrap();
        assert_eq!(info.authorizer_info.unwrap()["nick_name"], "Corp App");

        let list: OpenWorkComponentAuthorizersResponse = serde_json::from_value(json!({
            "total_count": 1,
            "list": [{
                "authorizer_appid": "wx-authorizer",
                "refresh_token": "refresh",
                "auth_time": 1800000000
            }]
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.list[0].refresh_token.as_deref(), Some("refresh"));

        let option: OpenWorkComponentAuthorizerOptionResponse = serde_json::from_value(json!({
            "authorizer_appid": "wx-authorizer",
            "option_name": "voice_recognize",
            "option_value": "1"
        }))
        .unwrap();
        assert_eq!(option.option_name.as_deref(), Some("voice_recognize"));
        assert_eq!(option.option_value.as_deref(), Some("1"));
    }

    #[test]
    fn serializes_open_work_provider_and_suite_requests() {
        let auth_url = serde_json::to_value(OpenWorkCustomizedAuthUrlRequest {
            state: "state-token".to_string(),
            templateid_list: vec!["tpl-a".to_string(), "tpl-b".to_string()],
        })
        .unwrap();
        assert_eq!(auth_url["state"], "state-token");
        assert_eq!(auth_url["templateid_list"][1], "tpl-b");

        let session = serde_json::to_value(OpenWorkSetSessionInfoRequest {
            pre_auth_code: "pre-auth".to_string(),
            session_info: OpenWorkSessionInfo {
                appid: vec![1, 2],
                auth_type: Some(1),
            },
        })
        .unwrap();
        assert_eq!(session["pre_auth_code"], "pre-auth");
        assert_eq!(session["session_info"]["appid"][0], 1);
        assert_eq!(session["session_info"]["auth_type"], 1);

        let no_appid = serde_json::to_value(OpenWorkSetSessionInfoRequest {
            pre_auth_code: "pre-auth".to_string(),
            session_info: OpenWorkSessionInfo {
                appid: Vec::new(),
                auth_type: None,
            },
        })
        .unwrap();
        assert!(no_appid["session_info"].get("appid").is_none());
        assert!(no_appid["session_info"].get("auth_type").is_none());
    }

    #[test]
    fn deserializes_open_work_provider_and_suite_responses() {
        let open_corp: OpenWorkOpenCorpIdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "open_corpid": "open-corp"
        }))
        .unwrap();
        assert_eq!(open_corp.open_corpid.as_deref(), Some("open-corp"));

        let auth_url: OpenWorkCustomizedAuthUrlResponse = serde_json::from_value(json!({
            "qrcode_url": "https://example.com/qrcode"
        }))
        .unwrap();
        assert_eq!(
            auth_url.qrcode_url.as_deref(),
            Some("https://example.com/qrcode")
        );

        let permanent: OpenWorkPermanentCodeResponse = serde_json::from_value(json!({
            "access_token": "corp-token",
            "expires_in": 7200,
            "permanent_code": "permanent",
            "auth_corp_info": { "corpid": "corp", "corp_name": "Corp" },
            "auth_info": { "agent": [{ "agentid": 100001, "name": "App" }] },
            "auth_user_info": { "userid": "admin", "name": "Admin" },
            "register_code_info": {
                "register_code": "register",
                "template_id": "tpl",
                "state": "state"
            },
            "state": "state",
            "dealer_corp_info": { "corpid": "dealer", "corp_name": "Dealer" }
        }))
        .unwrap();
        assert_eq!(permanent.access_token.as_deref(), Some("corp-token"));
        assert_eq!(permanent.auth_corp_info.unwrap()["corp_name"], "Corp");
        assert_eq!(permanent.register_code_info.unwrap()["template_id"], "tpl");

        let convert: OpenWorkUserIdToOpenUserIdResponse = serde_json::from_value(json!({
            "open_userid_list": [{
                "userid": "user",
                "open_userid": "open-user"
            }]
        }))
        .unwrap();
        assert_eq!(
            convert.open_userid_list[0].open_userid.as_deref(),
            Some("open-user")
        );
    }

    #[test]
    fn deserializes_open_work_auth_typed_responses() {
        let pre_auth: OpenWorkPreAuthCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "pre_auth_code": "pre-auth",
            "expires_in": 1200
        }))
        .unwrap();
        assert_eq!(pre_auth.pre_auth_code.as_deref(), Some("pre-auth"));
        assert_eq!(pre_auth.expires_in, Some(1200));

        let permanent: OpenWorkPermanentCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "access_token": "access-token",
            "expires_in": 7200,
            "permanent_code": "permanent",
            "auth_corp_info": { "corpid": "corp" },
            "auth_info": { "agent": [] },
            "auth_user_info": { "userid": "admin" }
        }))
        .unwrap();
        assert_eq!(permanent.permanent_code.as_deref(), Some("permanent"));
        assert_eq!(permanent.auth_user_info.unwrap()["userid"], "admin");

        let corp_token: OpenWorkCorpTokenResponse = serde_json::from_value(json!({
            "errcode": 0,
            "access_token": "corp-access-token",
            "expires_in": 7200
        }))
        .unwrap();
        assert_eq!(
            corp_token.access_token.as_deref(),
            Some("corp-access-token")
        );

        let status: OpenWorkStatusResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok"
        }))
        .unwrap();
        assert_eq!(status.errmsg.as_deref(), Some("ok"));
    }

    #[test]
    fn deserializes_open_work_user_responses() {
        let by_code: OpenWorkUserInfo3rdByCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "corpid": "corp",
            "userid": "user",
            "deviceid": "device",
            "user_ticket": "ticket",
            "expires_in": 1800,
            "open_userid": "open-user"
        }))
        .unwrap();
        assert_eq!(by_code.corpid.as_deref(), Some("corp"));
        assert_eq!(by_code.userid.as_deref(), Some("user"));
        assert_eq!(by_code.user_ticket.as_deref(), Some("ticket"));
        assert_eq!(by_code.open_userid.as_deref(), Some("open-user"));

        let by_ticket: OpenWorkUserInfo3rdByUserTicketResponse = serde_json::from_value(json!({
            "errcode": 0,
            "corpid": "corp",
            "userid": "user",
            "name": "User",
            "mobile": "13800000000",
            "gender": 1,
            "email": "user@example.com",
            "avatar": "https://example.com/avatar.png",
            "qrcode": "https://example.com/qrcode.png"
        }))
        .unwrap();
        assert_eq!(by_ticket.name.as_deref(), Some("User"));
        assert_eq!(by_ticket.mobile.as_deref(), Some("13800000000"));
        assert_eq!(by_ticket.gender, Some(1));
        assert_eq!(by_ticket.email.as_deref(), Some("user@example.com"));
    }

    #[test]
    fn deserializes_open_work_external_contact_response() {
        let response: OpenWorkUnionIdToExternalUserId3rdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_userid_info": [{
                "corpid": "corp",
                "external_userid": "external-user"
            }]
        }))
        .unwrap();

        assert_eq!(
            response.external_userid_info[0].external_userid.as_deref(),
            Some("external-user")
        );
    }

    #[test]
    fn serializes_open_work_license_requests() {
        let new_order = serde_json::to_value(OpenWorkLicenseCreateNewOrderRequest {
            corpid: Some("corp".to_string()),
            buyer_userid: Some("buyer".to_string()),
            account_count: Some(OpenWorkLicenseAccountCount {
                base_count: Some(10),
                external_contact_count: None,
            }),
            account_duration: Some(OpenWorkLicenseAccountDuration {
                month: Some(12),
                days: None,
                new_expire_time: None,
            }),
        })
        .unwrap();
        assert_eq!(new_order["corpid"], "corp");
        assert_eq!(new_order["account_count"]["base_count"], 10);
        assert!(new_order["account_count"]
            .get("external_contact_count")
            .is_none());

        let renew = serde_json::to_value(OpenWorkLicenseCreateRenewOrderJobRequest {
            corpid: Some("corp".to_string()),
            account_list: vec![OpenWorkLicenseActiveInfo {
                active_code: Some("active-code".to_string()),
                userid: Some("user".to_string()),
                corpid: None,
                account_type: Some(1),
                status: None,
                create_time: None,
                active_time: None,
                expire_time: None,
                merge_info: None,
                share_info: None,
            }],
            jobid: None,
        })
        .unwrap();
        assert_eq!(renew["account_list"][0]["active_code"], "active-code");
        assert_eq!(renew["account_list"][0]["type"], 1);
        assert!(renew.get("jobid").is_none());

        let list = serde_json::to_value(OpenWorkLicenseListOrderRequest {
            corpid: Some("corp".to_string()),
            start_time: Some("2026-07-01".to_string()),
            end_time: Some("2026-07-09".to_string()),
            cursor: None,
            limit: Some(100),
        })
        .unwrap();
        assert_eq!(list["limit"], 100);
        assert!(list.get("cursor").is_none());

        let transfer = serde_json::to_value(OpenWorkLicenseTransferInfo {
            handover_userid: Some("old-user".to_string()),
            takeover_userid: Some("new-user".to_string()),
            errcode: None,
        })
        .unwrap();
        assert_eq!(transfer["handover_userid"], "old-user");
        assert_eq!(transfer["takeover_userid"], "new-user");
    }

    #[test]
    fn deserializes_open_work_license_responses() {
        let order_id: OpenWorkLicenseOrderIdResponse =
            serde_json::from_value(json!({ "errcode": 0, "order_id": "order-id" })).unwrap();
        assert_eq!(order_id.order_id.as_deref(), Some("order-id"));

        let job: OpenWorkLicenseCreateRenewOrderJobResponse = serde_json::from_value(json!({
            "jobid": "job-id",
            "invalid_account_list": [{ "userid": "bad-user", "errcode": 40001 }]
        }))
        .unwrap();
        assert_eq!(job.jobid.as_deref(), Some("job-id"));
        assert_eq!(job.invalid_account_list[0]["userid"], "bad-user");

        let orders: OpenWorkLicenseListOrderResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "has_more": 1,
            "order_list": [{ "order_id": "order-id", "order_status": 1 }]
        }))
        .unwrap();
        assert_eq!(orders.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(orders.order_list[0]["order_id"], "order-id");

        let accounts: OpenWorkLicenseListAccountResponse = serde_json::from_value(json!({
            "account_list": [{
                "active_code": "active-code",
                "userid": "user",
                "type": 1,
                "status": 2
            }]
        }))
        .unwrap();
        assert_eq!(
            accounts.account_list[0].active_code.as_deref(),
            Some("active-code")
        );
        assert_eq!(accounts.account_list[0].account_type, Some(1));

        let active: OpenWorkLicenseActiveInfoResponse = serde_json::from_value(json!({
            "active_info": { "active_code": "active-code", "userid": "user" }
        }))
        .unwrap();
        assert_eq!(
            active.active_info.expect("active_info").userid.as_deref(),
            Some("user")
        );

        let transfer: OpenWorkLicenseTransferResponse = serde_json::from_value(json!({
            "transfer_result": [{
                "handover_userid": "old-user",
                "takeover_userid": "new-user",
                "errcode": 0
            }]
        }))
        .unwrap();
        assert_eq!(
            transfer.transfer_result[0].takeover_userid.as_deref(),
            Some("new-user")
        );

        let license: OpenWorkLicenseInfoResponse = serde_json::from_value(json!({
            "license_status": 1,
            "license_check_time": 1800000000,
            "trail_info": { "start_time": 1800000000, "end_time": 1807776000 }
        }))
        .unwrap();
        assert_eq!(license.license_status, Some(1));
        assert_eq!(license.trail_info.unwrap()["end_time"], 1_807_776_000);

        let auto_active: OpenWorkLicenseAutoActiveStatusResponse =
            serde_json::from_value(json!({ "auto_active_status": 1 })).unwrap();
        assert_eq!(auto_active.auto_active_status, Some(1));
    }
}
