use super::{Header, HeaderBody, Status};
use crate::prelude::*;

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, StatusCode, response::Builder},
    response::{IntoResponse, Response as AxumResponse},
};
use bytes::Bytes;
use futures::Stream as FuturesStream;
use std::result::Result as StdResult;

/// The HTTP response builder
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Body,
    err: Option<DynError>,
}

impl Response {
    /// Creates a new response builder
    pub fn new(status: impl Into<Status>) -> Self {
        Self {
            status: status.into().try_into().expect("Invalid status code"),
            headers: HeaderMap::new(),
            body: Body::empty(),
            err: None,
        }
    }

    // --- HTTP STATUS ---

    /// Changes the status code (for example, 201, 404, 500)
    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.status = status.into().try_into().expect("Invalid status code");
        self
    }

    pub fn ok() -> Self {
        Self::new(200)
    }
    pub fn created() -> Self {
        Self::new(201)
    }
    pub fn no_content() -> Self {
        Self::new(204)
    }
    pub fn redirect() -> Self {
        Self::new(301)
    }
    pub fn temp_redirect() -> Self {
        Self::new(302)
    }
    pub fn not_modified() -> Self {
        Self::new(304)
    }
    pub fn bad_request() -> Self {
        Self::new(400)
    }
    pub fn unauthorized() -> Self {
        Self::new(401)
    }
    pub fn forbidden() -> Self {
        Self::new(403)
    }
    pub fn not_found() -> Self {
        Self::new(404)
    }
    pub fn bad_method() -> Self {
        Self::new(405)
    }
    pub fn conflict() -> Self {
        Self::new(409)
    }
    pub fn too_large() -> Self {
        Self::new(413)
    }
    pub fn bad_entity() -> Self {
        Self::new(422)
    }
    pub fn rate_limited() -> Self {
        Self::new(429)
    }
    pub fn error() -> Self {
        Self::new(500)
    }
    pub fn bad_gateway() -> Self {
        Self::new(502)
    }
    pub fn unavailable() -> Self {
        Self::new(503)
    }
    pub fn timeout() -> Self {
        Self::new(504)
    }

    // --- HTTP HEADER ---

    /// Sets the header
    pub fn header<'a>(
        mut self,
        header: impl Into<Header>,
        value: impl Into<HeaderBody<'a>>,
    ) -> Self {
        if self.err.is_some() {
            return self;
        }

        match HeaderValue::from_str(value.into().0) {
            Ok(val) => {
                self.headers.insert(header.into().0, val);
            }
            Err(e) => {
                self.err = Some(e.into());
            }
        }
        self
    }

    /// Sets the HTTPS-only connect header
    pub fn https_only(self, seconds: u64) -> Self {
        let val = format!("max-age={}; includeSubDomains", seconds);
        self.header(Header::StrictTransportSecurity, val.as_str())
    }

    /// Sets the iframe options header
    pub fn no_iframe(self) -> Self {
        self.header(Header::XFrameOptions, "DENY")
    }

    /// Sets the content sniff options header
    pub fn no_sniff(self) -> Self {
        self.header(Header::XContentTypeOptions, "nosniff")
    }

    /// Sets the cache control options header
    pub fn cache_control<'a>(self, value: impl Into<HeaderBody<'a>>) -> Self {
        self.header(Header::CacheControl, value)
    }

    /// Sets the cache control options header
    pub fn no_cache(self) -> Self {
        self.cache_control("no-store, no-cache, must-revalidate")
    }

    /// Sets the content-type header
    pub fn content_type<'a>(self, value: impl Into<HeaderBody<'a>>) -> Self {
        self.header(Header::ContentType, value)
    }

    pub fn content_type_text(self) -> Self {
        self.content_type("text/plain; charset=utf-8")
    }

    pub fn content_type_json(self) -> Self {
        self.content_type("application/json")
    }

    pub fn content_type_html(self) -> Self {
        self.content_type("text/html; charset=utf-8")
    }

    pub fn content_type_stream(self) -> Self {
        self.header(Header::ContentType, "text/event-stream")
            .header(Header::CacheControl, "no-cache")
            .header(Header::Connection, "keep-alive")
            .header(Header::XContentTypeOptions, "nosniff")
    }

    pub fn allow_origin(self) -> Self {
        self.header(Header::AccessControlAllowOrigin, "*")
    }

    pub fn allow_methods(self) -> Self {
        self.header(
            Header::AccessControlAllowMethods,
            "GET, POST, PUT, DELETE, OPTIONS",
        )
    }

    pub fn allow_headers(self) -> Self {
        self.header(
            Header::AccessControlAllowHeaders,
            "Content-Type, Authorization",
        )
    }

    /// Sets the file attachment header
    pub fn attachment(self, filename: &str) -> Self {
        let value = if filename.is_ascii() {
            filename.to_string()
        } else {
            urlencoding::encode(filename).into_owned()
        };

        self.header(
            Header::ContentDisposition,
            format!("attachment; filename*=UTF-8''{value}").as_str(),
        )
    }

    /// Sets the redirect location header
    pub fn location(self, uri: &str) -> Self {
        let value = if uri.is_ascii() {
            uri.to_string()
        } else {
            urlencoding::encode(uri).into_owned()
        };

        self.header(Header::Location, value.as_str())
    }

    // --- HTTP BODY ---

    /// Sets the body (string, bytes, or stream)
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = body.into();
        self
    }

    /// Sets the plain text body (forced UTF-8)
    pub fn text(self, text: impl Into<String>) -> Self {
        self.content_type_text().body(text.into())
    }

    /// Sets the HTML content body
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self = self.content_type_html();
        self.body = Body::from(html.into());
        self
    }

    /// Sets the JSON content body
    pub fn json<T: serde::Serialize>(mut self, v: &T) -> Self {
        match serde_json::to_vec(v) {
            Ok(bytes) => {
                self = self.content_type_json();
                self.body = Body::from(bytes);
                self
            }
            Err(e) => {
                self.status = StatusCode::INTERNAL_SERVER_ERROR;
                self = self.content_type_json();
                self.body = Body::from(format!(r#"{{"error":"Serialization failed: {}"}}"#, e));
                self
            }
        }
    }

    /// Sets the stream event body
    pub fn stream<S, E>(mut self, stream: S) -> Self
    where
        S: FuturesStream<Item = StdResult<Bytes, E>> + Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self = self.content_type_stream();
        self.body = Body::from_stream(stream);
        self
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        // if error — return 500:
        if let Some(e) = self.err {
            return AxumResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "text/plain; charset=utf-8")
                .body(Body::from(format!("Internal Server Error: {e}")))
                .unwrap();
        }

        // build Axum Response:
        let mut builder = Builder::new().status(self.status);

        if let Some(headers_mut) = builder.headers_mut() {
            *headers_mut = self.headers;
        }

        match builder.body(self.body) {
            Ok(r) => r,
            Err(e) => AxumResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Response build failed: {e}")))
                .unwrap(),
        }
    }
}
