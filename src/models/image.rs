use crate::ReadOnlyString;
use serde::Deserialize;
use std::ops::Deref;

/// A product image URL.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    #[serde(rename = "imageUrl")]
    url: ReadOnlyString,
}

impl Deref for Image {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}
