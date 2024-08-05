use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category {
    #[serde(rename = "categoryId")]
    id: Box<str>,
    #[serde(rename = "categoryName")]
    name: Box<str>,
}
