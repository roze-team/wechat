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

    pub async fn provider_token(
        &self,
        request: ProviderTokenRequest,
    ) -> Result<ProviderTokenResponse> {
        self.inner
            .post("cgi-bin/service/get_provider_token", None, request)
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

    pub async fn pre_auth_code(&self, suite_access_token: impl Into<String>) -> Result<Value> {
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
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/service/get_permanent_code",
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
    ) -> Result<Value> {
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

    pub async fn corp_token(
        &self,
        suite_access_token: impl Into<String>,
        auth_corpid: impl Into<String>,
        permanent_code: impl Into<String>,
    ) -> Result<Value> {
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

    pub fn license(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.license")
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "open_work.server")
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        OpenWorkUserInfo3rdByCodeResponse, OpenWorkUserInfo3rdByUserTicketResponse,
        ProviderTokenRequest, ProviderTokenResponse, SuiteTokenRequest, SuiteTokenResponse,
    };

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
}
