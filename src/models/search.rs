use super::{
    category::Category, image::Image, marketplace::Marketplace, price::Price, seller::Seller,
    Location,
};
use crate::ReadOnlyString;
use chrono::Utc;
use serde::Deserialize;

/// Search results from [`Searcher::search()`](crate::search::Searcher::search).
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct SearchResults {
    href: ReadOnlyString,
    total: usize,
    next: ReadOnlyString,
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
    /// Item ID.
    #[serde(rename = "itemId")]
    pub id: ReadOnlyString,

    /// Product name
    pub title: ReadOnlyString,
    // leafCategoryIds
    /// Product categories.
    pub categories: Box<[Category]>,
    /// Product image.
    pub image: Image,
    /// Product price.
    pub price: Price,

    /// API URl to fetch details of this product.
    #[serde(rename = "itemHref")]
    pub(crate) item_link: ReadOnlyString,

    /// Basic seller information.
    pub seller: Seller,
    /// Product condition.
    pub condition: ReadOnlyString,
    // conditionId
    // thumbnailImages
    // shippingOptions
    // buyingOptions
    // epid
    /// Direct link to the product on Ebay. *(non-API link)*
    #[serde(rename = "itemWebUrl")]
    pub web_link: ReadOnlyString,

    /// Origin of the product.
    #[serde(rename = "itemLocation")]
    pub location: Location,

    // additionalImages
    // adultOnly
    // legacyItemId
    // availableCoupons
    /// Date and time of listing creation.
    #[serde(rename = "itemCreationDate")]
    pub created_on: chrono::DateTime<Utc>,

    /// Top rated buying experience.
    #[serde(rename = "topRatedBuyingExperience")]
    pub top_rated_buying_experience: bool,

    /// Priority listing.
    #[serde(rename = "priorityListing")]
    pub priority_listing: bool,

    /// Marketplace where listing was posted.
    /// This should match whatever marketplace [`EbayApiClient`](crate::EbayApiClient) is using (or used when this item was obtained).
    #[serde(rename = "listingMarketplaceId")]
    pub listing_marketplace_id: Marketplace,
}
