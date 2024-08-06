use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    #[serde(rename = "imageUrl")]
    url: Box<str>,
}
