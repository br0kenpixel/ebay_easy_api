use super::{
    category::Category, image::Image, marketplace::Marketplace, price::Price, seller::Seller,
    Location,
};
use chrono::Utc;
use serde::Deserialize;

/// Search results from [`Searcher::search()`](crate::search::Searcher::search).
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct SearchResults {
    href: Box<str>,
    total: usize,
    next: Box<str>,
    limit: usize,
    offset: usize,
    #[serde(rename = "itemSummaries")]
    items: Box<[SearchItem]>,
}

/// A product from a search API result.
///
/// **Currently, this does not contain all fields from Ebay's API.**
#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct SearchItem {
    #[serde(rename = "itemId")]
    id: Box<str>,

    title: Box<str>,
    // leafCategoryIds
    categories: Box<[Category]>,
    image: Image,
    price: Price,

    #[serde(rename = "itemHref")]
    item_link: Box<str>,

    seller: Seller,
    condition: Box<str>,
    // conditionId
    // thumbnailImages
    // shippingOptions
    // buyingOptions
    // epid
    #[serde(rename = "itemWebUrl")]
    web_link: Box<str>,

    #[serde(rename = "itemLocation")]
    location: Location,

    // additionalImages
    // adultOnly
    // legacyItemId
    // availableCoupons
    #[serde(rename = "itemCreationDate")]
    created_on: chrono::DateTime<Utc>,

    #[serde(rename = "topRatedBuyingExperience")]
    top_rated_buying_experience: bool,

    #[serde(rename = "priorityListing")]
    priority_listing: bool,

    #[serde(rename = "listingMarketplaceId")]
    listing_marketplace_id: Marketplace,
}
