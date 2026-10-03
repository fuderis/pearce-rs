//! HTTP client module.

use crate::{Header, IntoHeader, Method, prelude::*};
use std::time::Duration;

use reqwest::Client as ReqwestClient;
pub use reqwest::{self, Proxy, Response};

#[cfg(feature = "stream")]
use serde::de::DeserializeOwned;

static TCP_CLIENT: State<ReqwestClient> = State::new(|| ReqwestClient::new());

/// HTTP client (based on [reqwest]).
#[derive(Debug, Clone)]
pub struct Client {
    inner: ReqwestClient,
    base_url: Option<String>,
}

impl Client {
    /// Creates new `TCP` client (clone from state).
    pub fn tcp() -> Self {
        Client {
            inner: TCP_CLIENT.get_cloned(),
            base_url: None,
        }
    }

    /// Creates new `IPC` client.
    pub fn ipc(endpoint: &str) -> Self {
        let req_client = ReqwestClient::builder()
            .pool_max_idle_per_host(1) // important for IPC
            .unix_socket(endpoint) // magic of reqwest 0.13+
            .build()
            .unwrap();

        Client {
            inner: req_client,
            base_url: Some("http://127.0.0.1".to_string()),
        }
    }

    /// Creates request builder.
    pub fn request(&self, method: Method, path: &str) -> Request {
        Request::new(self.clone(), method, path)
    }

    /// Creates `GET` request.
    pub fn get(&self, path: &str) -> Request {
        self.request(Method::Get, path)
    }

    /// Creates `POST` request.
    pub fn post(&self, path: &str) -> Request {
        self.request(Method::Post, path)
    }

    /// Creates `DELETE` request.
    pub fn delete(&self, path: &str) -> Request {
        self.request(Method::Delete, path)
    }

    /// Creates `PUT` request.
    pub fn put(&self, path: &str) -> Request {
        self.request(Method::Put, path)
    }

    /// Creates `PATCH` request.
    pub fn patch(&self, path: &str) -> Request {
        self.request(Method::Patch, path)
    }

    /// Creates `HEAD` request.
    pub fn head(&self, path: &str) -> Request {
        self.request(Method::Head, path)
    }

    /// Creates `OPTIONS` request.
    pub fn options(&self, path: &str) -> Request {
        self.request(Method::Options, path)
    }

    /// Helper method to prepare URLs.
    fn format_url(&self, path: &str) -> String {
        match &self.base_url {
            Some(base) => format!(
                "{}{}",
                base,
                if path.starts_with('/') {
                    path.to_string()
                } else {
                    format!("/{path}")
                }
            ),
            None => path.to_string(),
        }
    }
}

/// HTTP request builder.
pub struct Request {
    client: Client,
    method: Method,
    path: String,

    headers: reqwest::header::HeaderMap,
    timeout: Option<Duration>,
    query: Vec<(String, String)>,
    body: Option<StdResult<reqwest::Body, serde_json::Error>>,

    proxy: Option<Proxy>,
    connect_timeout: Option<Duration>,
    accept_invalid_certs: bool,
}

impl Request {
    /// Creates new request builder.
    fn new(client: Client, method: Method, path: &str) -> Self {
        Self {
            client,
            method,
            path: path.to_string(),
            headers: reqwest::header::HeaderMap::new(),
            timeout: None,
            query: Vec::new(),
            body: None,
            proxy: None,
            connect_timeout: None,
            accept_invalid_certs: false,
        }
    }

    /// Sets HTTP header.
    pub fn header(mut self, key: impl IntoHeader, value: impl Into<String>) -> Self {
        let key_str = key.header_str();
        if let (Ok(k), Ok(v)) = (
            reqwest::header::HeaderName::from_bytes(key_str.as_bytes()),
            reqwest::header::HeaderValue::from_str(&value.into()),
        ) {
            self.headers.insert(k, v);
        }
        self
    }

    /// Sets request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets timeout to connect.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Sets proxy options.
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Controls TLS validation for invalid/self-signed certificates.
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Sets query parameters.
    pub fn query<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.query.push((key.to_string(), value.to_string()));
        self
    }

    /// Serializes into JSON body and adds Content-Type header.
    pub fn json<T: Serialize>(mut self, json: &T) -> Self {
        self = self.header(Header::ContentType, "application/json");
        let body_res = serde_json::to_vec(json).map(reqwest::Body::from);
        self.body = Some(body_res);
        self
    }

    /// Sets raw request body.
    pub fn body(mut self, body: impl Into<reqwest::Body>) -> Self {
        self.body = Some(Ok(body.into()));
        self
    }

    /// Sends request to endpoint.
    pub async fn send(self) -> Result<Response> {
        let raw_url = self.client.format_url(&self.path);

        let mut url = reqwest::Url::parse(&raw_url)?;
        if !self.query.is_empty() {
            url.query_pairs_mut().extend_pairs(&self.query);
        }

        let req_client = if self.proxy.is_some()
            || self.connect_timeout.is_some()
            || self.accept_invalid_certs
        {
            let mut builder = ReqwestClient::builder();
            if let Some(p) = self.proxy {
                builder = builder.proxy(p);
            }
            if let Some(ct) = self.connect_timeout {
                builder = builder.connect_timeout(ct);
            }
            if self.accept_invalid_certs {
                builder = builder.danger_accept_invalid_certs(true);
            }
            builder.build()?
        } else {
            self.client.inner.clone()
        };

        let mut req = req_client.request(self.method.into(), url);

        if let Some(t) = self.timeout {
            req = req.timeout(t);
        }

        if !self.headers.is_empty() {
            req = req.headers(self.headers);
        }

        if let Some(b_res) = self.body {
            req = req.body(b_res?);
        }

        let resp = req.send().await?;
        Ok(resp)
    }

    #[cfg(feature = "stream")]
    pub async fn stream<T>(self) -> Result<Receiver<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        use futures::StreamExt;

        let response = self.send().await?;
        let stream = response.bytes_stream().map(|v| v.map_err(Into::into));

        Ok(crate::stream::stream_reader(stream))
    }
}
