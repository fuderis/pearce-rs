#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod prelude;

pub mod server;
pub use server::{Addr, Header, HeaderBody, Headers, Response, Server, Status};

pub use atoman::{self, Bytes, Stream, StreamReader, StreamSender};
pub use axum::{
    self,
    extract::{Json, Path as Paths, Query},
};
pub use urlencoding::{
    self, decode as url_decode, decode_binary as url_decode_binary, encode as url_encode,
    encode_binary as url_encode_binary,
};
pub use validator::{self, Validate, ValidationError};
