use super::{Header, HeaderBody, Status};
use crate::prelude::*;

use axum::{
    body::Body,
    http::{HeaderValue, response::Builder},
    response::{IntoResponse, Response as AxumResponse},
};
use futures::Stream as FuturesStream;

/// The HTTP response builder
pub struct Response {
    builder: Builder,
    body: Body,
}

impl Response {
    /// Creates a new response builder
    pub fn new(status: impl Into<Status>) -> Self {
        Self {
            builder: Builder::new().status(status.into().0),
            body: Body::empty(),
        }
    }

    // --- HTTP STATUS ---

    /// Changes the status code (for example, 201, 404, 500)
    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.builder = self.builder.status(status.into().0);
        self
    }

    /// Creates a new response with 200 code
    pub fn ok() -> Self {
        Self::new(200)
    }

    /// Creates a new response with 201 code
    pub fn created() -> Self {
        Self::new(201)
    }

    /// Creates a new response with 204 code
    pub fn no_content() -> Self {
        Self::new(204)
    }

    /// Creates a new response with 301 code
    pub fn redirect() -> Self {
        Self::new(301)
    }

    /// Creates a new response with 302 code
    pub fn temp_redirect() -> Self {
        Self::new(302)
    }

    /// Creates a new response with 304 code
    pub fn not_modified() -> Self {
        Self::new(304)
    }

    /// Creates a new response with 400 code
    pub fn bad_request() -> Self {
        Self::new(400)
    }

    /// Creates a new response with 401 code
    pub fn unauthorized() -> Self {
        Self::new(401)
    }

    /// Creates a new response with 403 code
    pub fn forbidden() -> Self {
        Self::new(403)
    }

    /// Creates a new response with 404 code
    pub fn not_found() -> Self {
        Self::new(404)
    }

    /// Creates a new response with 405 code
    pub fn bad_method() -> Self {
        Self::new(405)
    }

    /// Creates a new response with 409 code
    pub fn conflict() -> Self {
        Self::new(409)
    }

    /// Creates a new response with 413 code
    pub fn too_large() -> Self {
        Self::new(413)
    }

    /// Creates a new response with 422 code
    pub fn bad_entity() -> Self {
        Self::new(422)
    }

    /// Creates a new response with 429 code
    pub fn rate_limited() -> Self {
        Self::new(429)
    }

    /// Creates a new response with 500 code
    pub fn server_error() -> Self {
        Self::new(500)
    }

    /// Creates a new response with 502 code
    pub fn bad_gateway() -> Self {
        Self::new(502)
    }

    /// Creates a new response with 503 code
    pub fn unavailable() -> Self {
        Self::new(503)
    }

    /// Creates a new response with 504 code
    pub fn timeout() -> Self {
        Self::new(504)
    }

    // --- HTTP HEADER ---

    /// Sets the header
    pub fn header<'a>(
        mut self,
        header: impl Into<Header>,
        value: impl Into<HeaderBody<'a>>,
    ) -> Result<Self> {
        self.builder = self
            .builder
            .header(header.into().0, HeaderValue::from_str(value.into().0)?);
        Ok(self)
    }

    /// Sets the HTTPS-only connect header
    pub fn https_only(self, seconds: u64) -> Self {
        self.header(
            Header::StrictTransportSecurity,
            str!("max-age={}; includeSubDomains", seconds).as_str(),
        )
        .unwrap()
    }

    /// Sets the iframe options header
    pub fn no_iframe(self) -> Self {
        self.header(Header::XFrameOptions, "DENY").unwrap()
    }

    /// Sets the content sniff options header
    pub fn no_sniff(self) -> Self {
        self.header(Header::XContentTypeOptions, "nosniff").unwrap()
    }

    /// Sets the cache control options header
    pub fn cache_control<'a>(self, value: impl Into<HeaderBody<'a>>) -> Self {
        self.header(Header::CacheControl, value).unwrap()
    }

    /// Sets the cache control options header
    pub fn no_cache(self) -> Self {
        self.cache_control("no-store, no-cache, must-revalidate")
    }

    /// Sets the content-type header
    pub fn content_type<'a>(self, value: impl Into<HeaderBody<'a>>) -> Self {
        self.header(Header::ContentType, value).unwrap()
    }

    /// Sets the content-type=TEXT header
    pub fn content_type_text(self) -> Self {
        self.content_type("text/plain; charset=utf-8")
    }

    /// Sets the content-type=JSON header
    pub fn content_type_json(self) -> Self {
        self.content_type("application/json")
    }

    /// Sets the content-type=HTML header
    pub fn content_type_html(self) -> Self {
        self.content_type("text/html; charset=utf-8")
    }

    /// Sets the content-type=EVENT-STREAM header
    pub fn content_type_stream(self) -> Self {
        self.header(Header::ContentType, "text/event-stream")
            .unwrap()
            .header(Header::CacheControl, "no-cache")
            .unwrap()
            .header(Header::Connection, "keep-alive")
            .unwrap()
            .header(Header::XContentTypeOptions, "nosniff")
            .unwrap()
    }

    /// Sets the access control header
    pub fn allow_origin(self) -> Self {
        self.header(Header::AccessControlAllowOrigin, "").unwrap()
    }

    /// Sets the access control header
    pub fn allow_methods(self) -> Self {
        self.header(Header::AccessControlAllowMethods, "").unwrap()
    }

    /// Sets the access control header
    pub fn allow_headers(self) -> Self {
        self.header(Header::AccessControlAllowHeaders, "").unwrap()
    }

    /// Sets the file attachment header
    pub fn attachment(self, filename: &str) -> Result<Self> {
        let value = if filename.is_ascii() {
            filename
        } else {
            &urlencoding::encode(filename)
        };

        self.header(
            Header::ContentDisposition,
            str!("attachment; filename*=UTF-8''{value}").as_str(),
        )
    }

    /// Sets the redirect location header
    pub fn location(self, uri: &str) -> Result<Self> {
        let value = if uri.is_ascii() {
            uri
        } else {
            &urlencoding::encode(uri)
        };

        self.header(Header::Location, value)
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
                self = self.status(500).content_type_json();
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
        match self.builder.body(self.body) {
            Ok(r) => r,
            Err(e) => AxumResponse::builder()
                .status(500)
                .body(Body::from(str!("{e}")))
                .unwrap(),
        }
    }
}
