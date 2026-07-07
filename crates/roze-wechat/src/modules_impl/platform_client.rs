use bytes::Bytes;
use reqwest::multipart;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    client::{ApiRequest, Client, Endpoint},
    config::Platform,
    error::Result,
};

#[derive(Debug, Clone)]
pub struct PlatformClient {
    client: Client,
    platform: Platform,
}

impl PlatformClient {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self { client, platform }
    }

    pub fn platform(&self) -> Platform {
        self.platform
    }

    pub async fn get<R>(&self, path: impl Into<String>, access_token: Option<String>) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.get_with_query(path, access_token, Vec::new()).await
    }

    pub async fn get_with_query<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::get(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        self.client
            .execute(ApiRequest::<()> {
                endpoint,
                query,
                body: None,
            })
            .await
    }

    pub async fn get_with_headers<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::get(path);
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client.execute_json(endpoint, query, None).await
    }

    pub async fn get_bytes(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
    ) -> Result<Bytes> {
        let mut endpoint = Endpoint::get(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        self.client.execute_bytes(endpoint, query, None).await
    }

    pub async fn post_json_bytes(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: Value,
    ) -> Result<Bytes> {
        let mut endpoint = Endpoint::post(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        self.client
            .execute_bytes(endpoint, Vec::new(), Some(body))
            .await
    }

    pub async fn post<B, R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: B,
    ) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::post(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        self.client
            .execute(ApiRequest {
                endpoint,
                query: Vec::new(),
                body: Some(body),
            })
            .await
    }

    pub async fn post_json<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::post(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client
            .execute_json(endpoint, Vec::new(), Some(body))
            .await
    }

    pub async fn post_json_with_query<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::post(path);
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client.execute_json(endpoint, query, Some(body)).await
    }

    pub async fn put_json<R>(
        &self,
        path: impl Into<String>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::put(path);
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client
            .execute_json(endpoint, Vec::new(), Some(body))
            .await
    }

    pub async fn delete_json<R>(
        &self,
        path: impl Into<String>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::delete(path);
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client.execute_json(endpoint, Vec::new(), None).await
    }

    pub async fn post_multipart<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
        form: multipart::Form,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::post(path);
        if let Some(token) = access_token {
            endpoint = endpoint.with_access_token(token);
        }
        self.client.execute_multipart(endpoint, query, form).await
    }

    pub async fn post_xml<R>(&self, path: impl Into<String>, body: String) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let endpoint = Endpoint::post(path);
        self.client.execute_xml(endpoint, body).await
    }

    pub async fn post_raw_json<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        content_type: String,
        body: Vec<u8>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut endpoint = Endpoint::post(path);
        for (key, value) in headers {
            endpoint = endpoint.with_header(key, value);
        }
        self.client
            .execute_raw_json(endpoint, query, content_type, body)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct DomainModule {
    inner: PlatformClient,
    name: &'static str,
}

impl DomainModule {
    pub fn new(inner: PlatformClient, name: &'static str) -> Self {
        Self { inner, name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn platform(&self) -> Platform {
        self.inner.platform()
    }

    pub async fn get<R>(&self, path: impl Into<String>, access_token: Option<String>) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.get(path, access_token).await
    }

    pub async fn get_with_query<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.get_with_query(path, access_token, query).await
    }

    pub async fn get_with_headers<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.get_with_headers(path, query, headers).await
    }

    pub async fn get_bytes(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
    ) -> Result<Bytes> {
        self.inner.get_bytes(path, access_token, query).await
    }

    pub async fn post_json_bytes(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: Value,
    ) -> Result<Bytes> {
        self.inner.post_json_bytes(path, access_token, body).await
    }

    pub async fn post<B, R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: B,
    ) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.inner.post(path, access_token, body).await
    }

    pub async fn post_json<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner
            .post_json(path, access_token, body, headers)
            .await
    }

    pub async fn post_json_with_query<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner
            .post_json_with_query(path, query, body, headers)
            .await
    }

    pub async fn put_json<R>(
        &self,
        path: impl Into<String>,
        body: Value,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.put_json(path, body, headers).await
    }

    pub async fn delete_json<R>(
        &self,
        path: impl Into<String>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.delete_json(path, headers).await
    }

    pub async fn post_multipart<R>(
        &self,
        path: impl Into<String>,
        access_token: Option<String>,
        query: Vec<(String, String)>,
        form: multipart::Form,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner
            .post_multipart(path, access_token, query, form)
            .await
    }

    pub async fn post_xml<R>(&self, path: impl Into<String>, body: String) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner.post_xml(path, body).await
    }

    pub async fn post_raw_json<R>(
        &self,
        path: impl Into<String>,
        query: Vec<(String, String)>,
        content_type: String,
        body: Vec<u8>,
        headers: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.inner
            .post_raw_json(path, query, content_type, body, headers)
            .await
    }
}
