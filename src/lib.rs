#![allow(clippy::module_name_repetitions)]

use error::Result;
use models::marketplace::Marketplace;
use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder},
    Method,
};
use search::Searcher;

/// Error types.
pub mod error;
/// Models.
pub mod models;
/// Search API client(s).
pub mod search;
pub(crate) mod utils;

const BASE_URL: &str = "https://api.ebay.com/";

/// A synchronous (blocking) API client.
pub struct EbayApiClient {
    client: Client,
    token: Box<str>,
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
    pub fn new<S: AsRef<str>>(token: S, marketplace: Marketplace) -> Result<Self> {
        let client = Self::new_unchecked(token, marketplace);

        let response = client
            .request_builder(Method::GET, "sell/account/v1/subscription")
            .send()?;

        response.error_for_status()?;

        Ok(client)
    }

    /// Creates a new client instance using the given OAuth token and marketplace.
    ///
    /// This method will **not** check if the provided token is valid.
    /// If want to verify the token, use [`new`](Self::new) instead.
    ///
    /// # Panics
    /// This function calls `unwrap()` internally when creating a [`ClientBuilder`](ClientBuilder). This however, should never fail.
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
    pub const fn search(&self) -> Searcher {
        Searcher(self)
    }

    fn request_builder<S: AsRef<str>>(&self, method: Method, endpoint: S) -> RequestBuilder {
        self.client
            .request(method, format!("{BASE_URL}{}", endpoint.as_ref()))
            .header("X-EBAY-C-MARKETPLACE-ID", self.marketplace.as_str())
            .bearer_auth(&self.token)
    }
}
