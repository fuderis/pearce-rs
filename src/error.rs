use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    Io(std::io::Error),

    Json(Box<serde_json::Error>),

    Http(crate::http::HttpError),

    #[cfg(feature = "server")]
    Server(crate::server::ServerError),

    #[cfg(feature = "client")]
    Client(crate::client::ClientError),

    #[cfg(feature = "stream")]
    Stream(crate::stream::StreamError),
}
