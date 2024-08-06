#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

use error::Result;
use models::marketplace::Marketplace;
use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder},
    Method,
};
use search::Searcher;

pub mod error;
pub mod models;
pub mod search;
pub(crate) mod utils;

const BASE_URL: &str = "https://api.ebay.com/";

pub struct EbayApiClient {
    client: Client,
    token: Box<str>,
    marketplace: Marketplace,
}

impl EbayApiClient {
    pub fn new<S: AsRef<str>>(token: S, marketplace: Marketplace) -> Result<Self> {
        let client = Self::new_unchecked(token, marketplace);

        let response = client
            .request_builder(Method::GET, "sell/account/v1/subscription")
            .send()?;

        response.error_for_status()?;

        Ok(client)
    }

    pub fn new_unchecked<S: AsRef<str>>(token: S, marketplace: Marketplace) -> Self {
        let token = token.as_ref();
        let client = ClientBuilder::new().https_only(true).build().unwrap();

        Self {
            client,
            token: token.into(),
            marketplace,
        }
    }

    #[must_use]
    pub const fn marketplace(&self) -> Marketplace {
        self.marketplace
    }

    pub fn set_marketplace(&mut self, new: Marketplace) {
        self.marketplace = new;
    }

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
