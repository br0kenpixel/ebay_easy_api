use crate::ReadOnlyString;
use reqwest::{blocking::Response, StatusCode};
use serde::Deserialize;
use serde_json::Value;
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

    /// Failed to parse error object.
    #[error("Could not parse error object")]
    NoErr,

    /// API error.
    #[error("From API arror: {0}")]
    Api(#[from] ApiError),
}

/// An API error.
#[derive(Debug, Error, Deserialize)]
#[error("API Error: {message}")]
pub struct ApiError {
    /// Error ID.
    #[serde(rename = "errorId")]
    pub id: u32,

    /// Error domain.
    pub domain: ReadOnlyString,

    /// Error category.
    pub category: ReadOnlyString,

    /// Error message.
    pub message: ReadOnlyString,

    /// Loger error message.
    #[serde(rename = "longMessage")]
    pub long_message: ReadOnlyString,

    /// Status code.
    #[serde(skip)]
    pub(crate) status_code: StatusCode,
}

impl Error {
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::Api(error) => Some(error.status_code),
            _ => None,
        }
    }
}

impl TryFrom<Response> for ApiError {
    type Error = Error;

    fn try_from(value: Response) -> std::result::Result<Self, Self::Error> {
        let status = value.status();
        let mut error_obj: Value = serde_json::from_reader(value)?;

        let mut errors = error_obj
            .take()
            .get_mut("errors")
            .map(Value::take)
            .ok_or(Self::Error::NoErr)?;
        let first = errors.get_mut(0).map(Value::take);

        match first {
            None => Err(Self::Error::NoErr),
            Some(value) => {
                let mut error: ApiError = serde_json::from_value(value)?;

                error.status_code = status;

                Ok(error)
            }
        }
    }
}

pub(crate) fn handle_response_error(response: Response) -> Result<Response> {
    if !response.status().is_client_error() && !response.status().is_server_error() {
        return Ok(response);
    }

    let api_err: ApiError = response.try_into()?;
    Err(api_err.into())
}
