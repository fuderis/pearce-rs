use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Invalid HTTP status code `{0}`.")]
    InvalidStatusCode(u16),
}
