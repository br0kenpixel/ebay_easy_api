use serde::Deserialize;
use super::category::Category;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchResults {
    href: Box<str>,
    total: usize,
    next: Box<str>,
    limit: usize,
    offset: usize,
    #[serde(rename = "itemSummaries")]
    items: Box<[SearchItem]>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchItem {
    #[serde(rename = "itemId")]
    id: Box<str>,
    title: Box<str>,
    // leafCategoryIds
    categories: Box<[Category]>,
    
}
