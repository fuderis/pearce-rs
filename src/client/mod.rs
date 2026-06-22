use crate::prelude::*;
use reqwest::{Client as ReqClient, Method, RequestBuilder};

static TCP_CLIENT: State<ReqClient> = State::new(|| ReqClient::new());

// The HTTP client
pub struct Client {
    inner: ReqClient,
    base_url: Option<String>,
}

impl Client {
    /// Creates a new TCP client (clone from state)
    pub fn tcp() -> Self {
        Client {
            inner: TCP_CLIENT.dirty_get_cloned(),
            base_url: None,
        }
    }

    /// Creates a new ICP client
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

    /// Creates the request builder
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.format_url(path);
        self.inner.request(method, url)
    }

    /// Creates the GET request
    pub fn get(&self, path: &str) -> RequestBuilder {
        self.request(Method::GET, path)
    }

    /// Creates the POST request
    pub fn post(&self, path: &str) -> RequestBuilder {
        self.request(Method::POST, path)
    }

    /// Creates the DELETE request
    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.request(Method::DELETE, path)
    }

    /// Creates the PUT request
    pub fn put(&self, path: &str) -> RequestBuilder {
        self.request(Method::PUT, path)
    }

    /// Creates the PATCH request
    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.request(Method::PATCH, path)
    }

    /// Creates the HEAD request
    pub fn head(&self, path: &str) -> RequestBuilder {
        self.request(Method::HEAD, path)
    }

    /// Creates the OPTIONS request
    pub fn options(&self, path: &str) -> RequestBuilder {
        self.request(Method::OPTIONS, path)
    }

    /// Helper method to prepare URLs
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
