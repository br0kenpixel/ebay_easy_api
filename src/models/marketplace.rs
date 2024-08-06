use serde::Deserialize;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// An Ebay marketplace ID.
#[derive(Debug, Clone, Deserialize, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Marketplace {
    #[default]
    #[serde(rename = "EBAY_US")]
    UnitedStates,
    #[serde(rename = "EBAY_DE")]
    Germany,
    #[serde(rename = "EBAY_IT")]
    Italy,
    #[serde(rename = "EBAY_IR")]
    Ireland,
    #[serde(rename = "EBAY_SG")]
    Singapore,
    #[serde(rename = "EBAY_UK")]
    UnitedKingdom,
    #[serde(rename = "EBAY_FR")]
    France,
}

/// An error representing a failed conversion from a string to a marketplace ID.
#[derive(Debug, Error)]
#[error("Invalid or unknown marketplace: '{0}'")]
pub struct InvalidMarketplace(Box<str>);

impl Marketplace {
    /// Returns the marketplace ID as a static string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitedStates => "EBAY_US",
            Self::Germany => "EBAY_DE",
            Self::Italy => "EBAY_IT",
            Self::Ireland => "EBAY_IR",
            Self::Singapore => "EBAY_SG",
            Self::UnitedKingdom => "EBAY_UK",
            Self::France => "EBAY_FR",
        }
    }
}

impl FromStr for Marketplace {
    type Err = InvalidMarketplace;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const VARIANTS: [Marketplace; 7] = [
            Marketplace::UnitedStates,
            Marketplace::Germany,
            Marketplace::Italy,
            Marketplace::Ireland,
            Marketplace::Singapore,
            Marketplace::UnitedKingdom,
            Marketplace::France,
        ];

        for variant in VARIANTS {
            if s == variant.as_str() {
                return Ok(variant);
            }
        }

        Err(InvalidMarketplace(s.into()))
    }
}

impl Display for Marketplace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
