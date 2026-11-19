use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Invalid URL string `{0}`.")]
    InvalidUrl(String),

    Reqwest(reqwest::Error),

    #[display(fmt = "Callback channel is closed.")]
    CallbackClosed,
}
