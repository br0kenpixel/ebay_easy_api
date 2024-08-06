use crate::{error::Result, models::search::SearchResults, utils::Jsonify, EbayApiClient};
use reqwest::Method;

const ENDPOINT: &str = "buy/browse/v1/item_summary/search";

pub struct Searcher<'c>(pub(crate) &'c EbayApiClient);

impl<'c> Searcher<'c> {
    pub fn search<S: AsRef<str>>(&self, query: S, limit: u16) -> Result<SearchResults> {
        let limit_as_str = limit.to_string();
        let query = ["q", query.as_ref(), "limit", limit_as_str.as_str()];

        let builder = self.0.request_builder(Method::GET, ENDPOINT).query(&query);
        let response = builder.send()?;

        let response = response.error_for_status()?;
        let response = response.jsonify::<SearchResults>()?;

        Ok(response)
    }
}
