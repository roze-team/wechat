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

#[derive(Debug, Clone)]
pub struct HttpBytesResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Bytes,
}

#[derive(Debug)]
pub struct HttpStreamResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    response: reqwest::Response,
}

impl HttpStreamResponse {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    pub async fn next_chunk(&mut self) -> Result<Option<Bytes>> {
        Ok(self.response.chunk().await?)
    }
}

impl HttpBytesResponse {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
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
        Ok(self
            .execute_bytes_response(endpoint, query, body)
            .await?
            .body)
    }

    pub async fn execute_bytes_response(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<HttpBytesResponse> {
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
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|value| (name.as_str().to_string(), value.to_string()))
            })
            .collect();
        let body = response.bytes().await?;

        if let Some(error) = api_error_from_bytes(&body) {
            return Err(error);
        }

        Ok(HttpBytesResponse {
            status,
            headers,
            body,
        })
    }

    pub async fn execute_stream_response(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<HttpStreamResponse> {
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
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|value| (name.as_str().to_string(), value.to_string()))
            })
            .collect();

        Ok(HttpStreamResponse {
            status,
            headers,
            response,
        })
    }

    pub async fn execute_form_bytes(
        &self,
        endpoint: Endpoint,
        query: Vec<(String, String)>,
        form: Vec<(String, String)>,
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

        let body = form_urlencoded_body(form);
        let mut builder = self
            .http
            .request(endpoint.method, url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body);
        for (key, value) in endpoint.headers {
            builder = builder.header(key, value);
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

fn api_error_from_bytes(body: &[u8]) -> Option<WechatError> {
    let value = serde_json::from_slice::<Value>(body).ok()?;
    let code = value.get("errcode").and_then(Value::as_i64)?;
    if code == 0 {
        return None;
    }
    let message = value
        .get("errmsg")
        .and_then(Value::as_str)
        .unwrap_or("unknown wechat error")
        .to_string();
    Some(WechatError::Api { code, message })
}

fn form_urlencoded_body(form: Vec<(String, String)>) -> String {
    form.into_iter()
        .map(|(key, value)| format!("{}={}", percent_encode(&key), percent_encode(&value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
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

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use super::*;

    #[tokio::test]
    async fn streams_http_response_body_and_metadata() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 256];
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let read = socket.read(&mut buffer).unwrap();
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..read]);
            }
            socket
                .write_all(
                    b"HTTP/1.1 206 Partial Content\r\nContent-Length: 10\r\nContent-Range: bytes 0-9/10\r\nConnection: close\r\n\r\n01234",
                )
                .unwrap();
            socket.write_all(b"56789").unwrap();
        });
        let client = Client::new(WechatConfig {
            base_url: format!("http://{address}"),
            ..WechatConfig::default()
        })
        .unwrap();

        let mut response = client
            .execute_stream_response(Endpoint::get("/bill"), Vec::new(), None)
            .await
            .unwrap();
        assert_eq!(response.status, 206);
        assert_eq!(response.header("content-range"), Some("bytes 0-9/10"));

        let mut body = Vec::new();
        while let Some(chunk) = response.next_chunk().await.unwrap() {
            body.extend_from_slice(&chunk);
        }
        assert_eq!(body, b"0123456789");
        server.join().unwrap();
    }

    #[tokio::test]
    async fn accepts_no_content_bytes_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 256];
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let read = socket.read(&mut buffer).unwrap();
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..read]);
            }
            socket
                .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
                .unwrap();
        });
        let client = Client::new(WechatConfig {
            base_url: format!("http://{address}"),
            ..WechatConfig::default()
        })
        .unwrap();

        let body = client
            .execute_bytes(Endpoint::delete("/notification"), Vec::new(), None)
            .await
            .unwrap();
        assert!(body.is_empty());
        server.join().unwrap();
    }

    #[test]
    fn detects_api_errors_in_binary_responses() {
        let error = api_error_from_bytes(br#"{"errcode":40007,"errmsg":"invalid media_id"}"#)
            .expect("api error");
        match error {
            WechatError::Api { code, message } => {
                assert_eq!(code, 40007);
                assert_eq!(message, "invalid media_id");
            }
            other => panic!("unexpected error: {other}"),
        }

        assert!(api_error_from_bytes(br#"{"errcode":0,"errmsg":"ok"}"#).is_none());
        assert!(api_error_from_bytes(b"binary payload").is_none());

        let response = HttpBytesResponse {
            status: 206,
            headers: vec![("Content-Range".to_string(), "bytes 0-9/10".to_string())],
            body: Bytes::from_static(b"0123456789"),
        };
        assert_eq!(response.header("content-range"), Some("bytes 0-9/10"));
    }
}
