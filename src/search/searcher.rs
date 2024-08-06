use crate::{error::Result, models::SearchResults, utils::Jsonify, EbayApiClient};
use reqwest::Method;

const ENDPOINT: &str = "buy/browse/v1/item_summary/search";

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
    pub fn search<S: AsRef<str>>(&self, query: S, limit: u16) -> Result<SearchResults> {
        let limit_as_str = limit.to_string();
        let query = [("q", query.as_ref()), ("limit", limit_as_str.as_str())];

        let builder = self.0.request_builder(Method::GET, ENDPOINT).query(&query);
        let response = builder.send()?;

        let response = response.error_for_status()?;
        let response = response.jsonify::<SearchResults>()?;

        Ok(response)
    }
}
