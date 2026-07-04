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
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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
        self.post_component_json(
            component_access_token,
            "wxa/addtotemplate",
            json!({
                "draft_id": draft_id,
                "template_type": template_type,
            }),
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
        self.post_component_json(
            component_access_token,
            "wxa/deletetemplate",
            json!({ "template_id": template_id.into() }),
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
pub struct AuthorizerAccessTokenRequest {
    pub component_appid: String,
    pub authorizer_appid: String,
    pub authorizer_refresh_token: String,
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

    use super::{ComponentAccessTokenRequest, RegisterMiniProgramRequest};

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
}
