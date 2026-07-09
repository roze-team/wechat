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

    pub async fn get_authorizer_info(
        &self,
        component_access_token: impl Into<String>,
        component_appid: impl Into<String>,
        authorizer_appid: impl Into<String>,
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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

    pub async fn templates(&self, component_access_token: impl Into<String>) -> Result<Value> {
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
pub struct AddTemplateFromDraftRequest {
    pub draft_id: i64,
    pub template_type: i64,
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
        OpenPlatformAuthorizerInfoResponse, OpenPlatformAuthorizerOptionResponse,
        OpenPlatformAuthorizersResponse, OpenPlatformHandleAuthorizeResponse,
        OpenPlatformStatusResponse, PreauthCodeResponse, QueryAuthResponse,
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
