use macron::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum ServerError {
    Validation(validator::ValidationError),

    #[display(fmt = "Operation timeout.")]
    Timeout,
}
