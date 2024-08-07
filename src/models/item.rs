use super::{CategoryPath, Image, Location, Marketplace, Price, Seller};
use crate::ReadOnlyString;
use chrono::Utc;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// A detailed item.
/// This can be obtained using [`Searcher::find_item()`](crate::search::Searcher::find_item).
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    /// ID of the item.
    #[serde(rename = "itemId")]
    pub id: ReadOnlyString,

    /// Seller item revision.
    #[serde(rename = "sellerItemRevision")]
    #[serde_as(as = "DisplayFromStr")]
    pub revision: u8,

    /// Title.
    pub title: ReadOnlyString,

    /// Price.
    pub price: Price,

    /// Category paths.
    #[serde(rename = "categoryPath")]
    #[serde_as(as = "DisplayFromStr")]
    pub category_path: CategoryPath,

    // categoryIdPath
    /// Condition.
    pub condition: ReadOnlyString,

    /// Condition ID.
    #[serde(rename = "conditionId")]
    #[serde_as(as = "DisplayFromStr")]
    pub condition_id: u32,

    #[serde(rename = "itemLocation")]
    pub location: Location,

    /// Image URL.
    pub image: Image,

    // additionalImages
    /// Branding.
    pub brand: ReadOnlyString,

    /// Date and time of listing creation.
    #[serde(rename = "itemCreationDate")]
    pub created_on: chrono::DateTime<Utc>,

    /// Item seller.
    pub seller: Seller,

    // gtin
    // mpn
    // estimatedAvailabilities
    // shippingOptions
    // shipToLocations
    // returnTerms
    // taxes
    /// Top rated buying experience.
    #[serde(rename = "topRatedBuyingExperience")]
    pub top_rated_buying_experience: bool,

    // buyingOptions
    // itemAffiliateWebUrl
    /// Link to the product listing on Ebay's website.
    ///
    /// This is **not** an API URL.
    #[serde(rename = "itemWebUrl")]
    pub web_link: ReadOnlyString,

    /// Item description. *(HTML)*.
    pub description: ReadOnlyString,

    // paymentMethods
    /// Enabled for guest checkout.
    #[serde(rename = "enabledForGuestCheckout")]
    pub guest_checkout: bool,

    /// Eligible for inline checkout.
    #[serde(rename = "eligibleForInlineCheckout")]
    pub inline_checkout: bool,

    /// Lot size.
    #[serde(rename = "lotSize")]
    pub lot_size: u32,

    // legacyItemId
    /// Priority listing.
    #[serde(rename = "priorityListing")]
    pub priority_listing: bool,

    // adultOnly
    // categoryId
    /// Marketplace where listing was posted.
    /// This should match whatever marketplace [`EbayApiClient`](crate::EbayApiClient) is using (or used when this item was obtained).
    #[serde(rename = "listingMarketplaceId")]
    pub listing_marketplace_id: Marketplace,
}
