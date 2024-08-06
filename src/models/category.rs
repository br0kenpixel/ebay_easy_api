use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// A category.
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "categoryId")]
    id: u32,

    #[serde(rename = "categoryName")]
    name: Box<str>,
}
