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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ProviderTokenRequest, SuiteTokenRequest};

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
}
