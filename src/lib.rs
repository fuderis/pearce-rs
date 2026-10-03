#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(all(
    feature = "client",
    not(any(feature = "rustls", feature = "native-tls"))
))]
compile_error!(
    "Feature 'client' requires at least one TLS backend to be enabled. \
    Please enable 'rustls' or 'native-tls'."
);

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features 'rustls' and 'native-tls' cannot be enabled at the same time.");

pub mod error;
pub mod prelude;

pub mod http;
#[cfg(feature = "server")]
pub use http::Headers;
pub use http::{Header, HeaderBody, IntoHeader, Method, Socket, Status};

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use server::{Callback, CallbackReceiver, Json, Paths, Query, Response, Server, axum, url};

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::{Client, Proxy, Request, reqwest};

#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use stream::{
    Bytes, BytesMut, Receiver, Sender, Stream, StreamExt, stream_body, stream_reader,
};
