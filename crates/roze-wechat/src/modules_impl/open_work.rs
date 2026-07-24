use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    error::{Result, WechatError},
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
        request.validate()?;
        let response: SuiteTokenResponse = self
            .inner
            .post("cgi-bin/service/get_suite_token", None, request)
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn pre_auth_code(
        &self,
        suite_access_token: impl Into<String>,
    ) -> Result<OpenWorkPreAuthCodeResponse> {
        let suite_access_token = suite_access_token.into();
        validate_open_work_auth_identifier("suite access token", &suite_access_token)?;
        let response: OpenWorkPreAuthCodeResponse = self
            .inner
            .get_with_query(
                "cgi-bin/service/get_pre_auth_code",
                Some(suite_access_token),
                Vec::new(),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn pre_auth_code_typed(
        &self,
        suite_access_token: impl Into<String>,
    ) -> Result<OpenWorkPreAuthCodeResponse> {
        self.pre_auth_code(suite_access_token).await
    }

    pub async fn permanent_code(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        let suite_access_token = suite_access_token.into();
        let auth_code = auth_code.into();
        validate_open_work_auth_identifier("suite access token", &suite_access_token)?;
        validate_open_work_auth_identifier("authorization code", &auth_code)?;
        let response: OpenWorkPermanentCodeResponse = self
            .inner
            .post(
                "cgi-bin/service/get_permanent_code",
                Some(suite_access_token),
                json!({ "auth_code": auth_code }),
            )
            .await?;
        response.validate_permanent_code("open-work get permanent code")?;
        Ok(response)
    }

    pub async fn permanent_code_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.permanent_code(suite_access_token, auth_code).await
    }

    pub async fn set_session_info(
        &self,
        suite_access_token: impl Into<String>,
        request: OpenWorkSetSessionInfoRequest,
    ) -> Result<OpenWorkStatusResponse> {
        let suite_access_token = suite_access_token.into();
        validate_open_work_auth_identifier("suite access token", &suite_access_token)?;
        request.validate()?;
        let response: OpenWorkStatusResponse = self
            .inner
            .post(
                "cgi-bin/service/set_session_info",
                Some(suite_access_token),
                request,
            )
            .await?;
        response.validate_for("open-work set session info")?;
        Ok(response)
    }

    pub async fn permanent_code_v2(
        &self,
        suite_access_token: impl Into<String>,
        auth_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        let suite_access_token = suite_access_token.into();
        let auth_code = auth_code.into();
        validate_open_work_auth_identifier("suite access token", &suite_access_token)?;
        validate_open_work_auth_identifier("authorization code", &auth_code)?;
        let response: OpenWorkPermanentCodeResponse = self
            .inner
            .post(
                "cgi-bin/service/v2/get_permanent_code",
                Some(suite_access_token),
                json!({ "auth_code": auth_code }),
            )
            .await?;
        response.validate_permanent_code("open-work get permanent code v2")?;
        Ok(response)
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
        let suite_access_token = suite_access_token.into();
        let auth_corpid = auth_corpid.into();
        let permanent_code = permanent_code.into();
        validate_open_work_auth_credentials(&suite_access_token, &auth_corpid, &permanent_code)?;
        let response: OpenWorkPermanentCodeResponse = self
            .inner
            .post(
                "cgi-bin/service/get_auth_info",
                Some(suite_access_token),
                json!({
                    "auth_corpid": auth_corpid,
                    "permanent_code": permanent_code,
                }),
            )
            .await?;
        response.validate_auth_info("open-work get authorization info")?;
        Ok(response)
    }

    pub async fn auth_info_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        self.auth_info(suite_access_token, auth_corpid, permanent_code)
            .await
    }

    pub async fn auth_info_v2(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkPermanentCodeResponse> {
        let suite_access_token = suite_access_token.into();
        let auth_corpid = auth_corpid.into();
        let permanent_code = permanent_code.into();
        validate_open_work_auth_credentials(&suite_access_token, &auth_corpid, &permanent_code)?;
        let response: OpenWorkPermanentCodeResponse = self
            .inner
            .post(
                "cgi-bin/service/v2/get_auth_info",
                Some(suite_access_token),
                json!({
                    "auth_corpid": auth_corpid,
                    "permanent_code": permanent_code,
                }),
            )
            .await?;
        response.validate_auth_info("open-work get authorization info v2")?;
        Ok(response)
    }

    pub async fn corp_token(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkCorpTokenResponse> {
        let suite_access_token = suite_access_token.into();
        let auth_corpid = auth_corpid.into();
        let permanent_code = permanent_code.into();
        validate_open_work_auth_credentials(&suite_access_token, &auth_corpid, &permanent_code)?;
        let response: OpenWorkCorpTokenResponse = self
            .inner
            .post(
                "cgi-bin/service/get_corp_token",
                Some(suite_access_token),
                json!({
                    "auth_corpid": auth_corpid,
                    "permanent_code": permanent_code,
                }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn corp_token_typed(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<OpenWorkCorpTokenResponse> {
        self.corp_token(suite_access_token, auth_corpid, permanent_code)
            .await
    }

    pub async fn user_id_to_open_user_id(
        &self,
        suite_access_token: impl Into<String>,
        user_id_list: Vec<String>,
    ) -> Result<OpenWorkUserIdToOpenUserIdResponse> {
        let suite_access_token = suite_access_token.into();
        validate_open_work_auth_identifier("suite access token", &suite_access_token)?;
        validate_open_work_auth_identifier_batch("user id", &user_id_list, 1_000)?;
        let response: OpenWorkUserIdToOpenUserIdResponse = self
            .inner
            .post(
                "cgi-bin/batch/userid_to_openuserid",
                Some(suite_access_token),
                json!({ "userid_list": user_id_list }),
            )
            .await?;
        response.validate()?;
        Ok(response)
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
        request.validate()?;
        let response: OpenWorkLicenseOrderIdResponse = self
            .inner
            .post(
                "cgi-bin/license/create_new_order",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate_for("open-work license create new order")?;
        Ok(response)
    }

    pub async fn create_license_renew_order_job(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseCreateRenewOrderJobRequest,
    ) -> Result<OpenWorkLicenseCreateRenewOrderJobResponse> {
        request.validate()?;
        let response: OpenWorkLicenseCreateRenewOrderJobResponse = self
            .inner
            .post(
                "cgi-bin/license/create_renew_order_job",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn submit_license_order_job(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseSubmitOrderJobRequest,
    ) -> Result<OpenWorkLicenseOrderIdResponse> {
        request.validate()?;
        let response: OpenWorkLicenseOrderIdResponse = self
            .inner
            .post(
                "cgi-bin/license/submit_order_job",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate_for("open-work license submit renewal order")?;
        Ok(response)
    }

    pub async fn list_license_order(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListOrderRequest,
    ) -> Result<OpenWorkLicenseListOrderResponse> {
        request.validate()?;
        let response: OpenWorkLicenseListOrderResponse = self
            .inner
            .post(
                "cgi-bin/license/list_order",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_license_order(
        &self,
        provider_access_token: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseOrderResponse> {
        let order_id = order_id.into();
        validate_open_work_license_identifier("order id", &order_id)?;
        let response: OpenWorkLicenseOrderResponse = self
            .inner
            .post(
                "cgi-bin/license/get_order",
                Some(provider_access_token.into()),
                json!({ "order_id": order_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn list_license_order_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListOrderAccountRequest,
    ) -> Result<OpenWorkLicenseListAccountResponse> {
        request.validate()?;
        let response: OpenWorkLicenseListAccountResponse = self
            .inner
            .post(
                "cgi-bin/license/list_order_account",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn cancel_license_order(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Result<OpenWorkStatusResponse> {
        let corp_id = corp_id.into();
        let order_id = order_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_identifier("order id", &order_id)?;
        let response: OpenWorkStatusResponse = self
            .inner
            .post(
                "cgi-bin/license/cancel_order",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "order_id": order_id }),
            )
            .await?;
        response.validate_for("open-work license cancel order")?;
        Ok(response)
    }

    pub async fn activate_license_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseActiveInfo,
    ) -> Result<OpenWorkStatusResponse> {
        request.validate_activation()?;
        let response: OpenWorkStatusResponse = self
            .inner
            .post(
                "cgi-bin/license/active_account",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate_for("open-work license activate account")?;
        Ok(response)
    }

    pub async fn batch_activate_license_account(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_list: Vec<OpenWorkLicenseActiveInfo>,
    ) -> Result<OpenWorkStatusResponse> {
        let corp_id = corp_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_active_batch(&active_list)?;
        let response: OpenWorkStatusResponse = self
            .inner
            .post(
                "cgi-bin/license/batch_active_account",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "active_list": active_list }),
            )
            .await?;
        response.validate_for("open-work license batch activate accounts")?;
        Ok(response)
    }

    pub async fn get_license_active_info_by_code(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_code: impl Into<String>,
    ) -> Result<OpenWorkLicenseActiveInfoResponse> {
        let corp_id = corp_id.into();
        let active_code = active_code.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_identifier("activation code", &active_code)?;
        let response: OpenWorkLicenseActiveInfoResponse = self
            .inner
            .post(
                "cgi-bin/license/get_active_info_by_code",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "active_code": active_code }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn batch_get_license_active_info_by_code(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        active_code_list: Vec<String>,
    ) -> Result<OpenWorkLicenseActiveInfoListResponse> {
        let corp_id = corp_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_identifier_batch("activation code", &active_code_list, 1_000)?;
        let response: OpenWorkLicenseActiveInfoListResponse = self
            .inner
            .post(
                "cgi-bin/license/batch_get_active_info_by_code",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "active_code_list": active_code_list }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn list_license_activated_account(
        &self,
        provider_access_token: impl Into<String>,
        request: OpenWorkLicenseListActivatedAccountRequest,
    ) -> Result<OpenWorkLicenseListAccountResponse> {
        request.validate()?;
        let response: OpenWorkLicenseListAccountResponse = self
            .inner
            .post(
                "cgi-bin/license/list_activated_account",
                Some(provider_access_token.into()),
                request,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_license_active_info_by_user(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseUserActiveInfoResponse> {
        let corp_id = corp_id.into();
        let user_id = user_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_identifier("member userid", &user_id)?;
        let response: OpenWorkLicenseUserActiveInfoResponse = self
            .inner
            .post(
                "cgi-bin/license/get_active_info_by_user",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "userid": user_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn batch_transfer_license(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        transfer_list: Vec<OpenWorkLicenseTransferInfo>,
    ) -> Result<OpenWorkLicenseTransferResponse> {
        let corp_id = corp_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_transfer_batch(&transfer_list)?;
        let response: OpenWorkLicenseTransferResponse = self
            .inner
            .post(
                "cgi-bin/license/batch_transfer_license",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "transfer_list": transfer_list }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn get_app_license_info(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        suite_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseInfoResponse> {
        let corp_id = corp_id.into();
        let suite_id = suite_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_identifier("suite id", &suite_id)?;
        let response: OpenWorkLicenseInfoResponse = self
            .inner
            .post(
                "cgi-bin/license/get_app_license_info",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "suite_id": suite_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn set_license_auto_active_status(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
        auto_active_status: i64,
    ) -> Result<OpenWorkStatusResponse> {
        let corp_id = corp_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        validate_open_work_license_binary_state("auto-active status", auto_active_status)?;
        let response: OpenWorkStatusResponse = self
            .inner
            .post(
                "cgi-bin/license/set_auto_active_status",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id, "auto_active_status": auto_active_status }),
            )
            .await?;
        response.validate_for("open-work license set auto-active status")?;
        Ok(response)
    }

    pub async fn get_license_auto_active_status(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
    ) -> Result<OpenWorkLicenseAutoActiveStatusResponse> {
        let corp_id = corp_id.into();
        validate_open_work_license_identifier("corporation id", &corp_id)?;
        let response: OpenWorkLicenseAutoActiveStatusResponse = self
            .inner
            .post(
                "cgi-bin/license/get_auto_active_status",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id }),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.server")
    }

    pub fn parse_server_event_xml(xml: &str) -> Result<OpenWorkServerEvent> {
        let event = OpenWorkServerEvent::from_callback_message(CallbackMessage::parse_xml(xml)?);
        event.validate()?;
        Ok(event)
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

pub type OpenWorkComponentAuthorizationInfo =
    crate::modules::open_platform::OpenPlatformAuthorizationInfo;
pub type OpenWorkComponentAuthorizerInfo =
    crate::modules::open_platform::OpenPlatformAuthorizerInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkComponentQueryAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub authorization_info: Option<OpenWorkComponentAuthorizationInfo>,
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
    pub authorizer_info: Option<OpenWorkComponentAuthorizerInfo>,
    #[serde(default)]
    pub authorization_info: Option<OpenWorkComponentAuthorizationInfo>,
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
        state: Option<String>,
        create_time: Option<i64>,
    },
    ChangeAuth {
        suite_id: Option<String>,
        auth_corp_id: Option<String>,
        state: Option<String>,
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
        auth_code: Option<String>,
        create_time: Option<i64>,
    },
    ChangeContactUser {
        message: Box<CallbackMessage>,
    },
    ChangeContactParty {
        message: Box<CallbackMessage>,
    },
    ChangeContactTag {
        message: Box<CallbackMessage>,
    },
    ShareAgentChange {
        message: Box<CallbackMessage>,
    },
    ShareChainChange {
        message: Box<CallbackMessage>,
    },
    CorpArchAuth {
        message: Box<CallbackMessage>,
    },
    ApproveSpecialAuth {
        message: Box<CallbackMessage>,
    },
    CancelSpecialAuth {
        message: Box<CallbackMessage>,
    },
    ChangeAppAdmin {
        message: Box<CallbackMessage>,
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
    ChangeContactUser,
    ChangeContactParty,
    ChangeContactTag,
    ShareAgentChange,
    ShareChainChange,
    CorpArchAuth,
    ApproveSpecialAuth,
    CancelSpecialAuth,
    ChangeAppAdmin,
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
            Self::ChangeContactUser | Self::ChangeContactParty | Self::ChangeContactTag => {
                Some("change_contact")
            }
            Self::ShareAgentChange => Some("share_agent_change"),
            Self::ShareChainChange => Some("share_chain_change"),
            Self::CorpArchAuth => Some("corp_arch_auth"),
            Self::ApproveSpecialAuth => Some("approve_special_auth"),
            Self::CancelSpecialAuth => Some("cancel_special_auth"),
            Self::ChangeAppAdmin => Some("change_app_admin"),
            Self::Unknown => None,
        }
    }

    pub fn is_auth_lifecycle(self) -> bool {
        matches!(self, Self::CreateAuth | Self::ChangeAuth | Self::CancelAuth)
    }

    pub fn is_ticket_refresh(self) -> bool {
        matches!(self, Self::SuiteTicket | Self::ResetPermanentCode)
    }

    pub fn is_contact_change(self) -> bool {
        matches!(
            self,
            Self::ChangeContactUser | Self::ChangeContactParty | Self::ChangeContactTag
        )
    }

    pub fn is_share_change(self) -> bool {
        matches!(self, Self::ShareAgentChange | Self::ShareChainChange)
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
                state: message.state.clone(),
                create_time: message.create_time,
            },
            Some("change_auth") => Self::ChangeAuth {
                suite_id: message.suite_id.clone(),
                auth_corp_id: message.auth_corp_id.clone(),
                state: message.state.clone(),
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
                auth_code: message.authorization_code.clone(),
                create_time: message.create_time,
            },
            Some("change_contact") => match message.change_type.as_deref() {
                Some("create_user" | "update_user" | "delete_user") => Self::ChangeContactUser {
                    message: Box::new(message),
                },
                Some("create_party" | "update_party" | "delete_party") => {
                    Self::ChangeContactParty {
                        message: Box::new(message),
                    }
                }
                Some("update_tag") => Self::ChangeContactTag {
                    message: Box::new(message),
                },
                _ => Self::Unknown {
                    info_type: message.info_type.clone(),
                    message: Box::new(message),
                },
            },
            Some("share_agent_change") => Self::ShareAgentChange {
                message: Box::new(message),
            },
            Some("share_chain_change") => Self::ShareChainChange {
                message: Box::new(message),
            },
            Some("corp_arch_auth") => Self::CorpArchAuth {
                message: Box::new(message),
            },
            Some("approve_special_auth") => Self::ApproveSpecialAuth {
                message: Box::new(message),
            },
            Some("cancel_special_auth") => Self::CancelSpecialAuth {
                message: Box::new(message),
            },
            None if message.event.as_deref() == Some("change_app_admin") => Self::ChangeAppAdmin {
                message: Box::new(message),
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
            Self::ChangeContactUser { .. } => OpenWorkServerEventKind::ChangeContactUser,
            Self::ChangeContactParty { .. } => OpenWorkServerEventKind::ChangeContactParty,
            Self::ChangeContactTag { .. } => OpenWorkServerEventKind::ChangeContactTag,
            Self::ShareAgentChange { .. } => OpenWorkServerEventKind::ShareAgentChange,
            Self::ShareChainChange { .. } => OpenWorkServerEventKind::ShareChainChange,
            Self::CorpArchAuth { .. } => OpenWorkServerEventKind::CorpArchAuth,
            Self::ApproveSpecialAuth { .. } => OpenWorkServerEventKind::ApproveSpecialAuth,
            Self::CancelSpecialAuth { .. } => OpenWorkServerEventKind::CancelSpecialAuth,
            Self::ChangeAppAdmin { .. } => OpenWorkServerEventKind::ChangeAppAdmin,
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
            Self::ChangeContactUser { .. }
            | Self::ChangeContactParty { .. }
            | Self::ChangeContactTag { .. } => Some("change_contact"),
            Self::ShareAgentChange { .. } => Some("share_agent_change"),
            Self::ShareChainChange { .. } => Some("share_chain_change"),
            Self::CorpArchAuth { .. } => Some("corp_arch_auth"),
            Self::ApproveSpecialAuth { .. } => Some("approve_special_auth"),
            Self::CancelSpecialAuth { .. } => Some("cancel_special_auth"),
            Self::ChangeAppAdmin { .. } => Some("change_app_admin"),
            Self::Unknown { info_type, .. } => info_type.as_deref(),
        }
    }

    pub fn is_auth_lifecycle(&self) -> bool {
        self.kind().is_auth_lifecycle()
    }

    pub fn is_ticket_refresh(&self) -> bool {
        self.kind().is_ticket_refresh()
    }

    pub fn is_contact_change(&self) -> bool {
        self.kind().is_contact_change()
    }

    pub fn is_share_change(&self) -> bool {
        self.kind().is_share_change()
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::SuiteTicket {
                suite_id,
                suite_ticket,
                create_time,
            } => {
                validate_open_work_server_identifier("suite id", suite_id.as_deref())?;
                validate_open_work_server_identifier("suite ticket", suite_ticket.as_deref())?;
                validate_open_work_server_timestamp(*create_time)
            }
            Self::CreateAuth {
                suite_id,
                auth_code,
                create_time,
                ..
            } => {
                validate_open_work_server_identifier("suite id", suite_id.as_deref())?;
                validate_open_work_server_identifier("authorization code", auth_code.as_deref())?;
                validate_open_work_server_timestamp(*create_time)
            }
            Self::ChangeAuth {
                suite_id,
                auth_corp_id,
                create_time,
                ..
            }
            | Self::CancelAuth {
                suite_id,
                auth_corp_id,
                create_time,
            } => {
                validate_open_work_server_identifier("suite id", suite_id.as_deref())?;
                validate_open_work_server_identifier(
                    "authorized corporation id",
                    auth_corp_id.as_deref(),
                )?;
                validate_open_work_server_timestamp(*create_time)
            }
            Self::ResetPermanentCode {
                suite_id,
                auth_corp_id,
                auth_code,
                create_time,
            } => {
                validate_open_work_server_identifier("suite id", suite_id.as_deref())?;
                validate_open_work_server_identifier(
                    "authorized corporation id",
                    auth_corp_id.as_deref(),
                )?;
                validate_open_work_server_identifier("authorization code", auth_code.as_deref())?;
                validate_open_work_server_timestamp(*create_time)
            }
            Self::ChangeContactUser { message } => {
                validate_open_work_server_message(message, true)?;
                validate_open_work_server_identifier("user id", message.user_id.as_deref())
            }
            Self::ChangeContactParty { message } => {
                validate_open_work_server_message(message, true)?;
                validate_open_work_server_positive_number("party id", message.id)
            }
            Self::ChangeContactTag { message } => {
                validate_open_work_server_message(message, true)?;
                validate_open_work_server_positive_number("tag id", message.tag_id)
            }
            Self::ShareAgentChange { message } => {
                validate_open_work_server_message(message, false)?;
                validate_open_work_server_identifier("application id", message.app_id.as_deref())?;
                validate_open_work_server_identifier("corporation id", message.corp_id.as_deref())?;
                validate_open_work_server_positive_number("agent id", message.agent_id)
            }
            Self::ShareChainChange { message } => {
                validate_open_work_server_message(message, false)?;
                validate_open_work_server_identifier("corporation id", message.corp_id.as_deref())
            }
            Self::CorpArchAuth { message } => {
                validate_open_work_server_message(message, false)?;
                validate_open_work_server_identifier(
                    "authorized corporation id",
                    message.auth_corp_id.as_deref(),
                )
            }
            Self::ApproveSpecialAuth { message } | Self::CancelSpecialAuth { message } => {
                validate_open_work_server_message(message, false)?;
                validate_open_work_server_identifier(
                    "authorized corporation id",
                    message.auth_corp_id.as_deref(),
                )?;
                validate_open_work_server_identifier(
                    "authorization type",
                    message.auth_type.as_deref(),
                )
            }
            Self::ChangeAppAdmin { message } => {
                validate_open_work_server_identifier(
                    "authorized corporation id",
                    message.auth_corp_id.as_deref(),
                )?;
                validate_open_work_server_positive_number("agent id", message.agent_id)
            }
            Self::Unknown { .. } => Ok(()),
        }
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

impl SuiteTokenRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_auth_identifier("suite id", &self.suite_id)?;
        validate_open_work_auth_identifier("suite secret", &self.suite_secret)?;
        validate_open_work_auth_identifier("suite ticket", &self.suite_ticket)
    }
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

impl SuiteTokenResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work get suite access token",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_auth_token_response(
            "suite access token",
            self.suite_access_token.as_deref(),
            self.expires_in,
        )
    }
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

impl OpenWorkPreAuthCodeResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work get pre-authorization code",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_auth_identifier(
            "pre-authorization code",
            self.pre_auth_code.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_auth_expiry("pre-authorization code", self.expires_in)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkSetSessionInfoRequest {
    pub pre_auth_code: String,
    pub session_info: OpenWorkSessionInfo,
}

impl OpenWorkSetSessionInfoRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_auth_identifier("pre-authorization code", &self.pre_auth_code)?;
        self.session_info.validate()
    }
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

    pub fn validate(&self) -> Result<()> {
        if self.appid.len() > 1_000 {
            return Err(WechatError::Config(
                "open-work session appid list cannot exceed 1000 values".to_string(),
            ));
        }
        let mut appids = HashSet::with_capacity(self.appid.len());
        if self
            .appid
            .iter()
            .any(|appid| *appid == 0 || !appids.insert(*appid))
        {
            return Err(WechatError::Config(
                "open-work session appid values must be positive and unique".to_string(),
            ));
        }
        if self
            .auth_type
            .is_some_and(|auth_type| !matches!(auth_type, 0 | 1))
        {
            return Err(WechatError::Config(
                "open-work session auth_type must be 0 or 1".to_string(),
            ));
        }
        Ok(())
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
    #[serde(default)]
    pub location: Option<String>,
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
    pub is_customized_app: Option<bool>,
    #[serde(default)]
    pub privilege: Option<OpenWorkAuthPrivilege>,
    #[serde(default)]
    pub shared_from: Option<OpenWorkAuthSharedFrom>,
    #[serde(default)]
    pub edition_id: Option<String>,
    #[serde(default)]
    pub edition_name: Option<String>,
    #[serde(default)]
    pub app_status: Option<i64>,
    #[serde(default)]
    pub user_limit: Option<i64>,
    #[serde(default)]
    pub expired_time: Option<i64>,
    #[serde(default)]
    pub is_virtual_version: Option<bool>,
    #[serde(default)]
    pub is_shared_from_other_corp: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthPrivilege {
    #[serde(default)]
    pub allow_party: Vec<i64>,
    #[serde(default)]
    pub allow_tag: Vec<i64>,
    #[serde(default)]
    pub allow_user: Vec<String>,
    #[serde(default)]
    pub extra_party: Vec<i64>,
    #[serde(default)]
    pub extra_user: Vec<String>,
    #[serde(default)]
    pub extra_tag: Vec<i64>,
    #[serde(default)]
    pub level: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkAuthSharedFrom {
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkAuthPrivilegeLevelKind {
    BasicReadOnly,
    FullReadOnly,
    FullReadWrite,
    SingleBasicReadOnly,
    FullWriteOnly,
    Other,
}

impl OpenWorkAuthPrivilegeLevelKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::BasicReadOnly,
            2 => Self::FullReadOnly,
            3 => Self::FullReadWrite,
            4 => Self::SingleBasicReadOnly,
            5 => Self::FullWriteOnly,
            _ => Self::Other,
        }
    }

    pub fn can_write(self) -> bool {
        matches!(self, Self::FullReadWrite | Self::FullWriteOnly)
    }
}

impl OpenWorkAuthPrivilege {
    pub fn level_kind(&self) -> Option<OpenWorkAuthPrivilegeLevelKind> {
        self.level.map(OpenWorkAuthPrivilegeLevelKind::from_code)
    }

    pub fn can_write_contacts(&self) -> bool {
        self.level_kind()
            .is_some_and(OpenWorkAuthPrivilegeLevelKind::can_write)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenWorkAuthAppStatusKind {
    Unpaid,
    LimitedTrial,
    TrialExpired,
    Purchased,
    PurchaseExpired,
    UnlimitedTrial,
    PurchasedOverLimitGrace,
    PurchasedOverLimitExpired,
    Other,
}

impl OpenWorkAuthAppStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Unpaid,
            1 => Self::LimitedTrial,
            2 => Self::TrialExpired,
            3 => Self::Purchased,
            4 => Self::PurchaseExpired,
            5 => Self::UnlimitedTrial,
            6 => Self::PurchasedOverLimitGrace,
            7 => Self::PurchasedOverLimitExpired,
            _ => Self::Other,
        }
    }

    pub fn has_active_entitlement(self) -> bool {
        matches!(
            self,
            Self::LimitedTrial
                | Self::Purchased
                | Self::UnlimitedTrial
                | Self::PurchasedOverLimitGrace
        )
    }

    pub fn is_paid(self) -> bool {
        matches!(
            self,
            Self::Purchased | Self::PurchasedOverLimitGrace | Self::PurchasedOverLimitExpired
        )
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

    pub fn app_status_kind(&self) -> Option<OpenWorkAuthAppStatusKind> {
        self.app_status.map(OpenWorkAuthAppStatusKind::from_code)
    }

    pub fn has_active_edition_entitlement(&self) -> bool {
        self.app_status_kind()
            .is_some_and(OpenWorkAuthAppStatusKind::has_active_entitlement)
    }

    pub fn is_shared_install(&self) -> bool {
        self.is_shared_from_other_corp == Some(true) || self.shared_from.is_some()
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
    pub edition_info: Option<OpenWorkAuthInfo>,
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

impl OpenWorkPermanentCodeResponse {
    pub fn validate_permanent_code(&self, operation: &str) -> Result<()> {
        ensure_open_work_response_success(operation, self.errcode, self.errmsg.as_deref())?;
        validate_open_work_auth_identifier(
            "permanent code",
            self.permanent_code.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_auth_corp_info(self.auth_corp_info.as_ref())?;
        validate_open_work_auth_info(self.auth_info.as_ref())
    }

    pub fn validate_auth_info(&self, operation: &str) -> Result<()> {
        ensure_open_work_response_success(operation, self.errcode, self.errmsg.as_deref())?;
        validate_open_work_auth_corp_info(self.auth_corp_info.as_ref())?;
        validate_open_work_auth_info(self.auth_info.as_ref())
    }
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

impl OpenWorkCorpTokenResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work get corporation access token",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_auth_token_response(
            "corporation access token",
            self.access_token.as_deref(),
            self.expires_in,
        )
    }
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

impl OpenWorkUserIdToOpenUserIdResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work convert user ids to open user ids",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.open_userid_list.len() > 1_000 {
            return Err(WechatError::Config(
                "open-work converted user id response cannot exceed 1000 values".to_string(),
            ));
        }
        let mut userids = HashSet::with_capacity(self.open_userid_list.len());
        let mut open_userids = HashSet::with_capacity(self.open_userid_list.len());
        for item in &self.open_userid_list {
            let userid = item.userid.as_deref().unwrap_or_default();
            let open_userid = item.open_userid.as_deref().unwrap_or_default();
            validate_open_work_auth_identifier("converted user id", userid)?;
            validate_open_work_auth_identifier("open user id", open_userid)?;
            if !userids.insert(userid) || !open_userids.insert(open_userid) {
                return Err(WechatError::Config(
                    "open-work converted user id response contains duplicates".to_string(),
                ));
            }
        }
        Ok(())
    }
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

impl OpenWorkStatusResponse {
    pub fn validate_for(&self, operation: &str) -> Result<()> {
        ensure_open_work_response_success(operation, self.errcode, self.errmsg.as_deref())
    }
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

impl OpenWorkLicenseAccountCount {
    pub fn validate(&self) -> Result<()> {
        let base_count = self.base_count.unwrap_or(0);
        let external_contact_count = self.external_contact_count.unwrap_or(0);
        if !(0..=1_000_000).contains(&base_count)
            || !(0..=1_000_000).contains(&external_contact_count)
        {
            return Err(WechatError::Config(
                "open-work license account counts must be between 0 and 1000000".to_string(),
            ));
        }
        if base_count == 0 && external_contact_count == 0 {
            return Err(WechatError::Config(
                "open-work license order requires at least one account".to_string(),
            ));
        }
        Ok(())
    }
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

impl OpenWorkLicenseAccountDuration {
    fn validate_purchase(&self) -> Result<()> {
        if self.new_expire_time.is_some() {
            return Err(WechatError::Config(
                "open-work new license order must use month/days instead of new_expire_time"
                    .to_string(),
            ));
        }
        self.validate_relative_days(31)
    }

    fn validate_renewal(&self) -> Result<()> {
        if let Some(new_expire_time) = self.new_expire_time {
            if new_expire_time <= 0 || self.month.unwrap_or(0) != 0 || self.days.unwrap_or(0) != 0 {
                return Err(WechatError::Config(
                    "open-work license renewal new_expire_time must be positive and exclusive"
                        .to_string(),
                ));
            }
            return Ok(());
        }
        self.validate_relative_days(1)
    }

    fn validate_relative_days(&self, minimum_days: i64) -> Result<()> {
        let month = self.month.unwrap_or(0);
        let days = self.days.unwrap_or(0);
        if month < 0 || days < 0 {
            return Err(WechatError::Config(
                "open-work license duration month and days cannot be negative".to_string(),
            ));
        }
        let total_days = month
            .checked_mul(31)
            .and_then(|total| total.checked_add(days))
            .ok_or_else(|| {
                WechatError::Config("open-work license duration overflowed".to_string())
            })?;
        if !(minimum_days..=1_860).contains(&total_days) {
            return Err(WechatError::Config(format!(
                "open-work license duration must be between {minimum_days} and 1860 days"
            )));
        }
        Ok(())
    }

    fn validate_response(&self) -> Result<()> {
        if self.month.is_some_and(|month| month < 0)
            || self.days.is_some_and(|days| days < 0)
            || self
                .new_expire_time
                .is_some_and(|new_expire_time| new_expire_time < 0)
        {
            return Err(WechatError::Config(
                "open-work license response duration values cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
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

impl OpenWorkLicenseCreateNewOrderRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "corporation id",
            self.corpid.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_license_identifier(
            "buyer userid",
            self.buyer_userid.as_deref().unwrap_or_default(),
        )?;
        self.account_count
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open-work new license order requires account_count".to_string(),
                )
            })?
            .validate()?;
        self.account_duration
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open-work new license order requires account_duration".to_string(),
                )
            })?
            .validate_purchase()
    }
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

impl OpenWorkLicenseCreateRenewOrderJobRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "corporation id",
            self.corpid.as_deref().unwrap_or_default(),
        )?;
        if let Some(jobid) = &self.jobid {
            validate_open_work_license_identifier("renewal job id", jobid)?;
        }
        if self.account_list.is_empty() || self.account_list.len() > 1_000 {
            return Err(WechatError::Config(
                "open-work license renewal job requires between 1 and 1000 accounts".to_string(),
            ));
        }
        let mut active_codes = HashSet::with_capacity(self.account_list.len());
        for account in &self.account_list {
            account.validate_renewal_item()?;
            let active_code = account
                .active_code
                .as_deref()
                .expect("renewal item validation requires active_code");
            if !active_codes.insert(active_code.trim()) {
                return Err(WechatError::Config(
                    "open-work license renewal job contains duplicate activation codes".to_string(),
                ));
            }
        }
        Ok(())
    }
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

impl OpenWorkLicenseSubmitOrderJobRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "renewal job id",
            self.jobid.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_license_identifier(
            "buyer userid",
            self.buyer_userid.as_deref().unwrap_or_default(),
        )?;
        self.account_duration
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open-work submit renewal order requires account_duration".to_string(),
                )
            })?
            .validate_renewal()
    }
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

impl OpenWorkLicenseListOrderRequest {
    pub fn validate(&self) -> Result<()> {
        if let Some(corpid) = &self.corpid {
            validate_open_work_license_identifier("corporation id", corpid)?;
        }
        match (self.start_time.as_deref(), self.end_time.as_deref()) {
            (None, None) => {}
            (Some(start_time), Some(end_time)) => {
                let start_time = validate_open_work_license_timestamp("start time", start_time)?;
                let end_time = validate_open_work_license_timestamp("end time", end_time)?;
                if end_time < start_time || end_time - start_time > 31 * 86_400 {
                    return Err(WechatError::Config(
                        "open-work license order time range must be ordered and at most 31 days"
                            .to_string(),
                    ));
                }
            }
            _ => {
                return Err(WechatError::Config(
                    "open-work license order start_time and end_time must be provided together"
                        .to_string(),
                ));
            }
        }
        validate_open_work_license_page(self.cursor.as_deref(), self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListOrderAccountRequest {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl OpenWorkLicenseListOrderAccountRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier("order id", &self.order_id)?;
        validate_open_work_license_page(self.cursor.as_deref(), self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkLicenseListActivatedAccountRequest {
    pub corpid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl OpenWorkLicenseListActivatedAccountRequest {
    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier("corporation id", &self.corpid)?;
        validate_open_work_license_page(self.cursor.as_deref(), self.limit)
    }
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

    pub fn validate_activation(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "corporation id",
            self.corpid.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_license_identifier(
            "member userid",
            self.userid.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_license_identifier(
            "activation code",
            self.active_code.as_deref().unwrap_or_default(),
        )
    }

    fn validate_renewal_item(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "renewal activation code",
            self.active_code.as_deref().unwrap_or_default(),
        )?;
        if let Some(userid) = &self.userid {
            validate_open_work_license_identifier("renewal member userid", userid)?;
        }
        Ok(())
    }

    pub fn validate_response(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "activation response code",
            self.active_code.as_deref().unwrap_or_default(),
        )?;
        validate_open_work_license_identifier(
            "activation response corporation id",
            self.corpid.as_deref().unwrap_or_default(),
        )?;
        let account_type = self.account_type.ok_or_else(|| {
            WechatError::Config(
                "open-work license activation response requires account type".to_string(),
            )
        })?;
        if !matches!(account_type, 1 | 2) {
            return Err(WechatError::Config(
                "open-work license activation response account type must be 1 or 2".to_string(),
            ));
        }
        let status = self.status.ok_or_else(|| {
            WechatError::Config("open-work license activation response requires status".to_string())
        })?;
        if !(1..=6).contains(&status) {
            return Err(WechatError::Config(
                "open-work license activation response status must be between 1 and 6".to_string(),
            ));
        }
        if status != 1 {
            validate_open_work_license_identifier(
                "activation response member userid",
                self.userid.as_deref().unwrap_or_default(),
            )?;
        } else if let Some(userid) = &self.userid {
            validate_open_work_license_identifier("activation response member userid", userid)?;
        }
        validate_open_work_license_timestamps(
            self.create_time,
            self.active_time,
            self.expire_time,
        )?;
        if status == 5 {
            let merge_info = self.merge_info.as_ref().ok_or_else(|| {
                WechatError::Config(
                    "open-work merged license activation requires merge_info".to_string(),
                )
            })?;
            if merge_info.to_active_code.is_none() && merge_info.from_active_code.is_none() {
                return Err(WechatError::Config(
                    "open-work merged license activation requires a merge activation code"
                        .to_string(),
                ));
            }
        }
        if let Some(merge_info) = &self.merge_info {
            if let Some(code) = &merge_info.to_active_code {
                validate_open_work_license_identifier("merged-to activation code", code)?;
            }
            if let Some(code) = &merge_info.from_active_code {
                validate_open_work_license_identifier("merged-from activation code", code)?;
            }
        }
        if let Some(share_info) = &self.share_info {
            if share_info.to_corpid.is_none() && share_info.from_corpid.is_none() {
                return Err(WechatError::Config(
                    "open-work shared license activation requires a corporation id".to_string(),
                ));
            }
            if let Some(corpid) = &share_info.to_corpid {
                validate_open_work_license_identifier("shared-to corporation id", corpid)?;
            }
            if let Some(corpid) = &share_info.from_corpid {
                validate_open_work_license_identifier("shared-from corporation id", corpid)?;
            }
        }
        Ok(())
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

impl OpenWorkLicenseTransferInfo {
    fn validate_request(&self) -> Result<()> {
        let handover_userid = self.handover_userid.as_deref().unwrap_or_default();
        let takeover_userid = self.takeover_userid.as_deref().unwrap_or_default();
        validate_open_work_license_identifier("handover userid", handover_userid)?;
        validate_open_work_license_identifier("takeover userid", takeover_userid)?;
        if handover_userid == takeover_userid {
            return Err(WechatError::Config(
                "open-work license handover and takeover users must be different".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_response(&self) -> Result<()> {
        self.validate_request()?;
        if self.errcode.is_none() {
            return Err(WechatError::Config(
                "open-work license transfer result requires errcode".to_string(),
            ));
        }
        Ok(())
    }
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

    pub fn validate(&self) -> Result<()> {
        validate_open_work_license_identifier(
            "order response id",
            self.order_id.as_deref().unwrap_or_default(),
        )?;
        if self.order_type.is_none_or(|order_type| order_type <= 0) {
            return Err(WechatError::Config(
                "open-work license order type must be positive".to_string(),
            ));
        }
        if self.order_status.is_none_or(|status| status < 0) {
            return Err(WechatError::Config(
                "open-work license order status cannot be negative".to_string(),
            ));
        }
        if let Some(corpid) = &self.corpid {
            validate_open_work_license_identifier("order corporation id", corpid)?;
        }
        if let Some(buyer_userid) = &self.buyer_userid {
            validate_open_work_license_identifier("order buyer userid", buyer_userid)?;
        }
        if self.price.is_some_and(|price| price < 0) {
            return Err(WechatError::Config(
                "open-work license order price cannot be negative".to_string(),
            ));
        }
        if let Some(account_count) = &self.account_count {
            account_count.validate()?;
        }
        if let Some(duration) = &self.account_duration {
            duration.validate_response()?;
        }
        validate_open_work_license_order_timestamps(self.create_time, self.pay_time)
    }
}

impl OpenWorkLicenseListOrderResponse {
    pub fn has_more(&self) -> bool {
        self.has_more == Some(1)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license list orders",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_response_page(
            self.has_more,
            self.next_cursor.as_deref(),
            self.order_list.len(),
        )?;
        let mut order_ids = HashSet::with_capacity(self.order_list.len());
        for order in &self.order_list {
            order.validate()?;
            let order_id = order
                .order_id
                .as_deref()
                .expect("order validation requires order_id");
            if !order_ids.insert(order_id) {
                return Err(WechatError::Config(
                    "open-work license order response contains duplicate order ids".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl OpenWorkLicenseListAccountResponse {
    pub fn has_more(&self) -> bool {
        self.has_more == Some(1)
    }

    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license list accounts",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_response_page(
            self.has_more,
            self.next_cursor.as_deref(),
            self.account_list.len(),
        )?;
        validate_open_work_license_active_response_list(&self.account_list)
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

impl OpenWorkLicenseTrialInfo {
    fn validate(&self) -> Result<()> {
        let start_time = self.start_time.ok_or_else(|| {
            WechatError::Config("open-work license trial requires start_time".to_string())
        })?;
        let end_time = self.end_time.ok_or_else(|| {
            WechatError::Config("open-work license trial requires end_time".to_string())
        })?;
        if start_time < 0 || end_time < start_time {
            return Err(WechatError::Config(
                "open-work license trial timestamps are inconsistent".to_string(),
            ));
        }
        if self.trial_status.is_some_and(|status| status < 0) {
            return Err(WechatError::Config(
                "open-work license trial status cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
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

impl OpenWorkLicenseOrderIdResponse {
    pub fn validate_for(&self, operation: &str) -> Result<()> {
        ensure_open_work_response_success(operation, self.errcode, self.errmsg.as_deref())?;
        validate_open_work_license_identifier(
            "order response id",
            self.order_id.as_deref().unwrap_or_default(),
        )
    }
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

impl OpenWorkLicenseCreateRenewOrderJobResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license create renewal job",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_identifier(
            "renewal response job id",
            self.jobid.as_deref().unwrap_or_default(),
        )?;
        let mut identities = HashSet::with_capacity(self.invalid_account_list.len());
        for account in &self.invalid_account_list {
            let identity = account
                .active_code
                .as_deref()
                .or(account.userid.as_deref())
                .unwrap_or_default();
            validate_open_work_license_identifier("invalid renewal account identity", identity)?;
            if !identities.insert(identity) {
                return Err(WechatError::Config(
                    "open-work license renewal response contains duplicate invalid accounts"
                        .to_string(),
                ));
            }
            if account.errcode.is_none_or(|errcode| errcode == 0) {
                return Err(WechatError::Config(
                    "open-work invalid renewal account requires nonzero errcode".to_string(),
                ));
            }
        }
        Ok(())
    }
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

impl OpenWorkLicenseOrderResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license get order",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        self.order
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config("open-work license order response requires order".to_string())
            })?
            .validate()
    }
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

impl OpenWorkLicenseActiveInfoResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license get activation info",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        self.active_info
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config(
                    "open-work license activation response requires active_info".to_string(),
                )
            })?
            .validate_response()
    }
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

impl OpenWorkLicenseActiveInfoListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license batch get activation info",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_active_response_list(&self.active_info_list)
    }
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

    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license get member activation info",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_binary_state(
            "member active status",
            self.active_status.ok_or_else(|| {
                WechatError::Config(
                    "open-work license member response requires active_status".to_string(),
                )
            })?,
        )?;
        validate_open_work_license_active_response_list(&self.active_info_list)?;
        if self.is_active() && self.active_info_list.is_empty() {
            return Err(WechatError::Config(
                "open-work active license member requires active_info_list".to_string(),
            ));
        }
        Ok(())
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

impl OpenWorkLicenseTransferResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work license batch transfer",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        if self.transfer_result.is_empty() || self.transfer_result.len() > 1_000 {
            return Err(WechatError::Config(
                "open-work license transfer response requires between 1 and 1000 results"
                    .to_string(),
            ));
        }
        let mut handover_users = HashSet::with_capacity(self.transfer_result.len());
        for transfer in &self.transfer_result {
            transfer.validate_response()?;
            let handover = transfer
                .handover_userid
                .as_deref()
                .expect("transfer validation requires handover userid");
            if !handover_users.insert(handover) {
                return Err(WechatError::Config(
                    "open-work license transfer response contains duplicate handover users"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
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

    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work get application license info",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_binary_state(
            "license status",
            self.license_status.ok_or_else(|| {
                WechatError::Config(
                    "open-work application license response requires license_status".to_string(),
                )
            })?,
        )?;
        if self
            .license_check_time
            .is_some_and(|timestamp| timestamp < 0)
        {
            return Err(WechatError::Config(
                "open-work license check time cannot be negative".to_string(),
            ));
        }
        if let Some(trial_info) = &self.trail_info {
            trial_info.validate()?;
        }
        Ok(())
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

    pub fn validate(&self) -> Result<()> {
        ensure_open_work_response_success(
            "open-work get license auto-active status",
            self.errcode,
            self.errmsg.as_deref(),
        )?;
        validate_open_work_license_binary_state(
            "auto-active status",
            self.auto_active_status.ok_or_else(|| {
                WechatError::Config(
                    "open-work license auto-active response requires auto_active_status"
                        .to_string(),
                )
            })?,
        )
    }
}

fn ensure_open_work_response_success(
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

fn validate_open_work_auth_identifier(kind: &str, value: &str) -> Result<()> {
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "open-work {kind} must contain 1 to 512 printable UTF-8 bytes"
        )));
    }
    Ok(())
}

fn validate_open_work_auth_identifier_batch(
    kind: &str,
    identifiers: &[String],
    maximum: usize,
) -> Result<()> {
    if identifiers.is_empty() || identifiers.len() > maximum {
        return Err(WechatError::Config(format!(
            "open-work {kind} list must contain between 1 and {maximum} values"
        )));
    }
    let mut unique = HashSet::with_capacity(identifiers.len());
    for identifier in identifiers {
        validate_open_work_auth_identifier(kind, identifier)?;
        if !unique.insert(identifier.trim()) {
            return Err(WechatError::Config(format!(
                "open-work {kind} list contains duplicates"
            )));
        }
    }
    Ok(())
}

fn validate_open_work_auth_credentials(
    suite_access_token: &str,
    auth_corpid: &str,
    permanent_code: &str,
) -> Result<()> {
    validate_open_work_auth_identifier("suite access token", suite_access_token)?;
    validate_open_work_auth_identifier("authorized corporation id", auth_corpid)?;
    validate_open_work_auth_identifier("permanent code", permanent_code)
}

fn validate_open_work_auth_expiry(kind: &str, expires_in: Option<i64>) -> Result<()> {
    if expires_in.is_none_or(|expires_in| expires_in <= 0) {
        return Err(WechatError::Config(format!(
            "open-work {kind} response requires a positive expires_in"
        )));
    }
    Ok(())
}

fn validate_open_work_auth_token_response(
    kind: &str,
    token: Option<&str>,
    expires_in: Option<i64>,
) -> Result<()> {
    validate_open_work_auth_identifier(kind, token.unwrap_or_default())?;
    validate_open_work_auth_expiry(kind, expires_in)
}

fn validate_open_work_auth_corp_info(corp: Option<&OpenWorkAuthCorpInfo>) -> Result<()> {
    let corp = corp.ok_or_else(|| {
        WechatError::Config("open-work authorization response requires auth_corp_info".to_string())
    })?;
    validate_open_work_auth_identifier(
        "authorized corporation id",
        corp.corpid.as_deref().unwrap_or_default(),
    )
}

fn validate_open_work_auth_info(auth_info: Option<&OpenWorkAuthInfo>) -> Result<()> {
    let Some(auth_info) = auth_info else {
        return Ok(());
    };
    let mut agent_ids = HashSet::with_capacity(auth_info.agent.len());
    for agent in &auth_info.agent {
        if agent.agentid.is_some_and(|agentid| agentid <= 0) {
            return Err(WechatError::Config(
                "open-work authorization agent id must be positive".to_string(),
            ));
        }
        if let Some(agentid) = agent.agentid {
            if !agent_ids.insert(agentid) {
                return Err(WechatError::Config(
                    "open-work authorization response contains duplicate agent ids".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_open_work_server_identifier(kind: &str, value: Option<&str>) -> Result<()> {
    validate_open_work_auth_identifier(&format!("server event {kind}"), value.unwrap_or_default())
}

fn validate_open_work_server_timestamp(create_time: Option<i64>) -> Result<()> {
    if create_time.is_none_or(|create_time| create_time <= 0) {
        return Err(WechatError::Config(
            "open-work server event requires a positive timestamp".to_string(),
        ));
    }
    Ok(())
}

fn validate_open_work_server_positive_number(kind: &str, value: Option<i64>) -> Result<()> {
    if value.is_none_or(|value| value <= 0) {
        return Err(WechatError::Config(format!(
            "open-work server event {kind} must be positive"
        )));
    }
    Ok(())
}

fn validate_open_work_server_message(message: &CallbackMessage, require_corp: bool) -> Result<()> {
    validate_open_work_server_identifier("suite id", message.suite_id.as_deref())?;
    validate_open_work_server_timestamp(message.create_time)?;
    if require_corp {
        validate_open_work_server_identifier(
            "authorized corporation id",
            message.auth_corp_id.as_deref(),
        )?;
    }
    Ok(())
}

fn validate_open_work_license_identifier(kind: &str, value: &str) -> Result<()> {
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "open-work license {kind} must contain 1 to 512 printable UTF-8 bytes"
        )));
    }
    Ok(())
}

fn validate_open_work_license_timestamp(kind: &str, value: &str) -> Result<i64> {
    let timestamp = value.parse::<i64>().map_err(|_| {
        WechatError::Config(format!(
            "open-work license {kind} must be a positive Unix timestamp"
        ))
    })?;
    if timestamp <= 0 {
        return Err(WechatError::Config(format!(
            "open-work license {kind} must be a positive Unix timestamp"
        )));
    }
    Ok(timestamp)
}

fn validate_open_work_license_page(cursor: Option<&str>, limit: Option<i64>) -> Result<()> {
    if cursor.is_some_and(|cursor| cursor.trim().is_empty()) {
        return Err(WechatError::Config(
            "open-work license pagination cursor cannot be blank".to_string(),
        ));
    }
    if limit.is_some_and(|limit| !(1..=1_000).contains(&limit)) {
        return Err(WechatError::Config(
            "open-work license pagination limit must be between 1 and 1000".to_string(),
        ));
    }
    Ok(())
}

fn validate_open_work_license_response_page(
    has_more: Option<i64>,
    next_cursor: Option<&str>,
    item_count: usize,
) -> Result<()> {
    let has_more = has_more.ok_or_else(|| {
        WechatError::Config("open-work license page response requires has_more".to_string())
    })?;
    validate_open_work_license_binary_state("page has_more", has_more)?;
    if item_count > 1_000 {
        return Err(WechatError::Config(
            "open-work license page response cannot exceed 1000 items".to_string(),
        ));
    }
    if has_more == 1 {
        validate_open_work_license_identifier("page next cursor", next_cursor.unwrap_or_default())?;
        if item_count == 0 {
            return Err(WechatError::Config(
                "open-work license page with has_more cannot be empty".to_string(),
            ));
        }
    } else if let Some(next_cursor) = next_cursor {
        validate_open_work_license_identifier("page next cursor", next_cursor)?;
    }
    Ok(())
}

fn validate_open_work_license_identifier_batch(
    kind: &str,
    identifiers: &[String],
    maximum: usize,
) -> Result<()> {
    if identifiers.is_empty() || identifiers.len() > maximum {
        return Err(WechatError::Config(format!(
            "open-work license {kind} list must contain between 1 and {maximum} values"
        )));
    }
    let mut unique = HashSet::with_capacity(identifiers.len());
    for identifier in identifiers {
        validate_open_work_license_identifier(kind, identifier)?;
        if !unique.insert(identifier.trim()) {
            return Err(WechatError::Config(format!(
                "open-work license {kind} list contains duplicates"
            )));
        }
    }
    Ok(())
}

fn validate_open_work_license_active_batch(
    active_list: &[OpenWorkLicenseActiveInfo],
) -> Result<()> {
    if active_list.is_empty() || active_list.len() > 1_000 {
        return Err(WechatError::Config(
            "open-work license activation batch must contain between 1 and 1000 accounts"
                .to_string(),
        ));
    }
    let mut active_codes = HashSet::with_capacity(active_list.len());
    let mut userids = HashSet::with_capacity(active_list.len());
    for active in active_list {
        active.validate_activation()?;
        let active_code = active
            .active_code
            .as_deref()
            .expect("activation validation requires active_code");
        let userid = active
            .userid
            .as_deref()
            .expect("activation validation requires userid");
        if !active_codes.insert(active_code.trim()) || !userids.insert(userid.trim()) {
            return Err(WechatError::Config(
                "open-work license activation batch contains duplicate codes or users".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_open_work_license_active_response_list(
    active_list: &[OpenWorkLicenseActiveInfo],
) -> Result<()> {
    if active_list.len() > 1_000 {
        return Err(WechatError::Config(
            "open-work license activation response cannot exceed 1000 accounts".to_string(),
        ));
    }
    let mut active_codes = HashSet::with_capacity(active_list.len());
    for active in active_list {
        active.validate_response()?;
        let active_code = active
            .active_code
            .as_deref()
            .expect("activation response validation requires active_code");
        if !active_codes.insert(active_code) {
            return Err(WechatError::Config(
                "open-work license activation response contains duplicate activation codes"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_open_work_license_transfer_batch(
    transfer_list: &[OpenWorkLicenseTransferInfo],
) -> Result<()> {
    if transfer_list.is_empty() || transfer_list.len() > 1_000 {
        return Err(WechatError::Config(
            "open-work license transfer batch must contain between 1 and 1000 transfers"
                .to_string(),
        ));
    }
    let mut handover_users = HashSet::with_capacity(transfer_list.len());
    for transfer in transfer_list {
        transfer.validate_request()?;
        let handover = transfer
            .handover_userid
            .as_deref()
            .expect("transfer validation requires handover userid");
        if !handover_users.insert(handover.trim()) {
            return Err(WechatError::Config(
                "open-work license transfer batch contains duplicate handover users".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_open_work_license_binary_state(kind: &str, value: i64) -> Result<()> {
    if !matches!(value, 0 | 1) {
        return Err(WechatError::Config(format!(
            "open-work license {kind} must be 0 or 1"
        )));
    }
    Ok(())
}

fn validate_open_work_license_timestamps(
    create_time: Option<i64>,
    active_time: Option<i64>,
    expire_time: Option<i64>,
) -> Result<()> {
    if [create_time, active_time, expire_time]
        .into_iter()
        .flatten()
        .any(|timestamp| timestamp < 0)
    {
        return Err(WechatError::Config(
            "open-work license activation timestamps cannot be negative".to_string(),
        ));
    }
    if create_time
        .zip(active_time)
        .is_some_and(|(created, activated)| activated < created)
        || active_time
            .zip(expire_time)
            .is_some_and(|(activated, expires)| expires < activated)
    {
        return Err(WechatError::Config(
            "open-work license activation timestamps are inconsistent".to_string(),
        ));
    }
    Ok(())
}

fn validate_open_work_license_order_timestamps(
    create_time: Option<i64>,
    pay_time: Option<i64>,
) -> Result<()> {
    if create_time.is_some_and(|timestamp| timestamp < 0)
        || pay_time.is_some_and(|timestamp| timestamp < 0)
        || create_time
            .zip(pay_time)
            .is_some_and(|(created, paid)| paid < created)
    {
        return Err(WechatError::Config(
            "open-work license order timestamps are inconsistent".to_string(),
        ));
    }
    Ok(())
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
                <TimeStamp>1800000001</TimeStamp>
                <AuthCode><![CDATA[auth-code]]></AuthCode>
                <State><![CDATA[install-state]]></State>
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
                state,
                create_time,
            } => {
                assert_eq!(suite_id.as_deref(), Some("suite-id"));
                assert_eq!(auth_code.as_deref(), Some("auth-code"));
                assert_eq!(state.as_deref(), Some("install-state"));
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
    fn parses_open_work_contact_and_share_events() {
        let user = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[change_contact]]></InfoType>
                <TimeStamp>1800000100</TimeStamp>
                <ChangeType><![CDATA[update_user]]></ChangeType>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                <UserID><![CDATA[user-id]]></UserID>
                <OpenUserID><![CDATA[open-user-id]]></OpenUserID>
                <Name><![CDATA[User Name]]></Name>
                <Department><![CDATA[1,2]]></Department>
                <MainDepartment>2</MainDepartment>
                <Mobile><![CDATA[13800000000]]></Mobile>
                <BizMail><![CDATA[user@example.com]]></BizMail>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(user.kind(), OpenWorkServerEventKind::ChangeContactUser);
        assert!(user.is_contact_change());
        assert_eq!(user.info_type(), Some("change_contact"));
        match user {
            OpenWorkServerEvent::ChangeContactUser { message } => {
                assert_eq!(message.change_type.as_deref(), Some("update_user"));
                assert_eq!(message.auth_corp_id.as_deref(), Some("corp-id"));
                assert_eq!(message.user_id.as_deref(), Some("user-id"));
                assert_eq!(message.open_user_id.as_deref(), Some("open-user-id"));
                assert_eq!(message.name.as_deref(), Some("User Name"));
                assert_eq!(message.department.as_deref(), Some("1,2"));
                assert_eq!(message.main_department, Some(2));
                assert_eq!(message.mobile.as_deref(), Some("13800000000"));
                assert_eq!(message.biz_mail.as_deref(), Some("user@example.com"));
                assert_eq!(message.create_time, Some(1_800_000_100));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let party = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[change_contact]]></InfoType>
                <TimeStamp>1800000101</TimeStamp>
                <ChangeType><![CDATA[create_party]]></ChangeType>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                <Id>10</Id>
                <Name><![CDATA[Engineering]]></Name>
                <ParentId>1</ParentId>
                <OrderId>20</OrderId>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(party.kind(), OpenWorkServerEventKind::ChangeContactParty);
        match party {
            OpenWorkServerEvent::ChangeContactParty { message } => {
                assert_eq!(message.id, Some(10));
                assert_eq!(message.parent_id, Some(1));
                assert_eq!(message.order_id, Some(20));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let tag = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[change_contact]]></InfoType>
                <TimeStamp>1800000102</TimeStamp>
                <ChangeType><![CDATA[update_tag]]></ChangeType>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                <TagId>30</TagId>
                <AddUserItems><![CDATA[user-a,user-b]]></AddUserItems>
                <DelPartyItems><![CDATA[4,5]]></DelPartyItems>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(tag.kind(), OpenWorkServerEventKind::ChangeContactTag);
        match tag {
            OpenWorkServerEvent::ChangeContactTag { message } => {
                assert_eq!(message.tag_id, Some(30));
                assert_eq!(message.add_user_items.as_deref(), Some("user-a,user-b"));
                assert_eq!(message.del_party_items.as_deref(), Some("4,5"));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let share_agent = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[share_agent_change]]></InfoType>
                <TimeStamp>1800000103</TimeStamp>
                <AppId><![CDATA[100001]]></AppId>
                <CorpId><![CDATA[downstream-corp]]></CorpId>
                <AgentID>200001</AgentID>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(
            share_agent.kind(),
            OpenWorkServerEventKind::ShareAgentChange
        );
        assert!(share_agent.is_share_change());
        match share_agent {
            OpenWorkServerEvent::ShareAgentChange { message } => {
                assert_eq!(message.app_id.as_deref(), Some("100001"));
                assert_eq!(message.corp_id.as_deref(), Some("downstream-corp"));
                assert_eq!(message.agent_id, Some(200001));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let share_chain = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[share_chain_change]]></InfoType>
                <TimeStamp>1800000104</TimeStamp>
                <CorpId><![CDATA[downstream-corp]]></CorpId>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(
            share_chain.kind(),
            OpenWorkServerEventKind::ShareChainChange
        );
        assert!(OpenWorkServerEventKind::ShareChainChange.is_share_change());
    }

    #[test]
    fn parses_open_work_arch_special_and_admin_events() {
        let corp_arch = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[corp_arch_auth]]></InfoType>
                <TimeStamp>1800000200</TimeStamp>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(corp_arch.kind(), OpenWorkServerEventKind::CorpArchAuth);

        let approved = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[approve_special_auth]]></InfoType>
                <TimeStamp>1800000201</TimeStamp>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                <AuthType><![CDATA[customer_contact]]></AuthType>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(approved.kind(), OpenWorkServerEventKind::ApproveSpecialAuth);
        match approved {
            OpenWorkServerEvent::ApproveSpecialAuth { message } => {
                assert_eq!(message.auth_corp_id.as_deref(), Some("corp-id"));
                assert_eq!(message.auth_type.as_deref(), Some("customer_contact"));
            }
            other => panic!("unexpected event: {other:?}"),
        }

        let canceled = OpenWork::parse_server_event_xml(
            r#"<xml>
                <SuiteId><![CDATA[suite-id]]></SuiteId>
                <InfoType><![CDATA[cancel_special_auth]]></InfoType>
                <TimeStamp>1800000202</TimeStamp>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                <AuthType><![CDATA[customer_contact]]></AuthType>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(canceled.kind(), OpenWorkServerEventKind::CancelSpecialAuth);

        let admin = OpenWork::parse_server_event_xml(
            r#"<xml>
                <Event><![CDATA[change_app_admin]]></Event>
                <AgentID>100001</AgentID>
                <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(admin.kind(), OpenWorkServerEventKind::ChangeAppAdmin);
        assert_eq!(admin.info_type(), Some("change_app_admin"));
        match admin {
            OpenWorkServerEvent::ChangeAppAdmin { message } => {
                assert_eq!(message.agent_id, Some(100001));
                assert_eq!(message.auth_corp_id.as_deref(), Some("corp-id"));
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
    fn validates_open_work_suite_auth_request_matrix() {
        assert!(SuiteTokenRequest {
            suite_id: "suite".to_string(),
            suite_secret: "secret".to_string(),
            suite_ticket: "ticket".to_string(),
        }
        .validate()
        .is_ok());
        assert!(SuiteTokenRequest {
            suite_id: " ".to_string(),
            suite_secret: "secret".to_string(),
            suite_ticket: "ticket".to_string(),
        }
        .validate()
        .is_err());

        assert!(OpenWorkSetSessionInfoRequest {
            pre_auth_code: "pre-auth".to_string(),
            session_info: OpenWorkSessionInfo {
                appid: vec![1, 2],
                auth_type: Some(1),
            },
        }
        .validate()
        .is_ok());
        assert!(OpenWorkSetSessionInfoRequest {
            pre_auth_code: "pre-auth".to_string(),
            session_info: OpenWorkSessionInfo {
                appid: vec![1, 1],
                auth_type: Some(2),
            },
        }
        .validate()
        .is_err());
        assert!(validate_open_work_auth_identifier_batch(
            "user id",
            &["user-a".to_string(), "user-a".to_string()],
            1_000,
        )
        .is_err());
    }

    #[test]
    fn validates_open_work_suite_auth_response_matrix() {
        let suite: SuiteTokenResponse = serde_json::from_value(json!({
            "errcode": 0,
            "suite_access_token": "suite-token",
            "expires_in": 7200
        }))
        .unwrap();
        assert!(suite.validate().is_ok());

        let api_error: SuiteTokenResponse = serde_json::from_value(json!({
            "errcode": 40001,
            "errmsg": "invalid credential"
        }))
        .unwrap();
        assert!(matches!(api_error.validate(), Err(WechatError::Api { .. })));

        let missing_token: SuiteTokenResponse =
            serde_json::from_value(json!({ "errcode": 0, "expires_in": 7200 })).unwrap();
        assert!(missing_token.validate().is_err());

        let permanent: OpenWorkPermanentCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "permanent_code": "permanent",
            "auth_corp_info": { "corpid": "corp" },
            "auth_info": { "agent": [{ "agentid": 100001 }] }
        }))
        .unwrap();
        assert!(permanent
            .validate_permanent_code("get permanent code")
            .is_ok());

        let duplicate_agents: OpenWorkPermanentCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "auth_corp_info": { "corpid": "corp" },
            "auth_info": { "agent": [{ "agentid": 1 }, { "agentid": 1 }] }
        }))
        .unwrap();
        assert!(duplicate_agents
            .validate_auth_info("get authorization info")
            .is_err());

        let converted: OpenWorkUserIdToOpenUserIdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "open_userid_list": [
                { "userid": "user-a", "open_userid": "open-a" },
                { "userid": "user-b", "open_userid": "open-b" }
            ]
        }))
        .unwrap();
        assert!(converted.validate().is_ok());
    }

    #[test]
    fn rejects_malformed_known_open_work_server_events() {
        assert!(OpenWork::parse_server_event_xml(
            r#"<xml>
                    <SuiteId><![CDATA[suite-id]]></SuiteId>
                    <InfoType><![CDATA[suite_ticket]]></InfoType>
                    <TimeStamp>1800000000</TimeStamp>
                </xml>"#,
        )
        .is_err());
        assert!(OpenWork::parse_server_event_xml(
            r#"<xml>
                    <SuiteId><![CDATA[suite-id]]></SuiteId>
                    <InfoType><![CDATA[change_contact]]></InfoType>
                    <TimeStamp>1800000000</TimeStamp>
                    <ChangeType><![CDATA[delete_user]]></ChangeType>
                    <AuthCorpId><![CDATA[corp-id]]></AuthCorpId>
                </xml>"#,
        )
        .is_err());

        let future_event = OpenWork::parse_server_event_xml(
            r#"<xml><InfoType><![CDATA[future_event]]></InfoType></xml>"#,
        )
        .unwrap();
        assert_eq!(future_event.kind(), OpenWorkServerEventKind::Unknown);
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
                "authorizer_access_token": "token",
                "func_info": [{
                    "funcscope_category": {
                        "id": 18,
                        "scope_name": "customer-service"
                    },
                    "confirm_info": {
                        "need_confirm": 1,
                        "already_confirm": 0,
                        "can_confirm": 1
                    },
                    "scope_revision": 2
                }],
                "authorization_revision": 3
            },
            "request_id": "component-query"
        }))
        .unwrap();
        let query_authorization = query
            .authorization_info
            .as_ref()
            .expect("authorization info");
        assert_eq!(
            query_authorization.authorizer_appid.as_deref(),
            Some("wx-authorizer")
        );
        assert_eq!(
            query_authorization.func_info[0]
                .funcscope_category
                .as_ref()
                .and_then(|scope| scope.id),
            Some(18)
        );
        assert_eq!(
            query_authorization.func_info[0]
                .confirm_info
                .as_ref()
                .and_then(|confirm| confirm.can_confirm),
            Some(1)
        );
        assert_eq!(query_authorization.func_info[0].extra["scope_revision"], 2);
        assert_eq!(query_authorization.extra["authorization_revision"], 3);
        assert_eq!(query.extra["request_id"], "component-query");

        let info: OpenWorkComponentAuthorizerInfoResponse = serde_json::from_value(json!({
            "authorizer_info": {
                "nick_name": "Corp App",
                "service_type_info": {
                    "id": 2,
                    "service_revision": 1
                },
                "MiniProgramInfo": {
                    "network": {
                        "RequestDomain": ["https://api.example.com"],
                        "network_revision": 2
                    },
                    "categories": [{
                        "first": "Tools",
                        "second": "Efficiency"
                    }],
                    "visit_status": 0
                },
                "authorizer_revision": 3
            },
            "authorization_info": {
                "authorizer_appid": "wx-authorizer",
                "func_info": []
            },
            "request_id": "component-info"
        }))
        .unwrap();
        let authorizer_info = info.authorizer_info.as_ref().expect("authorizer info");
        assert_eq!(authorizer_info.nick_name.as_deref(), Some("Corp App"));
        assert_eq!(
            authorizer_info
                .service_type_info
                .as_ref()
                .and_then(|service| service.id),
            Some(2)
        );
        assert_eq!(
            authorizer_info.service_type_info.as_ref().unwrap().extra["service_revision"],
            1
        );
        let mini_program = authorizer_info
            .mini_program_info
            .as_ref()
            .expect("mini program info");
        let network = mini_program.network.as_ref().expect("network");
        assert_eq!(network.request_domain[0], "https://api.example.com");
        assert_eq!(network.extra["network_revision"], 2);
        assert_eq!(
            mini_program.categories[0].second.as_deref(),
            Some("Efficiency")
        );
        assert_eq!(authorizer_info.extra["authorizer_revision"], 3);
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
            "auth_corp_info": {
                "corpid": "corp",
                "corp_name": "Corp",
                "location": "Guangdong Shenzhen",
                "corp_region": "CN"
            },
            "auth_info": {
                "agent": [{
                    "agentid": 100001,
                    "name": "App",
                    "auth_mode": 1,
                    "is_customized_app": true,
                    "privilege": {
                        "level": 3,
                        "allow_party": [1, 2],
                        "allow_user": ["user-a"],
                        "allow_tag": [10],
                        "extra_party": [3],
                        "extra_user": ["user-b"],
                        "extra_tag": [11],
                        "scope_revision": 2
                    },
                    "shared_from": {
                        "corpid": "upstream-corp",
                        "share_revision": 3
                    },
                    "edition": "pro"
                }],
                "auth_scope": "all"
            },
            "edition_info": {
                "agent": [{
                    "agentid": 100001,
                    "edition_id": "RLS65535",
                    "edition_name": "Professional",
                    "app_status": 6,
                    "user_limit": 200,
                    "expired_time": 1807776000,
                    "is_virtual_version": false,
                    "is_shared_from_other_corp": true,
                    "edition_revision": 4
                }],
                "edition_source": "provider"
            },
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
        assert_eq!(auth_corp.location.as_deref(), Some("Guangdong Shenzhen"));
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
        assert_eq!(auth_info.agent[0].is_customized_app, Some(true));
        let privilege = auth_info.agent[0]
            .privilege
            .as_ref()
            .expect("agent privilege");
        assert_eq!(privilege.allow_party, vec![1, 2]);
        assert_eq!(privilege.allow_user[0], "user-a");
        assert_eq!(
            privilege.level_kind(),
            Some(OpenWorkAuthPrivilegeLevelKind::FullReadWrite)
        );
        assert!(privilege.can_write_contacts());
        assert_eq!(privilege.extra["scope_revision"], 2);
        let shared_from = auth_info.agent[0]
            .shared_from
            .as_ref()
            .expect("shared source");
        assert_eq!(shared_from.corpid.as_deref(), Some("upstream-corp"));
        assert_eq!(shared_from.extra["share_revision"], 3);
        assert!(auth_info.agent[0].is_shared_install());
        assert_eq!(auth_info.agent[0].extra["edition"], "pro");
        let edition_info = permanent.edition_info.as_ref().expect("edition info");
        assert_eq!(edition_info.extra["edition_source"], "provider");
        let edition = &edition_info.agent[0];
        assert_eq!(edition.edition_id.as_deref(), Some("RLS65535"));
        assert_eq!(edition.edition_name.as_deref(), Some("Professional"));
        assert_eq!(
            edition.app_status_kind(),
            Some(OpenWorkAuthAppStatusKind::PurchasedOverLimitGrace)
        );
        assert!(edition.has_active_edition_entitlement());
        assert!(edition.app_status_kind().expect("app status").is_paid());
        assert_eq!(edition.user_limit, Some(200));
        assert_eq!(edition.expired_time, Some(1_807_776_000));
        assert_eq!(edition.is_virtual_version, Some(false));
        assert!(edition.is_shared_install());
        assert_eq!(edition.extra["edition_revision"], 4);
        assert_eq!(
            OpenWorkAuthAppStatusKind::from_code(2),
            OpenWorkAuthAppStatusKind::TrialExpired
        );
        assert!(!OpenWorkAuthAppStatusKind::PurchasedOverLimitExpired.has_active_entitlement());
        assert_eq!(
            OpenWorkAuthPrivilegeLevelKind::from_code(5),
            OpenWorkAuthPrivilegeLevelKind::FullWriteOnly
        );
        assert!(OpenWorkAuthPrivilegeLevelKind::FullWriteOnly.can_write());
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
            start_time: Some("1782835200".to_string()),
            end_time: Some("1783526400".to_string()),
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

    #[test]
    fn validates_open_work_license_request_matrix() {
        let new_order = OpenWorkLicenseCreateNewOrderRequest {
            corpid: Some("corp".to_string()),
            buyer_userid: Some("buyer".to_string()),
            account_count: Some(OpenWorkLicenseAccountCount {
                base_count: Some(10),
                external_contact_count: Some(2),
            }),
            account_duration: Some(OpenWorkLicenseAccountDuration {
                month: Some(1),
                days: None,
                new_expire_time: None,
            }),
        };
        assert!(new_order.validate().is_ok());
        let mut empty_count = new_order.clone();
        empty_count.account_count = Some(OpenWorkLicenseAccountCount {
            base_count: Some(0),
            external_contact_count: Some(0),
        });
        assert!(empty_count.validate().is_err());
        let mut short_duration = new_order;
        short_duration.account_duration = Some(OpenWorkLicenseAccountDuration {
            month: None,
            days: Some(30),
            new_expire_time: None,
        });
        assert!(short_duration.validate().is_err());

        let renewal_account = OpenWorkLicenseActiveInfo {
            active_code: Some("active-1".to_string()),
            userid: Some("user-1".to_string()),
            corpid: None,
            account_type: None,
            status: None,
            create_time: None,
            active_time: None,
            expire_time: None,
            merge_info: None,
            share_info: None,
            extra: Value::Null,
        };
        let renewal = OpenWorkLicenseCreateRenewOrderJobRequest {
            corpid: Some("corp".to_string()),
            account_list: vec![renewal_account.clone()],
            jobid: None,
        };
        assert!(renewal.validate().is_ok());
        let mut duplicate_renewal = renewal;
        duplicate_renewal.account_list.push(renewal_account);
        assert!(duplicate_renewal.validate().is_err());

        let submit = OpenWorkLicenseSubmitOrderJobRequest {
            jobid: Some("job".to_string()),
            buyer_userid: Some("buyer".to_string()),
            account_duration: Some(OpenWorkLicenseAccountDuration {
                month: None,
                days: None,
                new_expire_time: Some(1_830_000_000),
            }),
        };
        assert!(submit.validate().is_ok());
        let mut conflicting_duration = submit;
        conflicting_duration.account_duration = Some(OpenWorkLicenseAccountDuration {
            month: Some(1),
            days: None,
            new_expire_time: Some(1_830_000_000),
        });
        assert!(conflicting_duration.validate().is_err());

        let page = OpenWorkLicenseListOrderRequest {
            corpid: None,
            start_time: Some("1800000000".to_string()),
            end_time: Some("1802678400".to_string()),
            cursor: None,
            limit: Some(1_000),
        };
        assert!(page.validate().is_ok());
        let mut partial_range = page.clone();
        partial_range.end_time = None;
        assert!(partial_range.validate().is_err());
        let mut oversized_range = page;
        oversized_range.end_time = Some("1802678401".to_string());
        assert!(oversized_range.validate().is_err());

        let activation = OpenWorkLicenseActiveInfo {
            active_code: Some("active".to_string()),
            userid: Some("user".to_string()),
            corpid: Some("corp".to_string()),
            account_type: None,
            status: None,
            create_time: None,
            active_time: None,
            expire_time: None,
            merge_info: None,
            share_info: None,
            extra: Value::Null,
        };
        assert!(activation.validate_activation().is_ok());
        assert!(
            validate_open_work_license_active_batch(&[activation.clone(), activation]).is_err()
        );

        let transfer = OpenWorkLicenseTransferInfo {
            handover_userid: Some("old-user".to_string()),
            takeover_userid: Some("new-user".to_string()),
            errcode: None,
            extra: Value::Null,
        };
        assert!(validate_open_work_license_transfer_batch(&[transfer]).is_ok());
        let self_transfer = OpenWorkLicenseTransferInfo {
            handover_userid: Some("same-user".to_string()),
            takeover_userid: Some("same-user".to_string()),
            errcode: None,
            extra: Value::Null,
        };
        assert!(validate_open_work_license_transfer_batch(&[self_transfer]).is_err());
    }

    #[test]
    fn validates_open_work_license_response_matrix() {
        let order_id: OpenWorkLicenseOrderIdResponse =
            serde_json::from_value(json!({ "errcode": 0, "order_id": "order-1" })).unwrap();
        assert!(order_id.validate_for("create order").is_ok());

        let orders: OpenWorkLicenseListOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "has_more": 1,
            "next_cursor": "cursor-2",
            "order_list": [{
                "order_id": "order-1",
                "order_type": 1,
                "order_status": 1,
                "corpid": "corp",
                "account_count": { "base_count": 2 },
                "account_duration": { "month": 1 },
                "price": 100,
                "create_time": 1800000000,
                "pay_time": 1800000010
            }]
        }))
        .unwrap();
        assert!(orders.validate().is_ok());

        let active_info = json!({
            "active_code": "active-1",
            "userid": "user-1",
            "corpid": "corp",
            "type": 1,
            "status": 2,
            "create_time": 1800000000,
            "active_time": 1800000010,
            "expire_time": 1831536010
        });
        let active: OpenWorkLicenseActiveInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "active_info": active_info.clone()
        }))
        .unwrap();
        assert!(active.validate().is_ok());

        let account_page: OpenWorkLicenseListAccountResponse = serde_json::from_value(json!({
            "errcode": 0,
            "has_more": 0,
            "account_list": [active_info.clone()]
        }))
        .unwrap();
        assert!(account_page.validate().is_ok());

        let user_active: OpenWorkLicenseUserActiveInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "active_status": 1,
            "active_info_list": [active_info]
        }))
        .unwrap();
        assert!(user_active.validate().is_ok());

        let transfer: OpenWorkLicenseTransferResponse = serde_json::from_value(json!({
            "errcode": 0,
            "transfer_result": [{
                "handover_userid": "old-user",
                "takeover_userid": "new-user",
                "errcode": 0
            }]
        }))
        .unwrap();
        assert!(transfer.validate().is_ok());

        let license: OpenWorkLicenseInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "license_status": 1,
            "license_check_time": 1800000000,
            "trail_info": {
                "start_time": 1800000000,
                "end_time": 1807776000
            }
        }))
        .unwrap();
        assert!(license.validate().is_ok());

        let auto_active: OpenWorkLicenseAutoActiveStatusResponse =
            serde_json::from_value(json!({ "errcode": 0, "auto_active_status": 1 })).unwrap();
        assert!(auto_active.validate().is_ok());
    }

    #[test]
    fn rejects_inconsistent_open_work_license_responses() {
        let api_error: OpenWorkLicenseOrderIdResponse = serde_json::from_value(json!({
            "errcode": 40001,
            "errmsg": "invalid credential"
        }))
        .unwrap();
        assert!(matches!(
            api_error.validate_for("create order"),
            Err(WechatError::Api { .. })
        ));

        let missing_order_id: OpenWorkLicenseOrderIdResponse =
            serde_json::from_value(json!({ "errcode": 0 })).unwrap();
        assert!(missing_order_id.validate_for("create order").is_err());

        let stalled_page: OpenWorkLicenseListOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "has_more": 1,
            "next_cursor": "cursor",
            "order_list": []
        }))
        .unwrap();
        assert!(stalled_page.validate().is_err());

        let duplicate_accounts: OpenWorkLicenseActiveInfoListResponse =
            serde_json::from_value(json!({
                "errcode": 0,
                "active_info_list": [{
                    "active_code": "active",
                    "userid": "user",
                    "corpid": "corp",
                    "type": 1,
                    "status": 2
                }, {
                    "active_code": "active",
                    "userid": "other",
                    "corpid": "corp",
                    "type": 1,
                    "status": 2
                }]
            }))
            .unwrap();
        assert!(duplicate_accounts.validate().is_err());

        let invalid_trial: OpenWorkLicenseInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "license_status": 1,
            "trail_info": {
                "start_time": 1800000010,
                "end_time": 1800000000
            }
        }))
        .unwrap();
        assert!(invalid_trial.validate().is_err());

        let missing_active_list: OpenWorkLicenseUserActiveInfoResponse =
            serde_json::from_value(json!({
                "errcode": 0,
                "active_status": 1,
                "active_info_list": []
            }))
            .unwrap();
        assert!(missing_active_list.validate().is_err());
    }
}
