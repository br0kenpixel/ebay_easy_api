use derive_more::Display;
use serde::Deserialize;

#[derive(
    Debug, Clone, Deserialize, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
pub enum Marketplace {
    #[default]
    #[serde(rename = "EBAY_US")]
    #[display(fmt = "EBAY_US")]
    UnitedStates,
    #[serde(rename = "EBAY_DE")]
    #[display(fmt = "EBAY_DE")]
    Germany,
    #[serde(rename = "EBAY_IT")]
    #[display(fmt = "EBAY_IT")]
    Italy,
    #[serde(rename = "EBAY_IR")]
    #[display(fmt = "EBAY_IR")]
    Ireland,
    #[serde(rename = "EBAY_SG")]
    #[display(fmt = "EBAY_SG")]
    Singapore,
    #[serde(rename = "EBAY_UK")]
    #[display(fmt = "EBAY_UK")]
    UnitedKingdom,
    #[serde(rename = "EBAY_FR")]
    #[display(fmt = "EBAY_FR")]
    France,
}
