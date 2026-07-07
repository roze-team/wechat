use std::time::Duration;

use bytes::Bytes;
use quick_xml::de::from_str as from_xml_str;
use reqwest::{multipart, Identity, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tracing::debug;

use crate::{
    config::WechatConfig,
    error::{Result, WechatError},
};

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    config: WechatConfig,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
    pub access_token: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ApiRequest<T> {
    pub endpoint: Endpoint,
    pub query: Vec<(String, String)>,
    pub body: Option<T>,
}

impl Client {
    pub fn new(config: WechatConfig) -> Result<Self> {
        let mut builder = reqwest::Client::builder()
            .timeout(config.timeout())
            .pool_idle_timeout(Duration::from_secs(90));
        if let Some(identity_pem) = config.client_identity_pem.as_deref() {
            builder = builder.identity(Identity::from_pem(identity_pem.as_bytes())?);
        }
        let http = builder.build()?;

        Ok(Self { http, config })
    }

    pub async fn execute<B, R>(&self, request: ApiRequest<B>) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(request.endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in request.query {
                pairs.append_pair(&key, &value);
            }
            if let Some(token) = request.endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self.http.request(request.endpoint.method, url);
        for (key, value) in request.endpoint.headers {
            builder = builder.header(key, value);
        }
        if let Some(body) = request.body {
            builder = builder.json(&body);
        }

        let response = builder.send().await?.error_for_status()?;
        let value = response.json::<Value>().await?;
        debug!(wechat_response = %value, "wechat api response");

        if let Some(code) = value.get("errcode").and_then(Value::as_i64) {
            if code != 0 {
                let message = value
                    .get("errmsg")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown wechat error")
                    .to_string();
                return Err(WechatError::Api { code, message });
            }
        }

        Ok(serde_json::from_value(value)?)
    }

    pub async fn execute_json<R>(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query {
                pairs.append_pair(&key, &value);
            }
            if let Some(token) = endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self.http.request(endpoint.method, url);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
        }
        if let Some(body) = body {
            builder = builder
                .header("content-type", "application/json")
                .body(body.to_string());
        }

        let response = builder.send().await?.error_for_status()?;
        let value = response.json::<Value>().await?;
        debug!(wechat_response = %value, "wechat api response");

        if let Some(code) = value.get("errcode").and_then(Value::as_i64) {
            if code != 0 {
                let message = value
                    .get("errmsg")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown wechat error")
                    .to_string();
                return Err(WechatError::Api { code, message });
            }
        }

        Ok(serde_json::from_value(value)?)
    }

    pub async fn execute_bytes(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<Bytes> {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query {
                pairs.append_pair(&key, &value);
            }
            if let Some(token) = endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self.http.request(endpoint.method, url);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
        }
        if let Some(body) = body {
            builder = builder
                .header("content-type", "application/json")
                .body(body.to_string());
        }

        Ok(builder.send().await?.error_for_status()?.bytes().await?)
    }

    pub async fn execute_xml<R>(&self, endpoint: Endpoint, body: String) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            if let Some(token) = endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self
            .http
            .request(endpoint.method, url)
            .header("content-type", "text/xml; charset=utf-8")
            .body(body);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
        }

        let text = builder.send().await?.error_for_status()?.text().await?;
        debug!(wechat_response = %text, "wechat xml response");

        from_xml_str(&text).map_err(|err| WechatError::Xml(err.to_string()))
    }

    pub async fn execute_raw_json<R>(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        content_type: String,
        body: Vec<u8>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query {
                pairs.append_pair(&key, &value);
            }
            if let Some(token) = endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self
            .http
            .request(endpoint.method, url)
            .header("content-type", content_type)
            .body(body);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
        }

        let response = builder.send().await?.error_for_status()?;
        let value = response.json::<Value>().await?;
        debug!(wechat_response = %value, "wechat raw body response");

        if let Some(code) = value.get("errcode").and_then(Value::as_i64) {
            if code != 0 {
                let message = value
                    .get("errmsg")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown wechat error")
                    .to_string();
                return Err(WechatError::Api { code, message });
            }
        }

        Ok(serde_json::from_value(value)?)
    }

    pub async fn execute_multipart<R>(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        form: multipart::Form,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| WechatError::Config(format!("invalid base_url: {err}")))?;
        url.set_path(endpoint.path.trim_start_matches('/'));

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query {
                pairs.append_pair(&key, &value);
            }
            if let Some(token) = endpoint.access_token {
                pairs.append_pair("access_token", &token);
            }
        }

        let mut builder = self.http.request(endpoint.method, url).multipart(form);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
        }

        let response = builder.send().await?.error_for_status()?;
        let value = response.json::<Value>().await?;
        debug!(wechat_response = %value, "wechat multipart response");

        if let Some(code) = value.get("errcode").and_then(Value::as_i64) {
            if code != 0 {
                let message = value
                    .get("errmsg")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown wechat error")
                    .to_string();
                return Err(WechatError::Api { code, message });
            }
        }

        Ok(serde_json::from_value(value)?)
    }

    pub fn config(&self) -> &WechatConfig {
        &self.config
    }
}

impl Endpoint {
    pub fn get(path: impl Into<String>) -> Self {
        Self {
            method: Method::GET,
            path: path.into(),
            access_token: None,
            headers: Vec::new(),
        }
    }

    pub fn post(path: impl Into<String>) -> Self {
        Self {
            method: Method::POST,
            path: path.into(),
            access_token: None,
            headers: Vec::new(),
        }
    }

    pub fn put(path: impl Into<String>) -> Self {
        Self {
            method: Method::PUT,
            path: path.into(),
            access_token: None,
            headers: Vec::new(),
        }
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self {
            method: Method::DELETE,
            path: path.into(),
            access_token: None,
            headers: Vec::new(),
        }
    }

    pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
        self.access_token = Some(token.into());
        self
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
}
