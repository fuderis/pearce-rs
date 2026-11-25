#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(all(
    feature = "client",
    not(any(feature = "rustls", feature = "native-tls"))
))]
compile_error!(
    "Feature `client` requires at least one TLS backend to be enabled. \
    Please enable `rustls` or `native-tls`."
);

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features `rustls` and `native-tls` cannot be enabled at the same time.");

mod prelude;

pub mod error;
pub use error::Error;

pub mod http;
pub use http::*;

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use server::*;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use stream::*;
