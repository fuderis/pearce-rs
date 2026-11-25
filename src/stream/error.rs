use crate::prelude::DynError;
use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum StreamError {
    #[display(fmt = "SSE invalid UTF-8 chunk: {0}")]
    Utf8(std::str::Utf8Error),

    #[display(fmt = "SSE JSON deserialize error: {0}")]
    Json(Box<serde_json::Error>),

    #[display(fmt = "SSE transport error: {0}")]
    Transport(DynError),
}
