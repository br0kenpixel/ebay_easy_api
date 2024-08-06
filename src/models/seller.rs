use rust_decimal::Decimal;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Display;

/// An Ebay seller.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Seller {
    /// Username of the seller account.
    pub username: Box<str>,

    /// Feedback percentage as a fixed-precision decimal number.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "feedbackPercentage")]
    pub feedback_percentage: Decimal,

    /// Feedback score.
    #[serde(rename = "feedbackScore")]
    pub feedback_score: u32,
}

impl Display for Seller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
