use crate::ReadOnlyString;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// A category.
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category {
    /// Numeric category ID.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "categoryId")]
    pub id: u32,

    /// Name of the category.
    #[serde(rename = "categoryName")]
    pub name: ReadOnlyString,
}
