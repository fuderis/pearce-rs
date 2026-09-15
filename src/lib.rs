#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod error;
pub mod prelude;

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use server::{
    Addr, Callback, CallbackReceiver, Header, HeaderBody, Headers, Json, Paths, Query, Response,
    Server, Status, url,
};

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::{Client, StreamExt};

#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use stream::{Bytes, BytesMut, Receiver, Sender};
