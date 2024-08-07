use super::{
    category::Category, image::Image, marketplace::Marketplace, price::Price, seller::Seller,
    Location,
};
use crate::{ReadOnlyString, ReadOnlyVec};
use chrono::Utc;
use serde::Deserialize;

/// Search results from [`Browser::search()`](crate::search::Browser::search).
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchResults {
    /// The total number of search results.
    pub total: usize,

    /// API URL to get the next "page" of results. This may be [`None`] if there is no next page.
    pub(crate) next: Option<ReadOnlyString>,

    /// Search results.
    #[serde(rename = "itemSummaries")]
    #[serde(default)]
    pub items: ReadOnlyVec<SearchItem>,
}

/// A product from a search API result.
///
/// **Currently, this does not contain all fields from Ebay's API.**
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchItem {
    /// Item ID.
    #[serde(rename = "itemId")]
    pub id: ReadOnlyString,

    /// Product name
    pub title: ReadOnlyString,
    // leafCategoryIds
    /// Product categories.
    pub categories: ReadOnlyVec<Category>,
    /// Product image.
    pub image: Image,
    /// Product price.
    pub price: Price,

    /// API URL to fetch details of this product.
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
    /// Link to the product listing on Ebay's website.
    ///
    /// This is **not** an API URL.
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

impl SearchResults {
    /// Return the amount of search results.
    ///
    /// Shorthand to `items.len()`.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.items.len()
    }

    /// Return whether the result set is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over the search results.
    pub fn iter(&self) -> core::slice::Iter<SearchItem> {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a SearchResults {
    type IntoIter = std::slice::Iter<'a, SearchItem>;
    type Item = &'a SearchItem;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
