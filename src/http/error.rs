use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum HttpError {
    #[display(fmt = "Invalid HTTP status code `{0}`.")]
    InvalidStatusCode(u16),
}
