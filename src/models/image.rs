use serde::Deserialize;

/// A product image URL.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    #[serde(rename = "imageUrl")]
    url: Box<str>,
}
