use macron::{Display, Error, From};

/// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    Io(std::io::Error),

    #[display(fmt = "Callback timeout.")]
    CallbackTimeout,

    #[display(fmt = "Callback channel is closed.")]
    CallbackClosed,
}
