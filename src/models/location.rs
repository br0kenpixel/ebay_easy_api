use serde::Deserialize;

/// A location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Location {
    /// Postal code.
    #[serde(rename = "postalCode")]
    pub post_code: Box<str>,

    /// Country code.
    pub country: Box<str>,
}
