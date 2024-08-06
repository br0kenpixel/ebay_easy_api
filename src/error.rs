use thiserror::Error;

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

/// A library error.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to perform an HTTP request.
    #[error("Request: {0}")]
    Request(#[from] reqwest::Error),

    /// Token validation check failed. This is only returned by [`EbayApiClient::new()`](crate::EbayApiClient).
    #[error("Invalid token")]
    AccountCheck,

    /// Failed to de/serialize a JSON object.
    #[error("De/Serialization: {0}")]
    Serde(#[from] serde_json::Error),
}
