use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// An Ebay seller.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Seller {
    username: Box<str>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "feedbackPercentage")]
    feedback_percentage: f32,

    #[serde(rename = "feedbackScore")]
    feedback_score: u32,
}
