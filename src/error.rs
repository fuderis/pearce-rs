use macron::{Display, Error, From};

/// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    Io(std::io::Error),

    #[cfg(feature = "stream")]
    #[display = "Unexpected EOF with partial data"]
    UnexpectedEOF,
}
