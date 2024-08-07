use crate::{
    error::Result,
    models::{Item, SearchResults},
    utils::Jsonify,
    EbayApiClient,
};
use reqwest::{Method, StatusCode};

const SEARCH_ENDPOINT: &str = "buy/browse/v1/item_summary/search";
const ITEM_SEARCH_ENDPOINT: &str = "buy/browse/v1/item/";

/// A Search API client.
///
/// This structure references an instance of [`EbayApiClient`](EbayApiClient), so
/// it can only be used as long as the referenced [`EbayApiClient`](EbayApiClient) lives.
pub struct Searcher<'c>(pub(crate) &'c EbayApiClient);

impl<'c> Searcher<'c> {
    /// Perform a search using the given query string and return up to `limit` results.
    ///
    /// # Errors
    /// This can return an error if the request fails, or the response body could not be deserialized.
    ///
    /// # Example
    /// ```rust
    /// # use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// # use std::env::var;
    /// # let token = var("EBAY_TOKEN").unwrap();
    /// # let marketplace = Marketplace::UnitedStates;
    /// let client = EbayApiClient::new_unchecked(token, marketplace);
    ///
    /// // Obtain a search API client
    /// let searcher = client.search();
    ///
    /// // Define a limit
    /// let limit: usize = 3;
    ///
    /// // Perform the search query
    /// let results = searcher.search("gaming pc", limit);
    ///
    /// // If the search was successfull, we should get the results.
    /// assert!(results.is_ok());
    ///
    /// // We should not get more than `limit` results.
    /// assert!(results.unwrap().len() <= limit);
    /// ```
    pub fn search<S: AsRef<str>>(&self, query: S, limit: usize) -> Result<SearchResults> {
        let limit_as_str = limit.to_string();
        let query = [("q", query.as_ref()), ("limit", limit_as_str.as_str())];

        let builder = self
            .0
            .request_builder(Method::GET, SEARCH_ENDPOINT)
            .query(&query);
        let response = builder.send()?;

        let response = response.error_for_status()?;
        let response = response.jsonify::<SearchResults>()?;

        Ok(response)
    }

    /// Finds an item by it's ID.
    ///
    /// # Errors
    /// This can return an error if the request fails or the response body could not be deserialized.
    /// Furthermore, if the item was not found [`None`](None) is returned.
    ///
    /// # Example
    /// ```rust
    /// # use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// # use std::env::var;
    /// # let token = var("EBAY_TOKEN").unwrap();
    /// # let marketplace = Marketplace::UnitedStates;
    /// let client = EbayApiClient::new_unchecked(token, marketplace);
    ///
    /// // Obtain a search API client
    /// let searcher = client.search();
    ///
    /// // Perform the search query
    /// let result = searcher.find_item("v1|202975928242|0");
    ///
    /// // If the search was successfull, we should get the results.
    /// assert!(result.is_ok());
    ///
    /// match result.unwrap() {
    ///     Some(item) => println!("Found {}, with price {}", item.title, item.price),
    ///     None => println!("Item was not found"),
    /// }
    /// ```
    pub fn find_item<S: AsRef<str>>(&self, id: S) -> Result<Option<Item>> {
        let endpoint = format!("{ITEM_SEARCH_ENDPOINT}{}", id.as_ref());
        let builder = self.0.request_builder(Method::GET, endpoint);

        let response = builder.send()?;

        match response.error_for_status() {
            Ok(response) => Ok(Some(response.jsonify()?)),
            Err(why) => {
                if why.status() == Some(StatusCode::NOT_FOUND) {
                    return Ok(None);
                }

                Err(why.into())
            }
        }
    }
}
