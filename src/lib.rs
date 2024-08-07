#![allow(clippy::module_name_repetitions)]

use error::{handle_response_error, Result};
use models::Marketplace;
use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder},
    Method,
};
use search::Browser;

/// Error types.
pub mod error;
/// Models.
pub mod models;
/// Search API client(s).
pub mod search;
pub(crate) mod utils;

/// A heap-allocated immutable string.
pub type ReadOnlyString = Box<str>;
/// A heap-allocated immutable array.
pub type ReadOnlyVec<T> = Box<[T]>;

const BASE_URL: &str = "https://api.ebay.com/";

/// A synchronous (blocking) API client.
pub struct EbayApiClient {
    client: Client,
    token: ReadOnlyString,
    marketplace: Marketplace,
}

impl EbayApiClient {
    /// Creates a new client instance using the given OAuth token and marketplace.
    ///
    /// This method will perform a test request, to make sure that the provided token is valid.
    /// If you don't want to perform this check, use [`new_unchecked`](Self::new_unchecked) instead.
    ///
    /// # Errors
    /// An error is returned if the token verification fails.
    ///
    /// # Panics
    /// This function calls [`new_unchecked`](Self::new_unchecked) internally, please refer to it's panic documentation.
    ///
    /// # Example
    /// ```rust
    /// use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// use std::env::var;
    ///
    /// // Define our token
    /// let token = var("EBAY_TOKEN").unwrap();
    ///
    /// // Define our marketplace
    /// let marketplace = Marketplace::UnitedStates;
    ///
    /// // Build our client
    /// let client = EbayApiClient::new(token, marketplace);
    ///
    /// // If the token is valid, it should return a client instance wrapped in `Result`.
    /// assert!(client.is_ok());
    /// ```
    pub fn new<S: AsRef<str>>(token: S, marketplace: Marketplace) -> Result<Self> {
        let client = Self::new_unchecked(token, marketplace);

        let response = client
            .request_builder(Method::GET, "sell/account/v1/subscription")
            .send()?;

        handle_response_error(response)?;

        Ok(client)
    }

    #[allow(rustdoc::redundant_explicit_links)]
    /// Creates a new client instance using the given OAuth token and marketplace.
    ///
    /// This method will **not** check if the provided token is valid.
    /// If want to verify the token, use [`new`](Self::new) instead.
    ///
    /// Note that if the token is invalid, all operations with this client will fail.
    ///
    /// # Panics
    /// This function calls `unwrap()` internally when creating a [`ClientBuilder`](reqwest::blocking::ClientBuilder).
    /// This however, should never fail.
    ///
    /// # Example
    /// ```rust
    /// use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// use std::env::var;
    ///
    /// // Define our token
    /// let token = var("EBAY_TOKEN").unwrap();
    ///
    /// // Define our marketplace
    /// let marketplace = Marketplace::UnitedStates;
    ///
    /// // Build our client
    /// let client = EbayApiClient::new_unchecked(token, marketplace);
    /// ```
    pub fn new_unchecked<S: AsRef<str>>(token: S, marketplace: Marketplace) -> Self {
        let token = token.as_ref();
        let client = ClientBuilder::new().https_only(true).build().unwrap();

        Self {
            client,
            token: token.into(),
            marketplace,
        }
    }

    /// Returns the marketplace this client is using.
    #[must_use]
    pub const fn marketplace(&self) -> Marketplace {
        self.marketplace
    }

    /// Sets the marketplace this client is using.
    pub fn set_marketplace(&mut self, new: Marketplace) {
        self.marketplace = new;
    }

    /// Returns a Search API client. Refer to the [search api](crate::search) documentation.
    #[must_use]
    pub const fn search(&self) -> Browser {
        Browser(self)
    }

    fn request_builder<S: AsRef<str>>(&self, method: Method, endpoint: S) -> RequestBuilder {
        self.client
            .request(method, format!("{BASE_URL}{}", endpoint.as_ref()))
            .header("X-EBAY-C-MARKETPLACE-ID", self.marketplace.as_str())
            .bearer_auth(&self.token)
    }
}
