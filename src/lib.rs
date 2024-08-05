#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use error::{Error, Result};
use models::marketplace::Marketplace;
use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder},
    Method,
};

pub mod error;
pub mod models;

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

        if !response.status().is_success() {
            return Err(Error::AccountCheck);
        }

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

    fn request_builder<S: AsRef<str>>(&self, method: Method, endpoint: S) -> RequestBuilder {
        self.client
            .request(method, format!("{BASE_URL}{}", endpoint.as_ref()))
            .bearer_auth(self.token.clone())
            .header("X-EBAY-C-MARKETPLACE-ID", self.marketplace.to_string())
    }
}
