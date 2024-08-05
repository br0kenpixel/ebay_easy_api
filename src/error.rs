use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Invalid token")]
    AccountCheck,
}
