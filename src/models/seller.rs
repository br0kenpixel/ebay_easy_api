use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Seller {
    username: Box<str>,
    #[serde(rename = "feedbackPercentage")]
    feedback_percentage: f32,
    #[serde(rename = "feedbackScore")]
    feedback_score: u32,
}
