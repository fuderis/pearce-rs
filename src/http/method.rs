use crate::prelude::*;

#[cfg(feature = "server")]
use axum::http::Method as HttpMethod;
#[cfg(all(feature = "client", not(feature = "server")))]
use reqwest::Method as HttpMethod;

/// HTTP method.
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[display(rename_all = "UPPERCASE")]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[cfg(any(feature = "client", feature = "server"))]
impl Into<HttpMethod> for Method {
    fn into(self: Self) -> HttpMethod {
        match self {
            Self::Post => HttpMethod::POST,
            Self::Get => HttpMethod::GET,
            Self::Put => HttpMethod::PUT,
            Self::Delete => HttpMethod::DELETE,
            Self::Patch => HttpMethod::PATCH,
            Self::Head => HttpMethod::HEAD,
            Self::Options => HttpMethod::OPTIONS,
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl From<HttpMethod> for Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::POST => Self::Post,
            HttpMethod::GET => Self::Get,
            HttpMethod::PUT => Self::Put,
            HttpMethod::DELETE => Self::Delete,
            HttpMethod::PATCH => Self::Patch,
            HttpMethod::HEAD => Self::Head,
            HttpMethod::OPTIONS => Self::Options,
            _ => todo!(),
        }
    }
}
