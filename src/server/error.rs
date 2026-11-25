use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    Validation(validator::ValidationError),

    #[display(fmt = "Operation timeout.")]
    Timeout,
}
