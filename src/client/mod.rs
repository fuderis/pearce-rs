//! HTTP client module.

use crate::prelude::*;

use reqwest::{Client as ReqClient, Method, RequestBuilder};

static TCP_CLIENT: State<ReqClient> = State::new(|| ReqClient::new());

/// HTTP client (based on [reqwest]).
#[derive(Debug, Clone)]
pub struct Client {
    inner: ReqClient,
    base_url: Option<String>,
}

impl Client {
    /// Creates new `TCP` client (clone from state).
    pub fn tcp() -> Self {
        Client {
            inner: TCP_CLIENT.dirty_get_cloned(),
            base_url: None,
        }
    }

    /// Creates new `ICP` client.
    pub fn ipc(endpoint: &str) -> Self {
        let req_client = ReqClient::builder()
            .pool_max_idle_per_host(1) // important for IPC
            .unix_socket(endpoint) // magic of reqwest 0.13+
            .build()
            .unwrap();

        Client {
            inner: req_client,
            // set "http://localhost" as base URL by default
            base_url: Some("http://localhost".to_string()),
        }
    }

    /// Creates request builder.
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.format_url(path);
        self.inner.request(method, url)
    }

    /// Creates `GET` request.
    pub fn get(&self, path: &str) -> RequestBuilder {
        self.request(Method::GET, path)
    }

    /// Creates `POST` request.
    pub fn post(&self, path: &str) -> RequestBuilder {
        self.request(Method::POST, path)
    }

    /// Creates `DELETE` request.
    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.request(Method::DELETE, path)
    }

    /// Creates `PUT` request.
    pub fn put(&self, path: &str) -> RequestBuilder {
        self.request(Method::PUT, path)
    }

    /// Creates `PATCH` request.
    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.request(Method::PATCH, path)
    }

    /// Creates `HEAD` request.
    pub fn head(&self, path: &str) -> RequestBuilder {
        self.request(Method::HEAD, path)
    }

    /// Creates `OPTIONS` request.
    pub fn options(&self, path: &str) -> RequestBuilder {
        self.request(Method::OPTIONS, path)
    }

    /// Helper method to prepare URLs.
    fn format_url(&self, path: &str) -> String {
        match &self.base_url {
            // for IPC (http://localhost + /your/path)
            Some(base) => format!(
                "{}{}",
                base,
                if path.starts_with('/') {
                    path.to_string()
                } else {
                    format!("/{path}")
                }
            ),
            // for TCP (no format)
            None => path.to_string(),
        }
    }
}

#[cfg(feature = "stream")]
pub trait StreamExt {
    fn stream<T>(self) -> impl std::future::Future<Output = Result<Receiver<T>>> + Send
    where
        T: serde::de::DeserializeOwned + Send + 'static;
}

#[cfg(feature = "stream")]
impl StreamExt for reqwest::RequestBuilder {
    async fn stream<T>(self) -> Result<Receiver<T>>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        use futures::StreamExt;

        let response = self.send().await?;
        let stream = response.bytes_stream().map(|v| v.map_err(Into::into));

        Ok(crate::stream_reader(stream))
    }
}
