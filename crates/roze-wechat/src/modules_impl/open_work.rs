use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::Result,
    modules::{DomainModule, PlatformClient},
    types::CallbackMessage,
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

    pub fn parse_server_event_xml(xml: &str) -> Result<OpenWorkServerEvent> {
        Ok(OpenWorkServerEvent::from_callback_message(
            CallbackMessage::parse_xml(xml)?,
        ))
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentQueryAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentAuthorizationSummary {
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
pub struct OpenWorkComponentAuthorizersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub list: Vec<OpenWorkComponentAuthorizationSummary>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone)]
pub enum OpenWorkServerEvent {
    SuiteTicket {
        suite_id: Option<String>,
        suite_ticket: Option<String>,
        create_time: Option<i64>,
    },
    CreateAuth {
        suite_id: Option<String>,
        auth_code: Option<String>,
        create_time: Option<i64>,
    },
    ChangeAuth {
        suite_id: Option<String>,
        auth_corp_id: Option<String>,
        create_time: Option<i64>,
    },
    CancelAuth {
        suite_id: Option<String>,
        auth_corp_id: Option<String>,
        create_time: Option<i64>,
    },
    ResetPermanentCode {
        suite_id: Option<String>,
        auth_corp_id: Option<String>,
        create_time: Option<i64>,
    },
    Unknown {
        info_type: Option<String>,
        message: Box<CallbackMessage>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkServerEventKind {
    SuiteTicket,
    CreateAuth,
    ChangeAuth,
    CancelAuth,
    ResetPermanentCode,
    Unknown,
}

impl OpenWorkServerEventKind {
    pub fn info_type(self) -> Option<&'static str> {
        match self {
            Self::SuiteTicket => Some("suite_ticket"),
            Self::CreateAuth => Some("create_auth"),
            Self::ChangeAuth => Some("change_auth"),
            Self::CancelAuth => Some("cancel_auth"),
            Self::ResetPermanentCode => Some("reset_permanent_code"),
            Self::Unknown => None,
        }
    }

    pub fn is_auth_lifecycle(self) -> bool {
        matches!(self, Self::CreateAuth | Self::ChangeAuth | Self::CancelAuth)
    }

    pub fn is_ticket_refresh(self) -> bool {
        matches!(self, Self::SuiteTicket | Self::ResetPermanentCode)
    }
}

impl OpenWorkServerEvent {
    pub fn from_callback_message(message: CallbackMessage) -> Self {
        match message.info_type.as_deref() {
            Some("suite_ticket") => Self::SuiteTicket {
                suite_id: message.suite_id.clone(),
                suite_ticket: message.suite_ticket.clone(),
                create_time: message.create_time,
            },
            Some("create_auth") => Self::CreateAuth {
                suite_id: message.suite_id.clone(),
                auth_code: message.authorization_code.clone(),
                create_time: message.create_time,
            },
            Some("change_auth") => Self::ChangeAuth {
                suite_id: message.suite_id.clone(),
                auth_corp_id: message.auth_corp_id.clone(),
                create_time: message.create_time,
            },
            Some("cancel_auth") => Self::CancelAuth {
                suite_id: message.suite_id.clone(),
                auth_corp_id: message.auth_corp_id.clone(),
                create_time: message.create_time,
            },
            Some("reset_permanent_code") => Self::ResetPermanentCode {
                suite_id: message.suite_id.clone(),
                auth_corp_id: message.auth_corp_id.clone(),
                create_time: message.create_time,
            },
            _ => Self::Unknown {
                info_type: message.info_type.clone(),
                message: Box::new(message),
            },
        }
    }

    pub fn kind(&self) -> OpenWorkServerEventKind {
        match self {
            Self::SuiteTicket { .. } => OpenWorkServerEventKind::SuiteTicket,
            Self::CreateAuth { .. } => OpenWorkServerEventKind::CreateAuth,
            Self::ChangeAuth { .. } => OpenWorkServerEventKind::ChangeAuth,
            Self::CancelAuth { .. } => OpenWorkServerEventKind::CancelAuth,
            Self::ResetPermanentCode { .. } => OpenWorkServerEventKind::ResetPermanentCode,
            Self::Unknown { .. } => OpenWorkServerEventKind::Unknown,
        }
    }

    pub fn info_type(&self) -> Option<&str> {
        match self {
            Self::SuiteTicket { .. } => Some("suite_ticket"),
            Self::CreateAuth { .. } => Some("create_auth"),
            Self::ChangeAuth { .. } => Some("change_auth"),
            Self::CancelAuth { .. } => Some("cancel_auth"),
            Self::ResetPermanentCode { .. } => Some("reset_permanent_code"),
            Self::Unknown { info_type, .. } => info_type.as_deref(),
        }
    }

    pub fn is_auth_lifecycle(&self) -> bool {
        self.kind().is_auth_lifecycle()
    }

    pub fn is_ticket_refresh(&self) -> bool {
        self.kind().is_ticket_refresh()
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkSessionAuthTypeKind {
    Official,
    Test,
    Other,
}

impl OpenWorkSessionAuthTypeKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Official,
            1 => Self::Test,
            _ => Self::Other,
        }
    }

    pub fn is_test(self) -> bool {
        matches!(self, Self::Test)
    }
}

impl OpenWorkSessionInfo {
    pub fn auth_type_kind(&self) -> Option<OpenWorkSessionAuthTypeKind> {
        self.auth_type.map(OpenWorkSessionAuthTypeKind::from_code)
    }

    pub fn is_test_auth(&self) -> bool {
        self.auth_type_kind()
            .is_some_and(OpenWorkSessionAuthTypeKind::is_test)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthCorpInfo {
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub corp_name: Option<String>,
    #[serde(default)]
    pub corp_type: Option<String>,
    #[serde(default)]
    pub corp_square_logo_url: Option<String>,
    #[serde(default)]
    pub corp_user_max: Option<i64>,
    #[serde(default)]
    pub corp_agent_max: Option<i64>,
    #[serde(default)]
    pub corp_full_name: Option<String>,
    #[serde(default)]
    pub verified_end_time: Option<i64>,
    #[serde(default)]
    pub subject_type: Option<i64>,
    #[serde(default)]
    pub corp_wxqrcode: Option<String>,
    #[serde(default)]
    pub corp_scale: Option<String>,
    #[serde(default)]
    pub corp_industry: Option<String>,
    #[serde(default)]
    pub corp_sub_industry: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthInfo {
    #[serde(default)]
    pub agent: Vec<OpenWorkAuthAgent>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthAgent {
    #[serde(default)]
    pub agentid: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub square_logo_url: Option<String>,
    #[serde(default)]
    pub round_logo_url: Option<String>,
    #[serde(default)]
    pub appid: Option<i64>,
    #[serde(default)]
    pub auth_mode: Option<i64>,
    #[serde(default)]
    pub privilege: Option<Value>,
    #[serde(default)]
    pub shared_from: Option<Value>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkAuthModeKind {
    Admin,
    Member,
    Other,
}

impl OpenWorkAuthModeKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Admin,
            1 => Self::Member,
            _ => Self::Other,
        }
    }

    pub fn is_member_auth(self) -> bool {
        matches!(self, Self::Member)
    }
}

impl OpenWorkAuthAgent {
    pub fn auth_mode_kind(&self) -> Option<OpenWorkAuthModeKind> {
        self.auth_mode.map(OpenWorkAuthModeKind::from_code)
    }

    pub fn is_member_auth(&self) -> bool {
        self.auth_mode_kind()
            .is_some_and(OpenWorkAuthModeKind::is_member_auth)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthUserInfo {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkRegisterCodeInfo {
    #[serde(default)]
    pub register_code: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub follow_user: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Value,
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
    pub auth_corp_info: Option<OpenWorkAuthCorpInfo>,
    #[serde(default)]
    pub auth_info: Option<OpenWorkAuthInfo>,
    #[serde(default)]
    pub auth_user_info: Option<OpenWorkAuthUserInfo>,
    #[serde(default)]
    pub register_code_info: Option<OpenWorkRegisterCodeInfo>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub dealer_corp_info: Option<OpenWorkAuthCorpInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub merge_info: Option<OpenWorkLicenseActiveMergeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_info: Option<OpenWorkLicenseActiveShareInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveMergeInfo {
    #[serde(default)]
    pub to_active_code: Option<String>,
    #[serde(default)]
    pub from_active_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveShareInfo {
    #[serde(default)]
    pub to_corpid: Option<String>,
    #[serde(default)]
    pub from_corpid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseAccountTypeKind {
    Basic,
    ExternalContact,
    Other,
}

impl OpenWorkLicenseAccountTypeKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Basic,
            2 => Self::ExternalContact,
            _ => Self::Other,
        }
    }

    pub fn includes_customer_contact(self) -> bool {
        matches!(self, Self::ExternalContact)
    }
}

impl OpenWorkLicenseActiveInfo {
    pub fn account_type_kind(&self) -> Option<OpenWorkLicenseAccountTypeKind> {
        self.account_type
            .map(OpenWorkLicenseAccountTypeKind::from_code)
    }

    pub fn status_kind(&self) -> Option<OpenWorkLicenseActiveStatusKind> {
        self.status.map(OpenWorkLicenseActiveStatusKind::from_code)
    }

    pub fn is_active(&self) -> bool {
        self.status_kind()
            .is_some_and(OpenWorkLicenseActiveStatusKind::is_active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseActiveStatusKind {
    Unbound,
    Active,
    Expired,
    PendingTransfer,
    Merged,
    SharedDownstream,
    Other,
}

impl OpenWorkLicenseActiveStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Unbound,
            2 => Self::Active,
            3 => Self::Expired,
            4 => Self::PendingTransfer,
            5 => Self::Merged,
            6 => Self::SharedDownstream,
            _ => Self::Other,
        }
    }

    pub fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_assignable(self) -> bool {
        matches!(self, Self::Unbound)
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Expired | Self::Merged)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseTransferInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handover_userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takeover_userid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseInvalidAccount {
    #[serde(default)]
    pub active_code: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseOrder {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_type: Option<i64>,
    #[serde(default)]
    pub order_status: Option<i64>,
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub buyer_userid: Option<String>,
    #[serde(default)]
    pub account_count: Option<OpenWorkLicenseAccountCount>,
    #[serde(default)]
    pub account_duration: Option<OpenWorkLicenseAccountDuration>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub pay_time: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseOrderTypeKind {
    NewAccount,
    RenewAccount,
    HistoricalMigration,
    MultiCorpNewAccount,
    Other,
}

impl OpenWorkLicenseOrderTypeKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::NewAccount,
            2 => Self::RenewAccount,
            5 => Self::HistoricalMigration,
            8 => Self::MultiCorpNewAccount,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseOrderStatusKind {
    PendingPayment,
    Paid,
    Canceled,
    Expired,
    Refunding,
    Refunded,
    RefundRejected,
    Invalid,
    Other,
}

impl OpenWorkLicenseOrderStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::PendingPayment,
            1 => Self::Paid,
            2 => Self::Canceled,
            3 => Self::Expired,
            4 => Self::Refunding,
            5 => Self::Refunded,
            6 => Self::RefundRejected,
            7 => Self::Invalid,
            _ => Self::Other,
        }
    }

    pub fn is_terminal(self) -> bool {
        !matches!(self, Self::PendingPayment | Self::Refunding)
    }

    pub fn is_success(self) -> bool {
        matches!(self, Self::Paid)
    }
}

impl OpenWorkLicenseOrder {
    pub fn order_type_kind(&self) -> Option<OpenWorkLicenseOrderTypeKind> {
        self.order_type.map(OpenWorkLicenseOrderTypeKind::from_code)
    }

    pub fn order_status_kind(&self) -> Option<OpenWorkLicenseOrderStatusKind> {
        self.order_status
            .map(OpenWorkLicenseOrderStatusKind::from_code)
    }

    pub fn is_paid(&self) -> bool {
        self.order_status_kind()
            .is_some_and(OpenWorkLicenseOrderStatusKind::is_success)
    }
}

impl OpenWorkLicenseListOrderResponse {
    pub fn has_more(&self) -> bool {
        self.has_more == Some(1)
    }
}

impl OpenWorkLicenseListAccountResponse {
    pub fn has_more(&self) -> bool {
        self.has_more == Some(1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseTrialInfo {
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub trial_status: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseOrderIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub invalid_account_list: Vec<OpenWorkLicenseInvalidAccount>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub order_list: Vec<OpenWorkLicenseOrder>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order: Option<OpenWorkLicenseOrder>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_info: Option<OpenWorkLicenseActiveInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseActiveInfoListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_info_list: Vec<OpenWorkLicenseActiveInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseUserActiveStatusKind {
    Inactive,
    Active,
    Other,
}

impl OpenWorkLicenseUserActiveStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Inactive,
            1 => Self::Active,
            _ => Self::Other,
        }
    }

    pub fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

impl OpenWorkLicenseUserActiveInfoResponse {
    pub fn active_status_kind(&self) -> Option<OpenWorkLicenseUserActiveStatusKind> {
        self.active_status
            .map(OpenWorkLicenseUserActiveStatusKind::from_code)
    }

    pub fn is_active(&self) -> bool {
        self.active_status_kind()
            .is_some_and(OpenWorkLicenseUserActiveStatusKind::is_active)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseTransferResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub transfer_result: Vec<OpenWorkLicenseTransferInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub trail_info: Option<OpenWorkLicenseTrialInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseStatusKind {
    Disabled,
    Enabled,
    Other,
}

impl OpenWorkLicenseStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Disabled,
            1 => Self::Enabled,
            _ => Self::Other,
        }
    }

    pub fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

impl OpenWorkLicenseInfoResponse {
    pub fn license_status_kind(&self) -> Option<OpenWorkLicenseStatusKind> {
        self.license_status
            .map(OpenWorkLicenseStatusKind::from_code)
    }

    pub fn is_license_check_enabled(&self) -> bool {
        self.license_status_kind()
            .is_some_and(OpenWorkLicenseStatusKind::is_enabled)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseAutoActiveStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub auto_active_status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkLicenseAutoActiveStatusKind {
    Disabled,
    Enabled,
    Other,
}

impl OpenWorkLicenseAutoActiveStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Disabled,
            1 => Self::Enabled,
            _ => Self::Other,
        }
    }

    pub fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

impl OpenWorkLicenseAutoActiveStatusResponse {
    pub fn auto_active_status_kind(&self) -> Option<OpenWorkLicenseAutoActiveStatusKind> {
        self.auto_active_status
            .map(OpenWorkLicenseAutoActiveStatusKind::from_code)
    }

    pub fn is_enabled(&self) -> bool {
        self.auto_active_status_kind()
            .is_some_and(OpenWorkLicenseAutoActiveStatusKind::is_enabled)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parses_open_work_server_events() {
        let suite_ticket = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[suite_ticket]]></InfoType>
                <CreateTime>1800000000</CreateTime>
                <SuiteTicket><![CDATA[ticket]]></SuiteTicket>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(suite_ticket.kind(), OpenWorkServerEventKind::SuiteTicket);
        assert_eq!(suite_ticket.kind().info_type(), Some("suite_ticket"));
        assert!(suite_ticket.is_ticket_refresh());
        assert!(!suite_ticket.is_auth_lifecycle());
        match suite_ticket {
            OpenWorkServerEvent::SuiteTicket {
                suite_id,
                suite_ticket,
                create_time,
            } => {
                assert_eq!(suite_id.as_deref(), Some("suite-id"));
                assert_eq!(suite_ticket.as_deref(), Some("ticket"));
                assert_eq!(create_time, Some(1_800_000_000));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let create_auth = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[create_auth]]></InfoType>
                <CreateTime>1800000001</CreateTime>
                <AuthorizationCode><![CDATA[auth-code]]></AuthorizationCode>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(create_auth.info_type(), Some("create_auth"));
        assert_eq!(create_auth.kind(), OpenWorkServerEventKind::CreateAuth);
        assert!(create_auth.is_auth_lifecycle());
        assert!(!create_auth.is_ticket_refresh());
        assert!(OpenWorkServerEventKind::ChangeAuth.is_auth_lifecycle());
        assert!(OpenWorkServerEventKind::CancelAuth.is_auth_lifecycle());
        assert!(OpenWorkServerEventKind::ResetPermanentCode.is_ticket_refresh());
        match create_auth {
            OpenWorkServerEvent::CreateAuth {
                suite_id,
                auth_code,
                create_time,
            } => {
                assert_eq!(suite_id.as_deref(), Some("suite-id"));
                assert_eq!(auth_code.as_deref(), Some("auth-code"));
                assert_eq!(create_time, Some(1_800_000_001));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let unknown = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[new_event]]></InfoType>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(unknown.kind(), OpenWorkServerEventKind::Unknown);
        assert_eq!(unknown.kind().info_type(), None);
        assert!(!unknown.is_auth_lifecycle());
        assert!(!unknown.is_ticket_refresh());
        match unknown {
            OpenWorkServerEvent::Unknown { info_type, message } => {
                assert_eq!(info_type.as_deref(), Some("new_event"));
                assert_eq!(message.suite_id.as_deref(), Some("suite-id"));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

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
            "expires_in": 1200,
            "request_id": "component-preauth"
        }))
        .unwrap();
        assert_eq!(preauth.pre_auth_code.as_deref(), Some("pre-auth"));
        assert_eq!(preauth.expires_in, Some(1200));
        assert_eq!(preauth.extra["request_id"], "component-preauth");

        let query: OpenWorkComponentQueryAuthResponse = serde_json::from_value(json!({
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "authorizer_access_token": "token"
            },
            "request_id": "component-query"
        }))
        .unwrap();
        assert_eq!(
            query.authorization_info.as_ref().unwrap()["authorizer_appid"],
            "wx-authorizer"
        );
        assert_eq!(query.extra["request_id"], "component-query");

        let info: OpenWorkComponentAuthorizerInfoResponse = serde_json::from_value(json!({
            "authorizer_info": { "nick_name": "Corp App" },
            "authorization_info": { "authorizer_appid": "wx-authorizer" },
            "request_id": "component-info"
        }))
        .unwrap();
        assert_eq!(
            info.authorizer_info.as_ref().unwrap()["nick_name"],
            "Corp App"
        );
        assert_eq!(info.extra["request_id"], "component-info");

        let list: OpenWorkComponentAuthorizersResponse = serde_json::from_value(json!({
            "total_count": 1,
            "request_id": "component-list",
            "list": [{
                "authorizer_appid": "wx-authorizer",
                "refresh_token": "refresh",
                "auth_time": 1800000000,
                "summary_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.extra["request_id"], "component-list");
        assert_eq!(list.list[0].refresh_token.as_deref(), Some("refresh"));
        assert_eq!(list.list[0].extra["summary_extra"], "retained");

        let option: OpenWorkComponentAuthorizerOptionResponse = serde_json::from_value(json!({
            "authorizer_appid": "wx-authorizer",
            "option_name": "voice_recognize",
            "option_value": "1",
            "request_id": "component-option"
        }))
        .unwrap();
        assert_eq!(option.option_name.as_deref(), Some("voice_recognize"));
        assert_eq!(option.option_value.as_deref(), Some("1"));
        assert_eq!(option.extra["request_id"], "component-option");
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
        assert_eq!(
            OpenWorkSessionInfo {
                appid: vec![1, 2],
                auth_type: Some(1),
            }
            .auth_type_kind(),
            Some(OpenWorkSessionAuthTypeKind::Test)
        );
        assert!(OpenWorkSessionInfo {
            appid: vec![1, 2],
            auth_type: Some(1),
        }
        .is_test_auth());
        assert_eq!(
            OpenWorkSessionAuthTypeKind::from_code(0),
            OpenWorkSessionAuthTypeKind::Official
        );

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
            "auth_corp_info": { "corpid": "corp", "corp_name": "Corp", "corp_region": "CN" },
            "auth_info": { "agent": [{ "agentid": 100001, "name": "App", "auth_mode": 1, "edition": "pro" }], "auth_scope": "all" },
            "auth_user_info": { "userid": "admin", "name": "Admin", "role": "owner" },
            "register_code_info": {
                "register_code": "register",
                "template_id": "tpl",
                "state": "state",
                "register_source": "suite"
            },
            "state": "state",
            "dealer_corp_info": { "corpid": "dealer", "corp_name": "Dealer", "dealer_type": "direct" },
            "request_id": "permanent"
        }))
        .unwrap();
        assert_eq!(permanent.access_token.as_deref(), Some("corp-token"));
        assert_eq!(permanent.extra["request_id"], "permanent");
        let auth_corp = permanent.auth_corp_info.expect("auth corp");
        assert_eq!(auth_corp.corpid.as_deref(), Some("corp"));
        assert_eq!(auth_corp.corp_name.as_deref(), Some("Corp"));
        assert_eq!(auth_corp.extra["corp_region"], "CN");
        let auth_info = permanent.auth_info.expect("auth info");
        assert_eq!(auth_info.extra["auth_scope"], "all");
        assert_eq!(auth_info.agent[0].agentid, Some(100001));
        assert_eq!(auth_info.agent[0].name.as_deref(), Some("App"));
        assert_eq!(
            auth_info.agent[0].auth_mode_kind(),
            Some(OpenWorkAuthModeKind::Member)
        );
        assert!(auth_info.agent[0].is_member_auth());
        assert_eq!(
            OpenWorkAuthModeKind::from_code(0),
            OpenWorkAuthModeKind::Admin
        );
        assert_eq!(auth_info.agent[0].extra["edition"], "pro");
        let auth_user = permanent.auth_user_info.expect("auth user");
        assert_eq!(auth_user.userid.as_deref(), Some("admin"));
        assert_eq!(auth_user.name.as_deref(), Some("Admin"));
        assert_eq!(auth_user.extra["role"], "owner");
        let register = permanent.register_code_info.expect("register code");
        assert_eq!(register.register_code.as_deref(), Some("register"));
        assert_eq!(register.template_id.as_deref(), Some("tpl"));
        assert_eq!(register.state.as_deref(), Some("state"));
        assert_eq!(register.extra["register_source"], "suite");
        let dealer = permanent.dealer_corp_info.expect("dealer corp");
        assert_eq!(dealer.corpid.as_deref(), Some("dealer"));
        assert_eq!(dealer.corp_name.as_deref(), Some("Dealer"));
        assert_eq!(dealer.extra["dealer_type"], "direct");

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
            "expires_in": 1200,
            "request_id": "pre-auth"
        }))
        .unwrap();
        assert_eq!(pre_auth.pre_auth_code.as_deref(), Some("pre-auth"));
        assert_eq!(pre_auth.expires_in, Some(1200));
        assert_eq!(pre_auth.extra["request_id"], "pre-auth");

        let permanent: OpenWorkPermanentCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "access_token": "access-token",
            "expires_in": 7200,
            "permanent_code": "permanent",
            "auth_corp_info": { "corpid": "corp", "corp_alias": "alias" },
            "auth_info": { "agent": [{ "agentid": 100001, "suite_flag": true }], "auth_version": 1 },
            "auth_user_info": { "userid": "admin", "user_source": "suite" },
            "request_id": "permanent-auth"
        }))
        .unwrap();
        assert_eq!(permanent.permanent_code.as_deref(), Some("permanent"));
        assert_eq!(permanent.extra["request_id"], "permanent-auth");
        assert_eq!(
            permanent
                .auth_corp_info
                .as_ref()
                .and_then(|corp| corp.corpid.as_deref()),
            Some("corp")
        );
        assert_eq!(
            permanent.auth_corp_info.as_ref().unwrap().extra["corp_alias"],
            "alias"
        );
        assert_eq!(
            permanent.auth_info.as_ref().unwrap().extra["auth_version"],
            1
        );
        assert_eq!(
            permanent.auth_info.as_ref().unwrap().agent[0].extra["suite_flag"],
            true
        );
        assert_eq!(
            permanent
                .auth_user_info
                .as_ref()
                .and_then(|user| user.userid.as_deref()),
            Some("admin")
        );
        assert_eq!(
            permanent.auth_user_info.as_ref().unwrap().extra["user_source"],
            "suite"
        );

        let corp_token: OpenWorkCorpTokenResponse = serde_json::from_value(json!({
            "errcode": 0,
            "access_token": "corp-access-token",
            "expires_in": 7200,
            "request_id": "corp-token"
        }))
        .unwrap();
        assert_eq!(
            corp_token.access_token.as_deref(),
            Some("corp-access-token")
        );
        assert_eq!(corp_token.extra["request_id"], "corp-token");

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
                extra: Value::Null,
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
            extra: Value::Null,
        })
        .unwrap();
        assert_eq!(transfer["handover_userid"], "old-user");
        assert_eq!(transfer["takeover_userid"], "new-user");
    }

    #[test]
    fn deserializes_open_work_license_responses() {
        let order_id: OpenWorkLicenseOrderIdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "order_id": "order-id",
            "request_id": "order-create"
        }))
        .unwrap();
        assert_eq!(order_id.order_id.as_deref(), Some("order-id"));
        assert_eq!(order_id.extra["request_id"], "order-create");

        let job: OpenWorkLicenseCreateRenewOrderJobResponse = serde_json::from_value(json!({
            "jobid": "job-id",
            "job_status": "pending",
            "invalid_account_list": [{
                "active_code": "active-bad",
                "userid": "bad-user",
                "errcode": 40001,
                "errmsg": "invalid",
                "invalid_reason": "expired"
            }]
        }))
        .unwrap();
        assert_eq!(job.jobid.as_deref(), Some("job-id"));
        assert_eq!(job.extra["job_status"], "pending");
        assert_eq!(
            job.invalid_account_list[0].active_code.as_deref(),
            Some("active-bad")
        );
        assert_eq!(
            job.invalid_account_list[0].userid.as_deref(),
            Some("bad-user")
        );
        assert_eq!(job.invalid_account_list[0].errcode, Some(40001));
        assert_eq!(
            job.invalid_account_list[0].extra["invalid_reason"],
            "expired"
        );

        let orders: OpenWorkLicenseListOrderResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "has_more": 1,
            "total": 1,
            "order_list": [{
                "order_id": "order-id",
                "order_type": 1,
                "order_status": 1,
                "corpid": "corp",
                "buyer_userid": "buyer",
                "account_count": { "base_count": 10 },
                "account_duration": { "month": 12 },
                "price": 100,
                "create_time": 1800000000,
                "invoice_status": 1
            }]
        }))
        .unwrap();
        assert_eq!(orders.next_cursor.as_deref(), Some("cursor"));
        assert!(orders.has_more());
        assert_eq!(orders.extra["total"], 1);
        assert_eq!(orders.order_list[0].order_id.as_deref(), Some("order-id"));
        assert_eq!(orders.order_list[0].order_status, Some(1));
        assert_eq!(
            orders.order_list[0].order_type_kind(),
            Some(OpenWorkLicenseOrderTypeKind::NewAccount)
        );
        assert_eq!(
            orders.order_list[0].order_status_kind(),
            Some(OpenWorkLicenseOrderStatusKind::Paid)
        );
        assert!(orders.order_list[0].is_paid());
        assert!(orders.order_list[0]
            .order_status_kind()
            .expect("order status")
            .is_terminal());
        assert_eq!(
            OpenWorkLicenseOrderTypeKind::from_code(2),
            OpenWorkLicenseOrderTypeKind::RenewAccount
        );
        assert_eq!(
            OpenWorkLicenseOrderTypeKind::from_code(5),
            OpenWorkLicenseOrderTypeKind::HistoricalMigration
        );
        assert_eq!(
            OpenWorkLicenseOrderTypeKind::from_code(8),
            OpenWorkLicenseOrderTypeKind::MultiCorpNewAccount
        );
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(0),
            OpenWorkLicenseOrderStatusKind::PendingPayment
        );
        assert!(!OpenWorkLicenseOrderStatusKind::PendingPayment.is_terminal());
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(2),
            OpenWorkLicenseOrderStatusKind::Canceled
        );
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(3),
            OpenWorkLicenseOrderStatusKind::Expired
        );
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(4),
            OpenWorkLicenseOrderStatusKind::Refunding
        );
        assert!(!OpenWorkLicenseOrderStatusKind::Refunding.is_terminal());
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(5),
            OpenWorkLicenseOrderStatusKind::Refunded
        );
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(6),
            OpenWorkLicenseOrderStatusKind::RefundRejected
        );
        assert_eq!(
            OpenWorkLicenseOrderStatusKind::from_code(7),
            OpenWorkLicenseOrderStatusKind::Invalid
        );
        assert_eq!(orders.order_list[0].extra["invoice_status"], 1);
        assert_eq!(
            orders.order_list[0]
                .account_count
                .as_ref()
                .and_then(|count| count.base_count),
            Some(10)
        );

        let order: OpenWorkLicenseOrderResponse = serde_json::from_value(json!({
            "trace_id": "order-detail",
            "order": {
                "order_id": "order-id",
                "order_status": 2,
                "pay_time": 1800000100,
                "refund_status": 0
            }
        }))
        .unwrap();
        assert_eq!(order.extra["trace_id"], "order-detail");
        assert_eq!(
            order
                .order
                .as_ref()
                .and_then(|item| item.order_id.as_deref()),
            Some("order-id")
        );
        assert_eq!(
            order.order.as_ref().and_then(|item| item.pay_time),
            Some(1_800_000_100)
        );
        assert_eq!(order.order.as_ref().unwrap().extra["refund_status"], 0);

        let accounts: OpenWorkLicenseListAccountResponse = serde_json::from_value(json!({
            "next_cursor": "account-cursor",
            "has_more": 1,
            "account_total": 1,
            "account_list": [{
                "active_code": "active-code",
                "userid": "user",
                "type": 1,
                "status": 2,
                "merge_info": {
                    "to_active_code": "merged-active-code",
                    "merge_reason": "duplicate-user"
                },
                "share_info": {
                    "to_corpid": "downstream-corp",
                    "share_channel": "chain"
                },
                "account_revision": 3
            }]
        }))
        .unwrap();
        assert_eq!(accounts.next_cursor.as_deref(), Some("account-cursor"));
        assert!(accounts.has_more());
        assert_eq!(accounts.extra["account_total"], 1);
        assert_eq!(
            accounts.account_list[0].active_code.as_deref(),
            Some("active-code")
        );
        assert_eq!(accounts.account_list[0].account_type, Some(1));
        assert_eq!(
            accounts.account_list[0].status_kind(),
            Some(OpenWorkLicenseActiveStatusKind::Active)
        );
        assert!(accounts.account_list[0].is_active());
        let merge_info = accounts.account_list[0]
            .merge_info
            .as_ref()
            .expect("merge info");
        assert_eq!(
            merge_info.to_active_code.as_deref(),
            Some("merged-active-code")
        );
        assert_eq!(merge_info.extra["merge_reason"], "duplicate-user");
        let share_info = accounts.account_list[0]
            .share_info
            .as_ref()
            .expect("share info");
        assert_eq!(share_info.to_corpid.as_deref(), Some("downstream-corp"));
        assert_eq!(share_info.extra["share_channel"], "chain");
        assert_eq!(accounts.account_list[0].extra["account_revision"], 3);
        assert_eq!(
            accounts.account_list[0].account_type_kind(),
            Some(OpenWorkLicenseAccountTypeKind::Basic)
        );
        assert!(!accounts.account_list[0]
            .account_type_kind()
            .expect("account type")
            .includes_customer_contact());
        assert_eq!(
            OpenWorkLicenseAccountTypeKind::from_code(2),
            OpenWorkLicenseAccountTypeKind::ExternalContact
        );
        assert!(OpenWorkLicenseAccountTypeKind::ExternalContact.includes_customer_contact());
        assert_eq!(
            OpenWorkLicenseActiveStatusKind::from_code(1),
            OpenWorkLicenseActiveStatusKind::Unbound
        );
        assert!(OpenWorkLicenseActiveStatusKind::Unbound.is_assignable());
        assert_eq!(
            OpenWorkLicenseActiveStatusKind::from_code(3),
            OpenWorkLicenseActiveStatusKind::Expired
        );
        assert!(OpenWorkLicenseActiveStatusKind::Expired.is_terminal());
        assert_eq!(
            OpenWorkLicenseActiveStatusKind::from_code(4),
            OpenWorkLicenseActiveStatusKind::PendingTransfer
        );
        assert_eq!(
            OpenWorkLicenseActiveStatusKind::from_code(5),
            OpenWorkLicenseActiveStatusKind::Merged
        );
        assert_eq!(
            OpenWorkLicenseActiveStatusKind::from_code(6),
            OpenWorkLicenseActiveStatusKind::SharedDownstream
        );

        let active: OpenWorkLicenseActiveInfoResponse = serde_json::from_value(json!({
            "active_info": { "active_code": "active-code", "userid": "user" },
            "trace_id": "active-detail"
        }))
        .unwrap();
        assert_eq!(active.extra["trace_id"], "active-detail");
        assert_eq!(
            active.active_info.expect("active_info").userid.as_deref(),
            Some("user")
        );

        let transfer: OpenWorkLicenseTransferResponse = serde_json::from_value(json!({
            "trace_id": "transfer",
            "transfer_result": [{
                "handover_userid": "old-user",
                "takeover_userid": "new-user",
                "errcode": 0
            }]
        }))
        .unwrap();
        assert_eq!(transfer.extra["trace_id"], "transfer");
        assert_eq!(
            transfer.transfer_result[0].takeover_userid.as_deref(),
            Some("new-user")
        );

        let user_active: OpenWorkLicenseUserActiveInfoResponse = serde_json::from_value(json!({
            "active_status": 1,
            "active_info_list": [{
                "active_code": "active-code",
                "userid": "user",
                "type": 2
            }]
        }))
        .unwrap();
        assert_eq!(
            user_active.active_status_kind(),
            Some(OpenWorkLicenseUserActiveStatusKind::Active)
        );
        assert!(user_active.is_active());
        assert_eq!(
            user_active.active_info_list[0].account_type_kind(),
            Some(OpenWorkLicenseAccountTypeKind::ExternalContact)
        );
        assert_eq!(
            OpenWorkLicenseUserActiveStatusKind::from_code(0),
            OpenWorkLicenseUserActiveStatusKind::Inactive
        );

        let license: OpenWorkLicenseInfoResponse = serde_json::from_value(json!({
            "license_status": 1,
            "license_check_time": 1800000000,
            "license_source": "suite",
            "trail_info": {
                "start_time": 1800000000,
                "end_time": 1807776000,
                "trial_status": 1,
                "trial_plan": "standard"
            }
        }))
        .unwrap();
        assert_eq!(license.license_status, Some(1));
        assert_eq!(
            license.license_status_kind(),
            Some(OpenWorkLicenseStatusKind::Enabled)
        );
        assert!(license.is_license_check_enabled());
        assert_eq!(license.extra["license_source"], "suite");
        let trial = license.trail_info.expect("trial info");
        assert_eq!(trial.end_time, Some(1_807_776_000));
        assert_eq!(trial.trial_status, Some(1));
        assert_eq!(trial.extra["trial_plan"], "standard");

        let auto_active: OpenWorkLicenseAutoActiveStatusResponse = serde_json::from_value(json!({
            "auto_active_status": 1,
            "effective_time": 1_800_000_000
        }))
        .unwrap();
        assert_eq!(auto_active.auto_active_status, Some(1));
        assert_eq!(
            auto_active.auto_active_status_kind(),
            Some(OpenWorkLicenseAutoActiveStatusKind::Enabled)
        );
        assert!(auto_active.is_enabled());
        assert_eq!(
            OpenWorkLicenseAutoActiveStatusKind::from_code(0),
            OpenWorkLicenseAutoActiveStatusKind::Disabled
        );
        assert_eq!(auto_active.extra["effective_time"], 1_800_000_000);
    }
}
